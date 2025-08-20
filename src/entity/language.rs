use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Language {
    pub name: &'static str,
    pub exts: &'static [&'static str],
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Language", 2)?;
        state.serialize_field("name", self.name)?;
        state.serialize_field("exts", self.exts)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!("Language deserialization not supported");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod serialize {
        use super::*;

        #[test]
        fn test_serialize_single_extension() {
            let lang = Language {
                name: "Python",
                exts: &["py"],
            };

            let json = serde_json::to_string(&lang).unwrap();
            let expected = r#"{"name":"Python","exts":["py"]}"#;

            assert_eq!(json, expected);
        }

        #[test]
        fn test_serialize_multiple_extension() {
            let lang = Language {
                name: "JavaScript",
                exts: &["js", "jsx"],
            };

            let json = serde_json::to_string(&lang).unwrap();
            let expected = r#"{"name":"JavaScript","exts":["js","jsx"]}"#;

            assert_eq!(json, expected);
        }

        #[test]
        fn test_serialize_pretty() {
            let lang = Language {
                name: "HTML",
                exts: &["html", "htm"],
            };

            let json = serde_json::to_string_pretty(&lang).unwrap();
            let expected =
                "{\n  \"name\": \"HTML\",\n  \"exts\": [\n    \"html\",\n    \"htm\"\n  ]\n}";

            assert_eq!(json, expected);
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        #[should_panic(expected = "Language deserialization not supported")]
        fn test_deserialize_panics() {
            let json = r#"{"name":"Rust","exts":["rs"]}"#;
            let _: Language = serde_json::from_str(json).unwrap();
        }
    }
}
