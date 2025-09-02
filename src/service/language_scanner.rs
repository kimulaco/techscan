use crate::config::LanguageConfig;
use crate::entity::LanguageScannerOptions;
use crate::entity::{File, LanguageReport, LanguageReportItem, Result, TechScanError};
use ignore::{overrides::OverrideBuilder, Walk, WalkBuilder};
use std::collections::HashMap;
use std::path::Path;

const GLOBAL_EXCLUDE_PATH: [&str; 2] = [".git", ".DS_Store"];

#[derive(Debug)]
pub struct LanguageScanner {
    dir: String,
    opts: LanguageScannerOptions,
}

impl LanguageScanner {
    pub fn new(dir: &str, opts: Option<LanguageScannerOptions>) -> Result<Self> {
        if !Path::new(dir).exists() {
            return Err(TechScanError::DirectoryNotFound(dir.to_string()));
        }

        let opts = opts.map_or_else(LanguageScannerOptions::default, |o| o);

        Ok(Self {
            dir: dir.to_string(),
            opts,
        })
    }

    pub fn scan(&self) -> Result<Vec<File>> {
        let entries = self._walk_dir()?;
        let mut files = Vec::new();

        for entry in entries {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_none_or(|ft| !ft.is_file()) {
                        continue;
                    }
                    match File::from_dir_entry(&entry) {
                        Ok(file) => files.push(file),
                        Err(e) => {
                            eprintln!("Error processing file {}: {}", entry.path().display(), e,)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                }
            }
        }

        Ok(files)
    }

    pub fn analyze(&self, files: Vec<File>) -> LanguageReport {
        let mut language_data: HashMap<String, Vec<String>> = HashMap::new();

        for file in &files {
            if let Some(ext) = &file.ext {
                let ext_lower = ext.to_lowercase();
                if let Some(language) = LanguageConfig::detect_language(&ext_lower) {
                    language_data
                        .entry(language.name.to_string())
                        .or_default()
                        .push(file.path.clone());
                }
            }
        }

        let mut languages = Vec::new();
        for (lang_name, file_paths) in language_data {
            if let Some(language) = LanguageConfig::get_language_by_name(&lang_name) {
                languages.push(LanguageReportItem {
                    language,
                    file_count: file_paths.len() as u64,
                    file_paths,
                });
            }
        }

        languages.sort_by(|a, b| b.file_count.cmp(&a.file_count));

        LanguageReport {
            dir: self.dir.clone(),
            total_file_count: files.len() as u64,
            languages,
        }
    }

    fn _walk_dir(&self) -> Result<Walk> {
        let mut override_builder = OverrideBuilder::new(&self.dir);

        for pattern in GLOBAL_EXCLUDE_PATH.iter() {
            override_builder
                .add(&format!("!{}", pattern))
                .map_err(|e| {
                    TechScanError::ValidationError(format!(
                        "Failed to add global exclude pattern '{}': {}",
                        pattern, e
                    ))
                })?;
        }

        for pattern in &self.opts.exclude {
            override_builder
                .add(&format!("!{}", pattern))
                .map_err(|e| {
                    TechScanError::ValidationError(format!(
                        "Failed to add exclude pattern '{}': {}",
                        pattern, e
                    ))
                })?;
        }

        let overrides = override_builder.build().map_err(|e| {
            TechScanError::ValidationError(format!("Failed to build overrides: {}", e))
        })?;

        let entries = WalkBuilder::new(&self.dir)
            .git_ignore(true)
            .hidden(false)
            .overrides(overrides)
            .build();

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_scan_cli_fixture() {
        let scanner = LanguageScanner::new("tests/fixtures/cli", None)
            .expect("LanguageScanner creation should succeed for existing directory");

        let files = scanner.scan().expect("Scanning should succeed");

        let extensions: HashSet<&str> = files.iter().filter_map(|f| f.ext.as_deref()).collect();

        let expected_extensions = [
            "rs", "js", "ts", "jsx", "tsx", "py", "rb", "go", "c", "cpp", "cc", "cxx", "php",
            "html", "htm", "css", "sass", "scss", "sh", "json", "json5", "toml", "yaml", "yml",
            "cjs", "mjs", "cts", "mts",
        ];

        for ext in expected_extensions.iter() {
            assert!(
                extensions.contains(ext),
                "Expected extension '{}' not found in scanned files",
                ext
            );
        }

        // ファイル数の確認（適切な数のファイルがスキャンされていること）
        assert!(files.len() == 29, "Expected 29 files, got {}", files.len());
    }

    #[test]
    fn test_scanner_nonexistent_directory() {
        let result = LanguageScanner::new("nonexistent/directory", None);

        assert!(result.is_err());

        match result.unwrap_err() {
            TechScanError::DirectoryNotFound(path) => {
                assert_eq!(path, "nonexistent/directory");
            }
            _ => panic!("Expected DirectoryNotFound error"),
        }
    }

    #[test]
    fn test_scan_with_exclude_pattern() {
        let opts = LanguageScannerOptions {
            exclude: vec!["*.rs".to_string()],
        };

        let scanner = LanguageScanner::new("tests/fixtures/cli", Some(opts))
            .expect("LanguageScanner creation should succeed");

        let files = scanner.scan().expect("Scanning should succeed");

        let has_rust_files = files
            .iter()
            .any(|f| f.ext.as_ref() == Some(&"rs".to_string()));
        assert!(!has_rust_files, "Rust files should be excluded");

        let has_js_files = files
            .iter()
            .any(|f| f.ext.as_ref() == Some(&"js".to_string()));
        assert!(has_js_files, "JavaScript files should be included");
    }

    #[test]
    fn test_scan_with_multiple_exclude_patterns() {
        let opts = LanguageScannerOptions {
            exclude: vec!["*.rs".to_string(), "*.js".to_string(), "*.rb".to_string()],
        };

        let scanner = LanguageScanner::new("tests/fixtures/cli", Some(opts))
            .expect("LanguageScanner creation should succeed");

        let files = scanner.scan().expect("Scanning should succeed");

        let excluded_extensions = ["rs", "js", "rb"];
        for ext in excluded_extensions.iter() {
            let has_files = files
                .iter()
                .any(|f| f.ext.as_ref() == Some(&ext.to_string()));
            assert!(
                !has_files,
                "Files with extension '{}' should be excluded",
                ext
            );
        }

        let has_python_files = files
            .iter()
            .any(|f| f.ext.as_ref() == Some(&"py".to_string()));
        assert!(has_python_files, "Python files should be included");
    }
}
