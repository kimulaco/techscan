use crate::entity::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub dir: String,
    pub total_file_count: u64,
    pub languages: Vec<LanguageReport>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageReport {
    pub language: Language,
    pub file_count: u64,
    pub file_paths: Vec<String>,
}
