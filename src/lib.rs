pub mod codeclimate;
pub mod compiler;
pub mod github;
pub mod sarif;

use std::io::{BufRead, BufReader, Read};
use compiler::{CompilerArtifact, Diagnostic};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    CodeClimate,
    Sarif,
    GitHub,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "codeclimate" | "code_climate" | "cc" => Ok(OutputFormat::CodeClimate),
            "sarif" => Ok(OutputFormat::Sarif),
            "github" | "gh" | "actions" => Ok(OutputFormat::GitHub),
            other => Err(format!("Unknown format: {}. Valid options: codeclimate, sarif, github", other)),
        }
    }
}

pub struct ReporterConfig {
    pub format: OutputFormat,
    pub prefix_trim: Option<String>,
    pub ignore_wasm_bindgen: bool,
    pub min_level: Option<String>,
}

pub fn parse_compiler_messages<R: Read>(reader: R) -> Vec<Diagnostic> {
    let buf_reader = BufReader::new(reader);
    let mut diagnostics = Vec::new();

    for line in buf_reader.lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Attempt single artifact line
        if let Ok(artifact) = serde_json::from_str::<CompilerArtifact>(trimmed) {
            if artifact.reason == "compiler-message" {
                if let Some(diag) = artifact.message {
                    diagnostics.push(diag);
                }
            }
        } else if let Ok(diag) = serde_json::from_str::<Diagnostic>(trimmed) {
            diagnostics.push(diag);
        }
    }

    diagnostics
}

pub fn filter_diagnostics(
    diagnostics: Vec<Diagnostic>,
    config: &ReporterConfig,
) -> Vec<Diagnostic> {
    diagnostics
        .into_iter()
        .filter(|diag| {
            // Min level filtering
            if let Some(ref min) = config.min_level {
                if min == "error" && diag.level != "error" {
                    return false;
                }
                if min == "warning" && diag.level != "error" && diag.level != "warning" {
                    return false;
                }
            }

            // Cloudflare worker-rs / wasm-bindgen filtering
            if config.ignore_wasm_bindgen {
                if let Some(span) = diag.primary_span() {
                    if span.file_name.contains(".cargo") || span.file_name.contains("wasm_bindgen") {
                        return false;
                    }
                }
                if diag.message.contains("wasm-bindgen") {
                    return false;
                }
            }

            true
        })
        .collect()
}

pub fn generate_report(diagnostics: &[Diagnostic], config: &ReporterConfig) -> String {
    let prefix = config.prefix_trim.as_deref();
    match config.format {
        OutputFormat::CodeClimate => {
            let issues: Vec<_> = diagnostics
                .iter()
                .filter_map(|d| codeclimate::CodeClimateIssue::from_diagnostic(d, prefix))
                .collect();
            serde_json::to_string_pretty(&issues).unwrap_or_else(|_| "[]".to_string())
        }
        OutputFormat::Sarif => {
            let report = sarif::SarifReport::from_diagnostics(diagnostics, prefix);
            serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
        }
        OutputFormat::GitHub => github::GitHubReporter::format_all(diagnostics, prefix),
    }
}
