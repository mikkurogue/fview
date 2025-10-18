use clap::Parser;

use crate::config::Unit;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Directory to view
    #[arg(default_value = "./")]
    pub dir: String,

    /// Canonicalize file paths
    #[arg(short = 'C', long)]
    pub canonicalize: bool,

    /// Maximum depth to traverse
    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    #[arg(short = 'H')]
    pub show_hidden: bool,

    #[arg(short = 't', long)]
    pub table: bool,

    #[arg(short = 'u', long, default_value = "bytes")]
    pub unit: Option<Unit>,

    #[arg(short = 'r', long)]
    pub reversed: bool,
}
