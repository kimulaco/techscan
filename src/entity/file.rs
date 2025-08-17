use std::io;
use walkdir::DirEntry;

pub struct File {
    pub name: String,
    pub path: String,
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
            name,
            path: path.to_string_lossy().into_owned(),
            size: entry.metadata()?.len(),
        })
    }
}
