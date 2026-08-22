use clippy_cf_reporter::{
    codeclimate::CodeClimateIssue, filter_diagnostics, generate_report, parse_compiler_messages,
    sarif::SarifReport, OutputFormat, ReporterConfig,
};

const SAMPLE_CLIPPY_JSON: &str = r#"
{"reason":"compiler-message","package_id":"sample-worker 0.1.0 (path+file:///sample)","target":{"kind":["cdylib"],"crate_types":["cdylib"],"name":"sample_worker","src_path":"/workspace/src/lib.rs"},"message":{"rendered":"warning: redundant closure\n --> src/lib.rs:14:23\n","children":[],"code":{"code":"clippy::redundant_closure","explanation":"Checks for closures that just call another function."},"level":"warning","message":"redundant closure","spans":[{"byte_end":450,"byte_start":435,"column_end":38,"column_start":23,"file_name":"src/lib.rs","is_primary":true,"line_end":14,"line_start":14,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":38,"highlight_start":23,"text":"    let res = req.map(|r| handler(r));"}]}]}}
{"reason":"compiler-message","package_id":"sample-worker 0.1.0 (path+file:///sample)","target":{"kind":["cdylib"],"crate_types":["cdylib"],"name":"sample_worker","src_path":"/workspace/src/lib.rs"},"message":{"rendered":"error[E0308]: mismatched types\n --> src/lib.rs:25:5\n","children":[],"code":{"code":"E0308","explanation":"This error occurs when the compiler was unable to infer the concrete type."},"level":"error","message":"mismatched types","spans":[{"byte_end":800,"byte_start":780,"column_end":15,"column_start":5,"file_name":"src/lib.rs","is_primary":true,"line_end":25,"line_start":25,"suggested_replacement":null,"suggestion_applicability":null,"text":[{"highlight_end":15,"highlight_start":5,"text":"    Response::ok(\"ok\")"}]}]}}
"#;

#[test]
fn test_parse_and_codeclimate_output() {
    let diags = parse_compiler_messages(SAMPLE_CLIPPY_JSON.as_bytes());
    assert_eq!(diags.len(), 2);

    let config = ReporterConfig {
        format: OutputFormat::CodeClimate,
        prefix_trim: None,
        ignore_wasm_bindgen: true,
        min_level: None,
    };

    let filtered = filter_diagnostics(diags, &config);
    assert_eq!(filtered.len(), 2);

    let report_str = generate_report(&filtered, &config);
    let issues: Vec<CodeClimateIssue> = serde_json::from_str(&report_str).expect("Valid JSON");
    assert_eq!(issues.len(), 2);
    assert_eq!(issues[0].check_name, "clippy::redundant_closure");
    assert_eq!(issues[0].severity, "major");
    assert_eq!(issues[0].location.lines.begin, 14);
    assert_eq!(issues[1].check_name, "E0308");
    assert_eq!(issues[1].severity, "critical");
    assert_eq!(issues[1].location.lines.begin, 25);
}

#[test]
fn test_sarif_output() {
    let diags = parse_compiler_messages(SAMPLE_CLIPPY_JSON.as_bytes());
    let config = ReporterConfig {
        format: OutputFormat::Sarif,
        prefix_trim: None,
        ignore_wasm_bindgen: true,
        min_level: None,
    };

    let report_str = generate_report(&diags, &config);
    let sarif: SarifReport = serde_json::from_str(&report_str).expect("Valid SARIF JSON");
    assert_eq!(sarif.version, "2.1.0");
    assert_eq!(sarif.runs.len(), 1);
    assert_eq!(sarif.runs[0].results.len(), 2);
    assert_eq!(sarif.runs[0].results[0].rule_id, "clippy::redundant_closure");
    assert_eq!(sarif.runs[0].results[0].level, "warning");
}

#[test]
fn test_github_output() {
    let diags = parse_compiler_messages(SAMPLE_CLIPPY_JSON.as_bytes());
    let config = ReporterConfig {
        format: OutputFormat::GitHub,
        prefix_trim: None,
        ignore_wasm_bindgen: true,
        min_level: None,
    };

    let report_str = generate_report(&diags, &config);
    assert!(report_str.contains("::warning file=src/lib.rs,line=14"));
    assert!(report_str.contains("::error file=src/lib.rs,line=25"));
}
