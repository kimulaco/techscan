use crate::config::{REPORTER_FORMAT_JSON, REPORTER_FORMAT_TABLE};
use crate::entity::LanguageReport;
use std::io;
use tabled::builder::Builder;
use tabled::settings::{object::Rows, Alignment, Modify, Style};

pub struct LanguageReporter;

impl Default for LanguageReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageReporter {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_format(format: &str) -> io::Result<()> {
        match format {
            REPORTER_FORMAT_JSON | REPORTER_FORMAT_TABLE => Ok(()),
            _ => Err(io::Error::other(format!(
                "Unsupported reporter format: '{}'. Supported formats: {}, {}.",
                format, REPORTER_FORMAT_TABLE, REPORTER_FORMAT_JSON
            ))),
        }
    }

    pub fn output(&self, report: &LanguageReport, format: &str) -> io::Result<()> {
        Self::validate_format(format)?;

        let output_string = match format {
            REPORTER_FORMAT_JSON => self.to_json(report)?,
            REPORTER_FORMAT_TABLE => self.to_table(report)?,
            _ => unreachable!("Format validation should have caught this"),
        };

        println!("{}", output_string);
        Ok(())
    }

    fn to_json(&self, report: &LanguageReport) -> io::Result<String> {
        serde_json::to_string_pretty(report)
            .map_err(|e| io::Error::other(format!("JSON serialization error: {}", e)))
    }

    fn to_table(&self, report: &LanguageReport) -> io::Result<String> {
        let mut output = Vec::new();

        let detected_files_count: u64 = report
            .languages
            .iter()
            .map(|lang_report| lang_report.file_count)
            .sum();

        let excluded_files_count = report.total_file_count - detected_files_count;

        let mut summary_builder = Builder::default();
        summary_builder.push_record(vec!["Item", "Value"]);
        summary_builder.push_record(vec!["Directory", &report.dir]);
        summary_builder.push_record(vec!["Total Files", &report.total_file_count.to_string()]);
        summary_builder.push_record(vec!["Language Files", &detected_files_count.to_string()]);
        summary_builder.push_record(vec!["Excluded Files", &excluded_files_count.to_string()]);

        let summary_table = summary_builder
            .build()
            .with(Style::sharp())
            .with(Modify::new(Rows::new(0..=0)).with(Alignment::center()))
            .to_string();

        let mut lang_builder = Builder::default();
        lang_builder.push_record(vec!["Language", "Files", "Percentage"]);

        for lang_report in &report.languages {
            let percentage = if detected_files_count > 0 {
                (lang_report.file_count as f64 / detected_files_count as f64) * 100.0
            } else {
                0.0
            };
            lang_builder.push_record(vec![
                lang_report.language.name,
                &lang_report.file_count.to_string(),
                &format!("{:.1}%", percentage),
            ]);
        }

        let last_row = report.languages.len() + 1; // +1 for header
        let lang_table = lang_builder
            .build()
            .with(Style::sharp())
            .with(Modify::new(Rows::new(last_row..=last_row)).with(Alignment::center()))
            .to_string();

        output.push("=== Scan Summary ===".to_string());
        output.push(summary_table);
        output.push(String::new());
        output.push("=== Language Statistics ===".to_string());
        output.push(lang_table);

        Ok(output.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Language, LanguageReportItem};

    fn create_test_report() -> LanguageReport {
        let language = Language {
            name: "Rust",
            exts: &["rs"],
        };

        let language_report = LanguageReportItem {
            language,
            file_count: 5,
            file_paths: vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        };

        LanguageReport {
            dir: "/test/path".to_string(),
            total_file_count: 5,
            languages: vec![language_report],
        }
    }

    #[test]
    fn test_validate_format_valid() {
        assert!(LanguageReporter::validate_format("table").is_ok());
    }

    #[test]
    fn test_validate_format_invalid() {
        let result = LanguageReporter::validate_format("xml");
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert_eq!(
            error_msg,
            "Unsupported reporter format: 'xml'. Supported formats: table, json."
        );
    }

    #[test]
    fn test_to_json_success() {
        let reporter = LanguageReporter::new();
        let report = create_test_report();

        let result = reporter.to_json(&report);
        assert!(result.is_ok());

        let json_string = result.unwrap();

        assert!(json_string.contains("\"dir\": \"/test/path\""));
        assert!(json_string.contains("\"total_file_count\": 5"));
        assert!(json_string.contains("\"name\": \"Rust\""));
        assert!(json_string.contains("\"file_count\": 5"));

        let parsed: serde_json::Value =
            serde_json::from_str(&json_string).expect("Generated JSON is invalid");

        assert_eq!(parsed["dir"], "/test/path");
        assert_eq!(parsed["total_file_count"], 5);
    }

    #[test]
    fn test_percentage_excludes_unknown_files() {
        let reporter = LanguageReporter::new();

        let rust_lang = Language {
            name: "Rust",
            exts: &["rs"],
        };
        let js_lang = Language {
            name: "JavaScript",
            exts: &["js"],
        };

        let report = LanguageReport {
            dir: "/test/path".to_string(),
            total_file_count: 100,
            languages: vec![
                LanguageReportItem {
                    language: rust_lang,
                    file_count: 30,
                    file_paths: vec![],
                },
                LanguageReportItem {
                    language: js_lang,
                    file_count: 20,
                    file_paths: vec![],
                },
            ],
        };

        let result = reporter.to_table(&report);
        assert!(result.is_ok());

        let table_output = result.unwrap();

        assert!(table_output.contains("60.0%"));
        assert!(table_output.contains("40.0%"));
    }
}
