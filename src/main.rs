mod cli;
mod entity;
mod service;

use crate::cli::Cli;
use crate::service::scanner::scan_dir;

fn main() {
    let cli = Cli::new();

    println!("Processing directory: {}", cli.dir);

    let files = scan_dir(&cli.dir);
    match files {
        Ok(files) => {
            for file in files {
                println!(
                    "{} ({}) ({}) ({} B)",
                    file.path,
                    file.name,
                    file.ext.unwrap_or_default(),
                    file.size
                );
            }
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            std::process::exit(1);
        }
    }
}
