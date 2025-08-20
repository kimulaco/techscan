use ignore::DirEntry;
use std::io;
use std::path::Path;

pub struct File {
    pub path: String,
    pub ext: Option<String>,
}

impl File {
    pub fn from_dir_entry(entry: &DirEntry) -> io::Result<File> {
        let path = entry.path();
        Ok(Self::from_path(path))
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> File {
        let path = path.as_ref();
        File {
            path: path.to_string_lossy().into_owned(),
            ext: path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_path {
        use super::*;

        #[test]
        fn test_file_with_extension() {
            let file = File::from_path("/test/file.rs");

            assert_eq!(file.path, "/test/file.rs");
            assert_eq!(file.ext, Some("rs".to_string()));
        }

        #[test]
        fn test_file_without_extension() {
            let file = File::from_path("/test/file");

            assert_eq!(file.path, "/test/file");
            assert_eq!(file.ext, None);
        }

        #[test]
        fn test_file_with_multiple_extensions() {
            let file = File::from_path("/test/file.test.ts");

            assert_eq!(file.path, "/test/file.test.ts");
            assert_eq!(file.ext, Some("ts".to_string()));
        }

        #[test]
        fn test_file_with_hidden_extension() {
            let file = File::from_path("/test/.gitignore");

            assert_eq!(file.path, "/test/.gitignore");
            assert_eq!(file.ext, None);
        }

        #[test]
        fn test_file_path_from_path() {
            use std::path::Path;
            let path = Path::new("src/main.rs");
            let file = File::from_path(path);

            assert_eq!(file.path, "src/main.rs");
            assert_eq!(file.ext, Some("rs".to_string()));
        }
    }
}
