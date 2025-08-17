use std::io;
use std::path::Path;
use walkdir::WalkDir;
use crate::entity::file::File;

pub fn scan_dir(dir: &str) -> io::Result<Vec<File>> {
    if !Path::new(dir).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory does not exist: {}", dir)
        ));
    }

    let entries = WalkDir::new(dir);
    let mut files = Vec::new();

    for entry in entries {
        match entry {
            Ok(entry) => {
                if !entry.file_type().is_file() {
                    continue;
                }
                match File::from_dir_entry(&entry) {
                    Ok(file) => files.push(file),
                    Err(e) => eprintln!("Error processing file {}: {}", entry.path().display(), e),
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
            }
        }
    }

    Ok(files)
}
