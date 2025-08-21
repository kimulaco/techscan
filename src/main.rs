mod cli;
mod config;
mod entity;
mod service;

use crate::cli::Cli;
use crate::config::REPORTER_FORMAT_JSON;
use crate::service::{Reporter, Scanner, ScannerOptions};

fn main() {
    let cli = Cli::new().unwrap_or_else(|e| {
        eprintln!("Failed initializing CLI: {}", e);
        std::process::exit(1);
    });

    let reporter_format = cli.reporter.as_deref().unwrap_or(REPORTER_FORMAT_JSON);

    Reporter::validate_format(reporter_format).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let opts = ScannerOptions {
        exclude: cli.exclude.unwrap_or_default(),
    };

    let scanner = Scanner::new(cli.dir.clone(), Some(opts)).unwrap_or_else(|e| {
        eprintln!("Error initializing scanner: {}", e);
        std::process::exit(1);
    });

    println!("Processing directory: {}", &cli.dir);

    let files = scanner.scan().unwrap_or_else(|e| {
        eprintln!("Error scanning directory: {}", e);
        std::process::exit(1);
    });

    let report = scanner.analyze(files);

    let reporter = Reporter::new();

    reporter
        .output(&report, reporter_format)
        .unwrap_or_else(|e| {
            eprintln!("Error outputting report: {}", e);
            std::process::exit(1);
        });
}
