use clap::Parser;
pub mod cli;

use std::{error::Error, os::unix::fs::PermissionsExt};

use chrono::{DateTime, Local};
use chrono_lc::LocaleDate;
use walkdir::WalkDir;

use crate::cli::Args;

struct Config {
    pub dir: String,
    max_depth: Option<usize>,
    canonicalize: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dir: ".".to_string(),
            max_depth: None,
            canonicalize: false,
        }
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            dir: args.dir,
            max_depth: args.max_depth,
            canonicalize: args.canonicalize,
        }
    }
}

fn view_files(config: Option<Config>) {
    let config = config.unwrap_or_default();

    let depth = config.max_depth.unwrap_or(1);
    let canonicalize = config.canonicalize;

    let mut walker = WalkDir::new(config.dir);
    walker = walker.max_depth(depth);

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e.source().map(|c| c.to_string()).unwrap_or_default());
                continue;
            }
        };

        println!("{}", render_as_row(entry, canonicalize));
    }
}

fn get_file_name(entry: walkdir::DirEntry, canonicalize: bool) -> String {
    println!("{:?}", entry.path());
    let name = entry.path().file_name().unwrap().to_str().unwrap();
    let name = if canonicalize {
        entry
            .path()
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        name.to_string()
    };

    let icon = get_file_icon(entry);

    format!("{icon} {name}")
}

fn get_file_icon(entry: walkdir::DirEntry) -> &'static str {
    match (
        entry.path().is_dir(),
        entry.path_is_symlink(),
        entry.path().is_file(),
    ) {
        (true, _, _) => "", // Directory
        (_, true, _) => "", // Symlink
        (_, _, true) => "", // File
        _ => "",
    }
}

fn get_file_creation_date(entry: walkdir::DirEntry) -> Option<String> {
    let metadata = entry.path().metadata().ok()?;
    let system_time = metadata.created().ok()?;
    let datetime: DateTime<Local> = system_time.into();

    Some(datetime.formatl("%x %X", "").to_string())
}

fn get_file_permissions(entry: walkdir::DirEntry) -> Option<String> {
    let metadata = entry.path().metadata().ok()?;
    let mode = metadata.permissions().mode();

    // this somehow gets the last 9 bits
    // idk how because im a retard
    let perms = mode & 0o777;

    let to_rwx = |_, shift: u8| {
        // this is magic to me
        let bits = (perms >> shift) & 0o7;
        format!(
            "{}{}{}",
            if bits & 0o4 != 0 { 'r' } else { '-' },
            if bits & 0o2 != 0 { 'w' } else { '-' },
            if bits & 0o1 != 0 { 'x' } else { '-' }
        )
    };

    let owner = to_rwx(perms, 6);
    let group = to_rwx(perms, 3);
    let others = to_rwx(perms, 0);

    Some(format!("{}{}{}", owner, group, others))
}

fn get_file_size(entry: walkdir::DirEntry) -> Option<u64> {
    match entry.path().metadata() {
        Ok(metadata) => Some(metadata.len()),
        Err(_) => None,
    }
}

fn render_as_row(entry: walkdir::DirEntry, canonicalize: bool) -> String {
    let name = get_file_name(entry.clone(), canonicalize);
    let creation_date = get_file_creation_date(entry.clone()).unwrap_or_else(|| "-".to_string());
    let permissions = get_file_permissions(entry.clone()).unwrap_or_else(|| "-".to_string());
    let size = get_file_size(entry.clone())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "-".to_string());

    format!(
        "{:<20} {:<10} {:<10} {:<10}",
        name, creation_date, permissions, size
    )
}

fn main() {
    let cli = Args::parse();

    let config = Config::from(cli);

    view_files(Some(config));
}
