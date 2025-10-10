use clap::Parser;
pub mod cli;
pub mod config;
pub mod string_ext;

use crate::{cli::Args, config::Config};

fn main() {
    let cli = Args::parse();

    let mut config = Config::from(cli);

    // If the default directory is still "./", use the actual current working directory
    if config.dir == "./" {
        match std::env::current_dir() {
            Ok(cwd) => {
                config.dir = cwd.to_string_lossy().into_owned();
            }
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
                // Fallback to "./" if we can't get the current directory
                config.dir = "./".to_string();
            }
        }
    }

    config::view_files(Some(config));
}
