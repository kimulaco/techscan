use ignore::DirEntry;
use std::io;

pub struct File {
    pub path: String,
    pub ext: Option<String>,
}

impl File {
    pub fn from_dir_entry(entry: &DirEntry) -> io::Result<File> {
        let path = entry.path();

        Ok(File {
            path: path.to_string_lossy().into_owned(),
            ext: path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string()),
        })
    }
}
