use clap::{Parser, Subcommand};
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "techscan")]
#[command(about = "A tool for analyzing and visualizing technology stacks in codes.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias = "lang")]
    Language {
        #[arg(help = "Directory path to analyze")]
        dir: String,

        #[arg(
            short,
            long,
            help = "Exclude path patterns (can be used multiple times)"
        )]
        exclude: Option<Vec<String>>,

        #[arg(short, long, help = "Output format: table, json [default: table]")]
        reporter: Option<String>,

        #[arg(short, long, help = "Config file path")]
        config: Option<String>,
    },
}

#[derive(Deserialize, Default)]
pub struct AppConfig {
    pub exclude: Option<Vec<String>>,
    pub reporter: Option<String>,
}

impl Cli {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self::parse())
    }

    pub fn load_config_file(&self, path: &str) -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        config.try_deserialize()
    }

    pub fn apply_config(
        &self,
        config_path: &Option<String>,
        exclude: &mut Option<Vec<String>>,
        reporter: &mut Option<String>,
    ) -> Result<(), String> {
        if let Some(path) = config_path {
            match self.load_config_file(path) {
                Ok(config_data) => {
                    if exclude.is_none() && config_data.exclude.is_some() {
                        *exclude = config_data.exclude;
                    }
                    if reporter.is_none() && config_data.reporter.is_some() {
                        *reporter = config_data.reporter;
                    }
                    Ok(())
                }
                Err(_) => Err(format!("Failed to load config file: {}", path)),
            }
        } else {
            Ok(())
        }
    }
}
