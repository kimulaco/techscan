use crate::entity::Language;

const RUST: Language = Language {
    name: "Rust",
    exts: &["rs"],
};

const JAVASCRIPT: Language = Language {
    name: "JavaScript",
    exts: &["js", "mjs", "cjs", "jsx"],
};

const TYPESCRIPT: Language = Language {
    name: "TypeScript",
    exts: &["ts", "mts", "cts", "tsx"],
};

const PYTHON: Language = Language {
    name: "Python",
    exts: &["py"],
};

const JAVA: Language = Language {
    name: "Java",
    exts: &["java"],
};

const C: Language = Language {
    name: "C",
    exts: &["c"],
};

const CPP: Language = Language {
    name: "C++",
    exts: &["cpp", "cc", "cxx"],
};

const GO: Language = Language {
    name: "Go",
    exts: &["go"],
};

const RUBY: Language = Language {
    name: "Ruby",
    exts: &["rb"],
};

const PHP: Language = Language {
    name: "PHP",
    exts: &["php"],
};

const HTML: Language = Language {
    name: "HTML",
    exts: &["html", "htm"],
};

const CSS: Language = Language {
    name: "CSS",
    exts: &["css"],
};

const SCSS: Language = Language {
    name: "SCSS",
    exts: &["scss"]
};

const JSON: Language = Language {
    name: "JSON",
    exts: &["json", "json5"],
};

const TOML: Language = Language {
    name: "TOML",
    exts: &["toml"],
};

const YAML: Language = Language {
    name: "YAML",
    exts: &["yaml", "yml"],
};

const MARKDOWN: Language = Language {
    name: "Markdown",
    exts: &["md", "mdx"],
};

const SUPPORTER_LANGUAGES: &[Language] = &[
    RUST,
    JAVASCRIPT,
    TYPESCRIPT,
    PYTHON,
    JAVA,
    C,
    CPP,
    GO,
    RUBY,
    PHP,
    HTML,
    CSS,
    SCSS,
    JSON,
    TOML,
    YAML,
    MARKDOWN,
];

pub struct LanguageConfig;

impl LanguageConfig {
    pub fn detect_language(ext: &str) -> Option<Language> {
        SUPPORTER_LANGUAGES.iter()
            .find(|lang| lang.exts.contains(&ext))
            .copied()
    }

    pub fn get_language_by_name(name: &str) -> Option<Language> {
        SUPPORTER_LANGUAGES.iter()
            .find(|lang| lang.name == name)
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod detect_language {
        use super::*;

        #[test]
        fn should_detect_rust() {
            let result = LanguageConfig::detect_language("rs");
            assert!(result.is_some());
            assert_eq!(result.unwrap().name, "Rust");
        }

        #[test]
        fn should_detect_multiple_variants() {
            let extensions = ["ts", "mts", "cts", "tsx"];
            for ext in extensions {
                let result = LanguageConfig::detect_language(ext);
                assert!(result.is_some());
                assert_eq!(result.unwrap().name, "TypeScript");
            }
        }

        #[test]
        fn should_return_none_for_unsupported_extension() {
            let result = LanguageConfig::detect_language("xyz");
            assert!(result.is_none());
        }

        #[test]
        fn should_return_none_for_empty_extension() {
            let result = LanguageConfig::detect_language("");
            assert!(result.is_none());
        }
    }

    mod get_language_by_name {
        use super::*;

        #[test]
        fn should_return_language_when_found() {
            let result = LanguageConfig::get_language_by_name("Rust");
            assert!(result.is_some());
            assert_eq!(result.unwrap().name, "Rust");
        }

        #[test]
        fn should_return_none_when_not_found() {
            let result = LanguageConfig::get_language_by_name("UnknownLanguage");
            assert!(result.is_none());
        }
    }
}