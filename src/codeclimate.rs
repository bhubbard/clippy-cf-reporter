use crate::compiler::Diagnostic;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeClimateIssue {
    #[serde(rename = "type")]
    pub issue_type: String,
    pub check_name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<IssueContent>,
    pub categories: Vec<String>,
    pub location: IssueLocation,
    pub severity: String,
    pub fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_points: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueContent {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueLocation {
    pub path: String,
    pub lines: IssueLines,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<IssuePositions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssueLines {
    pub begin: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuePositions {
    pub begin: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl CodeClimateIssue {
    pub fn from_diagnostic(diag: &Diagnostic, prefix_trim: Option<&str>) -> Option<Self> {
        let span = diag.primary_span()?;
        let mut path = span.file_name.clone();
        if let Some(prefix) = prefix_trim {
            if path.starts_with(prefix) {
                path = path[prefix.len()..].trim_start_matches('/').to_string();
            }
        }

        let check_name = diag.rule_name();
        let description = diag.message.clone();
        let severity = match diag.level.as_str() {
            "error" => "critical",
            "warning" => "major",
            "note" => "minor",
            "help" => "info",
            _ => "minor",
        }
        .to_string();

        let category = if check_name.contains("perf") || check_name.contains("isolate") {
            "Performance"
        } else if check_name.contains("security") || check_name.contains("leak") {
            "Security"
        } else if check_name.contains("complexity") || check_name.contains("cognitive") {
            "Complexity"
        } else if check_name.contains("style") || check_name.contains("pedantic") {
            "Style"
        } else if diag.level == "error" || check_name.contains("correctness") {
            "Bug Risk"
        } else {
            "Clarity"
        };

        // Compute deterministic fingerprint
        let mut hasher = Sha256::new();
        hasher.update(path.as_bytes());
        hasher.update(b":");
        hasher.update(check_name.as_bytes());
        hasher.update(b":");
        hasher.update(description.as_bytes());
        let fingerprint = hex::encode(hasher.finalize());

        let content = diag.code.as_ref().and_then(|c| c.explanation.as_ref()).map(|exp| {
            IssueContent {
                body: exp.clone(),
            }
        });

        Some(Self {
            issue_type: "issue".to_string(),
            check_name,
            description,
            content,
            categories: vec![category.to_string()],
            location: IssueLocation {
                path,
                lines: IssueLines {
                    begin: span.line_start,
                    end: span.line_end.max(span.line_start),
                },
                positions: Some(IssuePositions {
                    begin: Position {
                        line: span.line_start,
                        column: span.column_start,
                    },
                    end: Position {
                        line: span.line_end,
                        column: span.column_end,
                    },
                }),
            },
            severity,
            fingerprint,
            remediation_points: Some(50_000),
        })
    }
}
