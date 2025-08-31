use crate::entity::AppConfig;
use config::{Config, ConfigError, File};

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
