use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "tech-scan")]
#[command(about = "A tool for analyzing and visualizing technology stacks in codes.")]
struct Cli {
    dir: String,
}

fn main() {
    let cli = Cli::parse();

    println!("Processing directory: {}", cli.dir);

    let entries = WalkDir::new(&cli.dir);
    for entry in entries {
        match entry {
            Ok(entry) => {
                let file_type = entry.file_type();
                let path = entry.path();

                if file_type.is_file() {
                    println!("{}", &path.to_string_lossy());
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
            }
        }
    }
}
