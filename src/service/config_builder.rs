use crate::entity::AppConfig;
use config::{Config, ConfigError, File};

#[derive(Debug)]
pub struct ConfigBuilder {
    pub exclude: Option<Vec<String>>,
    pub reporter: Option<String>,
}

impl ConfigBuilder {
    pub fn from_cli_args(exclude: &Option<Vec<String>>, reporter: &Option<String>) -> Self {
        Self {
            exclude: exclude.clone(),
            reporter: reporter.clone(),
        }
    }

    pub fn merge_file_config(mut self, config_path: &Option<String>) -> Result<Self, String> {
        if let Some(path) = config_path {
            let file_config = self
                .load_config_file(path)
                .map_err(|err| format!("Failed to load config file: {}: {}", path, err))?;

            if self.exclude.is_none() {
                self.exclude = file_config.exclude;
            }
            if self.reporter.is_none() {
                self.reporter = file_config.reporter;
            }
        }
        Ok(self)
    }

    pub fn build(self) -> AppConfig {
        AppConfig {
            exclude: self.exclude,
            reporter: self.reporter,
        }
    }

    fn load_config_file(&self, path: &str) -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_cli_args {
        use super::*;

        #[test]
        fn test_from_cli_args_with_values() {
            let exclude = Some(vec!["*.log".to_string(), "exclude_dir".to_string()]);
            let reporter = Some("json".to_string());

            let builder = ConfigBuilder::from_cli_args(&exclude, &reporter);

            assert_eq!(builder.exclude, exclude);
            assert_eq!(builder.reporter, reporter);
        }

        #[test]
        fn test_from_cli_args_with_none() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);

            assert_eq!(builder.exclude, None);
            assert_eq!(builder.reporter, None);
        }
    }

    mod build {
        use super::*;

        #[test]
        fn test_build_with_values() {
            let exclude = Some(vec!["*.tmp".to_string()]);
            let reporter = Some("json".to_string());

            let builder = ConfigBuilder {
                exclude: exclude.clone(),
                reporter: reporter.clone(),
            };

            let config = builder.build();

            assert_eq!(config.exclude, exclude);
            assert_eq!(config.reporter, reporter);
        }

        #[test]
        fn test_build_with_none() {
            let builder = ConfigBuilder {
                exclude: None,
                reporter: None,
            };

            let config = builder.build();

            assert_eq!(config.exclude, None);
            assert_eq!(config.reporter, None);
        }
    }

    mod merge_file_config {
        use super::*;

        #[test]
        fn test_merge_file_config_with_none() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder.merge_file_config(&None);

            assert!(result.is_ok());

            let builder = result.unwrap();
            assert_eq!(builder.exclude, None);
            assert_eq!(builder.reporter, None);
        }

        #[test]
        fn test_merge_file_config_with_valid_json() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder
                .merge_file_config(&Some("tests/fixtures/cli/config/complete.json".to_string()));

            assert!(result.is_ok());
            let builder = result.unwrap();

            assert_eq!(
                builder.exclude,
                Some(vec!["*.test.*".to_string(), "exclude_dir".to_string()])
            );
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_with_valid_json5() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder.merge_file_config(&Some(
                "tests/fixtures/cli/config/complete.json5".to_string(),
            ));

            assert!(result.is_ok());
            let builder = result.unwrap();

            assert_eq!(
                builder.exclude,
                Some(vec!["*.test.*".to_string(), "exclude_dir".to_string()])
            );
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_with_valid_yaml() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder
                .merge_file_config(&Some("tests/fixtures/cli/config/complete.yaml".to_string()));

            assert!(result.is_ok());
            let builder = result.unwrap();

            assert_eq!(
                builder.exclude,
                Some(vec!["*.test.*".to_string(), "exclude_dir".to_string()])
            );
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_with_valid_yml() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder
                .merge_file_config(&Some("tests/fixtures/cli/config/complete.yml".to_string()));

            assert!(result.is_ok());
            let builder = result.unwrap();

            assert_eq!(
                builder.exclude,
                Some(vec!["*.test.*".to_string(), "exclude_dir".to_string()])
            );
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_with_valid_toml() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder
                .merge_file_config(&Some("tests/fixtures/cli/config/complete.toml".to_string()));

            assert!(result.is_ok());
            let builder = result.unwrap();

            assert_eq!(
                builder.exclude,
                Some(vec!["*.test.*".to_string(), "exclude_dir".to_string()])
            );
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_cli_args_take_priority() {
            let cli_exclude = Some(vec!["*.cli".to_string()]);
            let cli_reporter = Some("table".to_string());

            let builder = ConfigBuilder::from_cli_args(&cli_exclude, &cli_reporter);
            let result = builder
                .merge_file_config(&Some("tests/fixtures/cli/config/complete.json".to_string()));

            assert!(result.is_ok());

            let builder = result.unwrap();
            assert_eq!(builder.exclude, cli_exclude);
            assert_eq!(builder.reporter, cli_reporter);
        }

        #[test]
        fn test_merge_file_config_partial_cli_args() {
            let cli_exclude = Some(vec!["*.cli".to_string()]);
            let cli_reporter = None;

            let builder = ConfigBuilder::from_cli_args(&cli_exclude, &cli_reporter);
            let result = builder
                .merge_file_config(&Some("tests/fixtures/cli/config/complete.json".to_string()));

            assert!(result.is_ok());

            let builder = result.unwrap();
            assert_eq!(builder.exclude, cli_exclude);
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_with_partial_config_file() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder.merge_file_config(&Some(
                "tests/fixtures/cli/config/reporter_only.yaml".to_string(),
            ));

            assert!(result.is_ok());

            let builder = result.unwrap();
            assert_eq!(builder.exclude, None);
            assert_eq!(builder.reporter, Some("json".to_string()));
        }

        #[test]
        fn test_merge_file_config_with_nonexistent_file() {
            let builder = ConfigBuilder::from_cli_args(&None, &None);
            let result = builder.merge_file_config(&Some("nonexistent/config.json".to_string()));

            assert!(result.is_err());

            let error_msg = result.unwrap_err();
            assert!(error_msg.contains("Failed to load config file: nonexistent/config.json:"));
        }
    }
}
