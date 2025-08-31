pub mod entity;
pub mod config;
pub mod service;

pub use entity::{
    File,
    Language,
    LanguageReport,
    LanguageReportItem,
    LanguageScannerOptions,
};

pub use config::{
    LanguageConfig,
    REPORTER_FORMAT_JSON,
    REPORTER_FORMAT_TABLE,
};

pub use service::{
    LanguageScanner,
    LanguageReporter,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
