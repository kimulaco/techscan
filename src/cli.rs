use clap::Parser;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "tech-scan")]
#[command(about = "A tool for analyzing and visualizing technology stacks in codes.")]
pub struct Cli {
    #[arg(help = "Directory path to scan")]
    pub dir: String,

    #[arg(
        short,
        long,
        help = "Exclude path patterns (can be used multiple times)"
    )]
    pub exclude: Option<Vec<String>>,

    #[arg(short, long, help = "Output format [default: json]")]
    pub reporter: Option<String>,

    #[arg(short, long, help = "Config file path")]
    pub config: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct AppConfig {
    pub exclude: Option<Vec<String>>,
    pub reporter: Option<String>,
}

impl Cli {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cli = Self::parse();

        if let Some(config_path) = &cli.config {
            let config_data = cli.load_config_file(config_path)?;

            if cli.exclude.is_none() && config_data.exclude.is_some() {
                cli.exclude = config_data.exclude;
            }

            if cli.reporter.is_none() && config_data.reporter.is_some() {
                cli.reporter = config_data.reporter;
            }
        }

        Ok(cli)
    }

    fn load_config_file(&self, path: &str) -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        config.try_deserialize()
    }
}
