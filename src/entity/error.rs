use config::ConfigError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TechScanError {
    IoError(io::Error),
    ConfigError(ConfigError),
    ValidationError(String),
    DirectoryNotFound(String),
}

impl fmt::Display for TechScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TechScanError::IoError(err) => write!(f, "I/O error: {}", err),
            TechScanError::ConfigError(err) => write!(f, "Configuration error: {}", err),
            TechScanError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            TechScanError::DirectoryNotFound(path) => write!(f, "Directory not found: {}", path),
        }
    }
}

impl std::error::Error for TechScanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TechScanError::IoError(err) => Some(err),
            TechScanError::ConfigError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for TechScanError {
    fn from(err: io::Error) -> Self {
        TechScanError::IoError(err)
    }
}

impl From<ConfigError> for TechScanError {
    fn from(err: ConfigError) -> Self {
        TechScanError::ConfigError(err)
    }
}
