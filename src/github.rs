use crate::compiler::Diagnostic;

pub struct GitHubReporter;

impl GitHubReporter {
    pub fn format_annotation(diag: &Diagnostic, prefix_trim: Option<&str>) -> Option<String> {
        let span = diag.primary_span()?;
        let mut path = span.file_name.clone();
        if let Some(prefix) = prefix_trim {
            if path.starts_with(prefix) {
                path = path[prefix.len()..].trim_start_matches('/').to_string();
            }
        }

        let cmd = match diag.level.as_str() {
            "error" => "error",
            "warning" => "warning",
            _ => "notice",
        };

        let title = diag.rule_name();
        let message = diag.message.replace('\r', "").replace('\n', "%0A");

        Some(format!(
            "::{} file={},line={},col={},endLine={},endColumn={},title={}::{}",
            cmd,
            path,
            span.line_start,
            span.column_start,
            span.line_end.max(span.line_start),
            span.column_end.max(span.column_start),
            title,
            message
        ))
    }

    pub fn format_all(diagnostics: &[Diagnostic], prefix_trim: Option<&str>) -> String {
        diagnostics
            .iter()
            .filter_map(|d| Self::format_annotation(d, prefix_trim))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
