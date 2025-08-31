mod cli;
mod config;
mod entity;
mod service;

use crate::cli::{Cli, Commands};
use crate::config::REPORTER_FORMAT_TABLE;
use crate::entity::LanguageScannerOptions;
use crate::service::{ConfigBuilder, LanguageReporter, LanguageScanner};

fn main() {
    let cli = Cli::new().unwrap_or_else(|e| {
        eprintln!("Failed initializing CLI: {}", e);
        std::process::exit(1);
    });

    match &cli.command {
        Commands::Language {
            dir,
            exclude,
            reporter,
            config,
        } => {
            handle_language_command(dir, exclude, reporter, config);
        }
    }
}

fn handle_language_command(
    dir: &str,
    exclude: &Option<Vec<String>>,
    reporter: &Option<String>,
    config: &Option<String>,
) {
    let config_builder = ConfigBuilder::from_cli_args(exclude, reporter)
        .merge_file_config(config)
        .unwrap_or_else(|error_msg| {
            eprintln!("Error: {}", error_msg);
            std::process::exit(1);
        });

    let final_config = config_builder.build();

    let reporter_format = final_config
        .reporter
        .as_deref()
        .unwrap_or(REPORTER_FORMAT_TABLE);

    LanguageReporter::validate_format(reporter_format).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let opts = LanguageScannerOptions {
        exclude: final_config.exclude.unwrap_or_default(),
    };

    let scanner = LanguageScanner::new(dir, Some(opts)).unwrap_or_else(|e| {
        eprintln!("Error initializing scanner: {}", e);
        std::process::exit(1);
    });

    println!("Processing directory: {}", dir);

    let files = scanner.scan().unwrap_or_else(|e| {
        eprintln!("Error scanning directory: {}", e);
        std::process::exit(1);
    });

    let report = scanner.analyze(files);

    let reporter = LanguageReporter::new();

    reporter
        .output(&report, reporter_format)
        .unwrap_or_else(|e| {
            eprintln!("Error outputting report: {}", e);
            std::process::exit(1);
        });
}
