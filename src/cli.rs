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
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn create_test_cli() -> Cli {
        Cli::parse_from(["techscan", "lang", "."])
    }

    mod load_config_file {
        use super::*;

        #[test]
        fn test_load_json_file() {
            let cli = create_test_cli();
            let config_path = "tests/fixtures/cli/config/complete.json";
            let config = cli.load_config_file(config_path).unwrap();

            assert_eq!(
                config.exclude,
                Some(vec!["*.test.*".into(), "exclude_dir".into()])
            );
            assert_eq!(config.reporter, Some("json".into()));
        }

        #[test]
        fn test_load_json5_file() {
            let cli = create_test_cli();
            let config_path = "tests/fixtures/cli/config/complete.json5";
            let config = cli.load_config_file(config_path).unwrap();

            assert_eq!(
                config.exclude,
                Some(vec!["*.test.*".into(), "exclude_dir".into()])
            );
            assert_eq!(config.reporter, Some("json".into()));
        }

        #[test]
        fn test_load_toml_file() {
            let cli = create_test_cli();
            let config_path = "tests/fixtures/cli/config/complete.toml";
            let config = cli.load_config_file(config_path).unwrap();

            assert_eq!(
                config.exclude,
                Some(vec!["*.test.*".into(), "exclude_dir".into()])
            );
            assert_eq!(config.reporter, Some("json".into()));
        }

        #[test]
        fn test_load_yaml_file() {
            let cli = create_test_cli();
            let config_path = "tests/fixtures/cli/config/complete.yaml";
            let config = cli.load_config_file(config_path).unwrap();

            assert_eq!(
                config.exclude,
                Some(vec!["*.test.*".into(), "exclude_dir".into()])
            );
            assert_eq!(config.reporter, Some("json".into()));
        }

        #[test]
        fn test_load_yml_file() {
            let cli = create_test_cli();
            let config_path = "tests/fixtures/cli/config/complete.yml";
            let config = cli.load_config_file(config_path).unwrap();

            assert_eq!(
                config.exclude,
                Some(vec!["*.test.*".into(), "exclude_dir".into()])
            );
            assert_eq!(config.reporter, Some("json".into()));
        }

        #[test]
        fn test_load_reporter_only() {
            let cli = create_test_cli();
            let config_path = "tests/fixtures/cli/config/reporter_only.yaml";
            let config = cli.load_config_file(config_path).unwrap();

            assert_eq!(
                config.exclude,
                Some(vec!["*.test.*".into(), "exclude_dir".into()])
            );
            assert_eq!(config.reporter, None);
        }

        #[test]
        fn test_nonexistent_config_file() {
            let cli = create_test_cli();
            let result = cli.load_config_file("non_existent.json");
            assert!(result.is_err());
        }
    }
}
