use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerArtifact {
    pub reason: String,
    #[serde(default)]
    pub message: Option<Diagnostic>,
    #[serde(default)]
    pub target: Option<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub kind: Vec<String>,
    pub src_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: String,
    #[serde(default)]
    pub spans: Vec<DiagnosticSpan>,
    #[serde(default)]
    pub children: Vec<Diagnostic>,
    #[serde(default)]
    pub rendered: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCode {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    pub text: Vec<DiagnosticSpanText>,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSpanText {
    pub text: String,
    pub highlight_start: usize,
    pub highlight_end: usize,
}

impl Diagnostic {
    pub fn primary_span(&self) -> Option<&DiagnosticSpan> {
        self.spans.iter().find(|s| s.is_primary).or_else(|| self.spans.first())
    }

    pub fn rule_name(&self) -> String {
        self.code
            .as_ref()
            .map(|c| c.code.clone())
            .unwrap_or_else(|| format!("rustc::{}", self.level))
    }
}
