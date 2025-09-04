use clap::{Parser, Subcommand};
use config::ConfigError;

#[derive(Parser)]
#[command(name = "techscan")]
#[command(about = "A tool for analyzing and visualizing technology stacks in codes.")]
#[command(version)]
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

impl Cli {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self::parse())
    }
}
