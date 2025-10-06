use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Directory to view
    #[arg(default_value = ".")]
    pub dir: String,

    /// Canonicalize file paths
    #[arg(short = 'C', long)]
    pub canonicalize: bool,

    /// Maximum depth to traverse
    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    /// Show the current directory
    #[arg(short = 'c', long)]
    pub show_current: bool,
}
