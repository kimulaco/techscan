mod cli;
mod entity;
mod service;

use crate::cli::Cli;
use crate::service::scanner::Scanner;
use crate::service::scanner_options::ScannerOptions;

fn main() {
    let cli = Cli::new();

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

    for file in files {
        println!(
            "{} ({}) ({}) ({} B)",
            file.path,
            file.name,
            file.ext.unwrap_or_default(),
            file.size
        );
    }
}
