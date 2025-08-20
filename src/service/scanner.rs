use crate::entity::File;
use crate::service::ScannerOptions;
use ignore::{overrides::OverrideBuilder, Walk, WalkBuilder};
use std::io;
use std::path::Path;

const GLOBAL_EXCLUDE_PATH: [&str; 2] = [".git", ".DS_Store"];

pub struct Scanner {
    dir: String,
    opts: ScannerOptions,
}

impl Scanner {
    pub fn new(dir: String, opts: Option<ScannerOptions>) -> io::Result<Self> {
        if !Path::new(&dir).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Directory does not exist: {}", dir),
            ));
        }

        let opts = opts.map_or_else(ScannerOptions::default, |o| o);

        Ok(Self { dir, opts })
    }

    pub fn scan(&self) -> io::Result<Vec<File>> {
        let entries = self._walk_dir()?;
        let mut files = Vec::new();

        for entry in entries {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_none_or(|ft| !ft.is_file()) {
                        continue;
                    }
                    match File::from_dir_entry(&entry) {
                        Ok(file) => files.push(file),
                        Err(e) => {
                            eprintln!("Error processing file {}: {}", entry.path().display(), e,)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                }
            }
        }

        Ok(files)
    }

    fn _walk_dir(&self) -> io::Result<Walk> {
        let mut override_builder = OverrideBuilder::new(&self.dir);

        for pattern in GLOBAL_EXCLUDE_PATH.iter() {
            override_builder
                .add(&format!("!{}", pattern))
                .map_err(|e| {
                    io::Error::other(format!(
                        "Failed to add global exclude pattern '{}': {}",
                        pattern, e
                    ))
                })?;
        }

        for pattern in &self.opts.exclude {
            override_builder
                .add(&format!("!{}", pattern))
                .map_err(|e| {
                    io::Error::other(format!(
                        "Failed to add exclude pattern '{}': {}",
                        pattern, e
                    ))
                })?;
        }

        let overrides = override_builder
            .build()
            .map_err(|e| io::Error::other(format!("Failed to build overrides: {}", e)))?;

        let entries = WalkBuilder::new(&self.dir)
            .git_ignore(true)
            .hidden(false)
            .overrides(overrides)
            .build();

        Ok(entries)
    }
}
