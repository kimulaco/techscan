use std::io;
use std::collections::HashMap;
use crate::entity::{File, Report, LanguageReport};
use crate::config::LanguageConfig;

pub struct Reporter {
    dir: String,
}

impl Reporter {
    pub fn new(dir: String) -> io::Result<Self> {
        Ok(Self { dir })
    }

    pub fn create(&self, files: Vec<File>) -> Report {
        let mut language_data: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files {
            if let Some(ext) = &file.ext {
                let ext_lower = ext.to_lowercase();
                if let Some(language) = LanguageConfig::detect_language(&ext_lower) {
                    language_data.entry(language.name.to_string()).or_default().push(file.path.clone());
                }
            }
        }

        let mut languages = Vec::new();
        for (lang_name, file_paths) in language_data {
            if let Some(language) = LanguageConfig::get_language_by_name(&lang_name) {
                languages.push(LanguageReport {
                    language,
                    file_count: file_paths.len() as u64,
                    file_paths,
                });
            }
        }

        languages.sort_by(|a, b| b.file_count.cmp(&a.file_count));

        Report {
            dir: self.dir.clone(),
            total_file_count: files.len() as u64,
            languages,
        }
    }

}
