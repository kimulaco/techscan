use crate::entity::Language;

const ASTRO: Language = Language {
    name: "Astro",
    exts: &["astro"],
};

const C: Language = Language {
    name: "C",
    exts: &["c"],
};

const COBOL: Language = Language {
    name: "COBOL",
    exts: &["cbl", "cob", "cobol"],
};

const CPP: Language = Language {
    name: "C++",
    exts: &["cpp", "cc", "cxx"],
};

const CSHARP: Language = Language {
    name: "C#",
    exts: &["cs"],
};

const CSS: Language = Language {
    name: "CSS",
    exts: &["css"],
};

const DART: Language = Language {
    name: "Dart",
    exts: &["dart"],
};

const ELIXIR: Language = Language {
    name: "Elixir",
    exts: &["ex", "exs"],
};

const GO: Language = Language {
    name: "Go",
    exts: &["go"],
};

const HASKELL: Language = Language {
    name: "Haskell",
    exts: &["hs"],
};

const HTML: Language = Language {
    name: "HTML",
    exts: &["html", "htm"],
};

const JAVA: Language = Language {
    name: "Java",
    exts: &["java"],
};

const JAVASCRIPT: Language = Language {
    name: "JavaScript",
    exts: &["js", "mjs", "cjs", "jsx"],
};

const KOTLIN: Language = Language {
    name: "Kotlin",
    exts: &["kt", "kts"],
};

const LUA: Language = Language {
    name: "Lua",
    exts: &["lua"],
};

const OBJECTIVE_C: Language = Language {
    name: "Objective-C",
    exts: &["m", "mm"],
};

const PERL: Language = Language {
    name: "Perl",
    exts: &["pl", "pm"],
};

const PHP: Language = Language {
    name: "PHP",
    exts: &["php"],
};

const PYTHON: Language = Language {
    name: "Python",
    exts: &["py"],
};

const R: Language = Language {
    name: "R",
    exts: &["r", "R"],
};

const RUBY: Language = Language {
    name: "Ruby",
    exts: &["rb"],
};

const SCALA: Language = Language {
    name: "Scala",
    exts: &["scala", "sc"],
};

const RUST: Language = Language {
    name: "Rust",
    exts: &["rs"],
};

const SCSS: Language = Language {
    name: "SCSS",
    exts: &["scss", "sass"],
};

const SHELL: Language = Language {
    name: "Shell",
    exts: &["sh"],
};

const SVELTE: Language = Language {
    name: "Svelte",
    exts: &["svelte"],
};

const SWIFT: Language = Language {
    name: "Swift",
    exts: &["swift"],
};

const TYPESCRIPT: Language = Language {
    name: "TypeScript",
    exts: &["ts", "mts", "cts", "tsx"],
};

const VUE: Language = Language {
    name: "Vue",
    exts: &["vue"],
};

const SUPPORTER_LANGUAGES: &[Language] = &[
    ASTRO,
    C,
    CPP,
    CSHARP,
    COBOL,
    CSS,
    DART,
    ELIXIR,
    GO,
    HASKELL,
    HTML,
    JAVA,
    JAVASCRIPT,
    KOTLIN,
    LUA,
    OBJECTIVE_C,
    PERL,
    PHP,
    PYTHON,
    R,
    RUBY,
    RUST,
    SCALA,
    SCSS,
    SHELL,
    SVELTE,
    SWIFT,
    TYPESCRIPT,
    VUE,
];

pub struct LanguageConfig;

impl LanguageConfig {
    pub fn detect_language(ext: &str) -> Option<Language> {
        SUPPORTER_LANGUAGES
            .iter()
            .find(|lang| lang.exts.contains(&ext))
            .copied()
    }

    pub fn get_language_by_name(name: &str) -> Option<Language> {
        SUPPORTER_LANGUAGES
            .iter()
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
