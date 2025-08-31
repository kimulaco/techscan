pub mod config;
pub mod entity;
pub mod service;

pub use entity::{File, Language, LanguageReport, LanguageReportItem, LanguageScannerOptions};

pub use config::{LanguageConfig, REPORTER_FORMAT_JSON, REPORTER_FORMAT_TABLE};

pub use service::{LanguageReporter, LanguageScanner};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
