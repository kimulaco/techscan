use crate::config::REPORTER_FORMAT_JSON;
use crate::entity::Report;
use std::io;

pub struct Reporter;

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_format(format: &str) -> io::Result<()> {
        match format {
            REPORTER_FORMAT_JSON => Ok(()),
            _ => Err(io::Error::other(format!(
                "Unsupported reporter format: '{}'. Only '{}' is supported.",
                format, REPORTER_FORMAT_JSON,
            ))),
        }
    }

    pub fn output(&self, report: &Report, format: &str) -> io::Result<()> {
        Self::validate_format(format)?;

        let output_string = match format {
            REPORTER_FORMAT_JSON => self.to_json(report)?,
            _ => unreachable!("Format validation should have caught this"),
        };

        println!("{}", output_string);
        Ok(())
    }

    fn to_json(&self, report: &Report) -> io::Result<String> {
        serde_json::to_string_pretty(report)
            .map_err(|e| io::Error::other(format!("JSON serialization error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Language, LanguageReport};

    fn create_test_report() -> Report {
        let language = Language {
            name: "Rust",
            exts: &["rs"],
        };

        let language_report = LanguageReport {
            language,
            file_count: 5,
            file_paths: vec!["src/main.rs".to_string(), "src/lib.rs".to_string()],
        };

        Report {
            dir: "/test/path".to_string(),
            total_file_count: 5,
            languages: vec![language_report],
        }
    }

    #[test]
    fn test_validate_format_valid() {
        assert!(Reporter::validate_format("json").is_ok());
    }

    #[test]
    fn test_validate_format_invalid() {
        let result = Reporter::validate_format("xml");
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert_eq!(
            error_msg,
            "Unsupported reporter format: 'xml'. Only 'json' is supported."
        );
    }

    #[test]
    fn test_to_json_success() {
        let reporter = Reporter::new();
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
}
