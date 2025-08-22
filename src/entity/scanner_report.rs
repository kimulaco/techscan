use crate::entity::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannerReport {
    pub dir: String,
    pub total_file_count: u64,
    pub languages: Vec<ScannerLanguage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScannerLanguage {
    pub language: Language,
    pub file_count: u64,
    pub file_paths: Vec<String>,
}
