use clap::Parser;
use clippy_cf_reporter::{filter_diagnostics, generate_report, parse_compiler_messages, OutputFormat, ReporterConfig};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(
    name = "clippy-cf-reporter",
    about = "Translates worker-rs / cargo clippy compiler diagnostics into Code Climate, SARIF v2.1.0, and GitHub PR annotations.",
    version
)]
struct Args {
    /// Input file containing JSON output from `cargo clippy --message-format=json` (reads from stdin if omitted)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output format: codeclimate, sarif, or github
    #[arg(short, long, default_value = "codeclimate")]
    format: OutputFormat,

    /// Strip path prefix from file names (useful for Docker/CI paths)
    #[arg(long)]
    prefix_trim: Option<String>,

    /// Ignore warnings generated inside wasm-bindgen and external cargo dependencies
    #[arg(long, default_value_t = true)]
    ignore_wasm_bindgen: bool,

    /// Minimum severity level to include: error, warning, info
    #[arg(long)]
    min_level: Option<String>,

    /// Exit with error code 1 if any compiler errors are encountered
    #[arg(long)]
    fail_on_errors: bool,

    /// Exit with error code 1 if any warnings are encountered
    #[arg(long)]
    fail_on_warnings: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let reader: Box<dyn Read> = match args.input {
        Some(ref path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };

    let raw_diagnostics = parse_compiler_messages(reader);

    let config = ReporterConfig {
        format: args.format,
        prefix_trim: args.prefix_trim,
        ignore_wasm_bindgen: args.ignore_wasm_bindgen,
        min_level: args.min_level,
    };

    let diagnostics = filter_diagnostics(raw_diagnostics, &config);

    let report = generate_report(&diagnostics, &config);
    println!("{}", report);

    let error_count = diagnostics.iter().filter(|d| d.level == "error").count();
    let warning_count = diagnostics.iter().filter(|d| d.level == "warning").count();

    if args.fail_on_errors && error_count > 0 {
        eprintln!("clippy-cf-reporter: Failed with {} error(s)", error_count);
        exit(1);
    }

    if args.fail_on_warnings && (error_count > 0 || warning_count > 0) {
        eprintln!(
            "clippy-cf-reporter: Failed with {} error(s), {} warning(s)",
            error_count, warning_count
        );
        exit(1);
    }

    Ok(())
}
// test comment
