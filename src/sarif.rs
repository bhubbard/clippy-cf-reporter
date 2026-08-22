use crate::compiler::Diagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifReport {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    #[serde(rename = "informationUri")]
    pub information_uri: String,
    pub rules: Vec<SarifRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifRule {
    pub id: String,
    #[serde(rename = "shortDescription")]
    pub short_description: SarifMessage,
    #[serde(rename = "fullDescription", skip_serializing_if = "Option::is_none")]
    pub full_description: Option<SarifMessage>,
    #[serde(rename = "defaultConfiguration")]
    pub default_configuration: SarifRuleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifRuleConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SarifMessage {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    pub region: SarifRegion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRegion {
    #[serde(rename = "startLine")]
    pub start_line: usize,
    #[serde(rename = "startColumn")]
    pub start_column: usize,
    #[serde(rename = "endLine")]
    pub end_line: usize,
    #[serde(rename = "endColumn")]
    pub end_column: usize,
}

impl SarifReport {
    pub fn from_diagnostics(diagnostics: &[Diagnostic], prefix_trim: Option<&str>) -> Self {
        let mut results = Vec::new();
        let mut rules = Vec::new();
        let mut seen_rules = std::collections::HashSet::new();

        for diag in diagnostics {
            if let Some(span) = diag.primary_span() {
                let mut path = span.file_name.clone();
                if let Some(prefix) = prefix_trim {
                    if path.starts_with(prefix) {
                        path = path[prefix.len()..].trim_start_matches('/').to_string();
                    }
                }

                let rule_id = diag.rule_name();
                let sarif_level = match diag.level.as_str() {
                    "error" => "error",
                    "warning" => "warning",
                    "note" => "note",
                    "help" => "none",
                    _ => "warning",
                }
                .to_string();

                if !seen_rules.contains(&rule_id) {
                    seen_rules.insert(rule_id.clone());
                    rules.push(SarifRule {
                        id: rule_id.clone(),
                        short_description: SarifMessage {
                            text: rule_id.clone(),
                        },
                        full_description: diag.code.as_ref().and_then(|c| c.explanation.as_ref()).map(|exp| {
                            SarifMessage {
                                text: exp.clone(),
                            }
                        }),
                        default_configuration: SarifRuleConfig {
                            level: sarif_level.clone(),
                        },
                    });
                }

                results.push(SarifResult {
                    rule_id,
                    level: sarif_level,
                    message: SarifMessage {
                        text: diag.message.clone(),
                    },
                    locations: vec![SarifLocation {
                        physical_location: SarifPhysicalLocation {
                            artifact_location: SarifArtifactLocation { uri: path },
                            region: SarifRegion {
                                start_line: span.line_start,
                                start_column: span.column_start,
                                end_line: span.line_end.max(span.line_start),
                                end_column: span.column_end.max(span.column_start),
                            },
                        },
                    }],
                });
            }
        }

        Self {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
            version: "2.1.0".to_string(),
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "clippy-cf-reporter".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string(),
                        information_uri: "https://github.com/bhubbard/clippy-cf-reporter".to_string(),
                        rules,
                    },
                },
                results,
            }],
        }
    }
}
