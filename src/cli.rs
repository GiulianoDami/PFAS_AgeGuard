use clap::{App, Arg, SubCommand};

pub fn print_usage() {
    println!(
        r#"PFAS_AgeGuard

A Rust-based tool for analyzing and monitoring PFAS exposure levels and their potential correlation with biological aging markers.

USAGE:
    pfas_guard [SUBCOMMAND]

SUBCOMMANDS:
    analyze    Analyze PFAS exposure data
    report     Generate aging risk report
    help       Print this help message

EXAMPLES:
    pfas_guard analyze --input data.csv --output results.json
    pfas_guard report --input results.json --format pdf

For more information about a specific command, run 'pfas_guard help <command>'
"#
    );
}

pub fn setup_cli() -> App<'static, 'static> {
    App::new("PFAS_AgeGuard")
        .version("1.0")
        .about("Analyze PFAS exposure and aging risk")
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable verbose output"))
        .subcommand(SubCommand::with_name("analyze")
            .about("Analyze PFAS exposure data")
            .arg(Arg::with_name("input")
                .long("input")
                .value_name("FILE")
                .help("Input CSV file with PFAS data")
                .required(true))
            .arg(Arg::with_name("output")
                .long("output")
                .value_name("FILE")
                .help("Output JSON file with analysis results")
                .required(true)))
        .subcommand(SubCommand::with_name("report")
            .about("Generate aging risk report")
            .arg(Arg::with_name("input")
                .long("input")
                .value_name("FILE")
                .help("Input JSON file with analysis results")
                .required(true))
            .arg(Arg::with_name("format")
                .long("format")
                .value_name("FORMAT")
                .help("Output format (json, pdf, csv)")
                .default_value("json")))
}