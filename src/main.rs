mod cli;
mod config;
mod entity;
mod service;

use crate::cli::Cli;
use crate::service::{Reporter, Scanner, ScannerOptions};

fn main() {
    let cli = Cli::new().unwrap_or_else(|e| {
        eprintln!("Failed initializing CLI: {}", e);
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

    let reporter = Reporter::new(cli.dir.clone()).unwrap_or_else(|e| {
        eprintln!("Error initializing reporter: {}", e);
        std::process::exit(1);
    });

    let report = reporter.create(files);

    let json_pretty = serde_json::to_string_pretty(&report).unwrap();
    println!("{}", json_pretty);
}
