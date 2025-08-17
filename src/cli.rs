use clap::Parser;

#[derive(Parser)]
#[command(name = "tech-scan")]
#[command(about = "A tool for analyzing and visualizing technology stacks in codes.")]
pub struct Cli {
    #[arg(help = "Directory path to scan")]
    pub dir: String,

    #[arg(short, long, help = "Exclude path patterns (can be used multiple times)")]
    pub exclude: Option<Vec<String>>,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
