use std::io;
use walkdir::DirEntry;

pub struct File {
    pub path: String,
    pub name: String,
    pub ext: Option<String>,
    pub size: u64,
}

impl File {
    pub fn from_dir_entry(entry: &DirEntry) -> io::Result<File> {
        let path = entry.path();
        let name = path.file_name()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to get file name from path: {}", path.display()),
                )
            })?
            .to_string_lossy()
            .into_owned();

        Ok(File {
            path: path.to_string_lossy().into_owned(),
            name,
            ext: path.extension().and_then(|s| s.to_str()).map(|s| s.to_string()),
            size: entry.metadata()?.len(),
        })
    }
}
