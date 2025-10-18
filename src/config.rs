use crate::cli::Args;
use crate::string_ext;

use chrono::{DateTime, Local};
use chrono_lc::LocaleDate;
use colored::*;
use std::str::FromStr;
use std::time::SystemTime;
use std::{error::Error, os::unix::fs::PermissionsExt};
use string_ext::*;
use walkdir::WalkDir;

/// Configuration for viewing files
#[derive(Debug, Clone)]
pub struct Config {
    /// Directory to view
    pub dir: String,
    /// Maximum depth to traverse
    pub max_depth: Option<usize>,
    /// If true, show canonicalized paths (absolute paths)
    pub canonicalize: bool,
    /// If true, show hidden files (files starting with a dot)
    pub show_hidden: bool,
    /// If true, render output as a table
    pub table: bool,
    /// Unit for file sizes
    pub unit: Option<Unit>,
    pub reversed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dir: "./".to_string(),
            max_depth: None,
            canonicalize: false,
            show_hidden: false,
            table: false,
            unit: Some(Unit::Bytes),
            reversed: false,
        }
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            dir: args.dir,
            max_depth: args.max_depth,
            canonicalize: args.canonicalize,
            show_hidden: args.show_hidden,
            table: args.table,
            unit: args.unit,
            reversed: args.reversed,
        }
    }
}

/// File size units that we support
#[derive(Debug, Clone)]
pub enum Unit {
    Bytes,
    KB,
    MB,
    GB,
    TB,
}

impl FromStr for Unit {
    type Err = String;
    /// Parse a string into a Unit enum
    /// Supports: b, bytes, k, kb, kib, m, mb, mib, g, gb, gib, t, tb, tib
    /// Examples:
    /// "b" -> Unit::Bytes
    /// "kb" -> Unit::KB
    /// "invalid" -> Err("Invalid unit: invalid")
    /// "KB" -> Unit::KB
    /// "MiB" -> Unit::MB
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "b" | "bytes" => Ok(Unit::Bytes),
            "k" | "kb" | "kib" => Ok(Unit::KB),
            "m" | "mb" | "mib" => Ok(Unit::MB),
            "g" | "gb" | "gib" => Ok(Unit::GB),
            "t" | "tb" | "tib" => Ok(Unit::TB),
            _ => Err(format!("Invalid unit: {}", s)),
        }
    }
}

/// Normalize the unit to a short string representation
/// Examples:
/// Unit::Bytes -> "b"
/// Unit::KB -> "kib"
pub fn normalize_size_unit(unit: &Unit) -> &str {
    match unit {
        Unit::Bytes => "b",
        Unit::KB => "kib",
        Unit::MB => "mib",
        Unit::GB => "gib",
        Unit::TB => "tib",
    }
}

/// View files in a directory based on the provided configuration
/// If no configuration is provided, the default configuration is used
pub fn view_files(config: Option<Config>) {
    let config = config.unwrap_or_default();

    let depth = config.max_depth.unwrap_or(1);
    let canonicalize = config.canonicalize;

    let unit = config.unit.unwrap_or(Unit::Bytes);

    let walker = WalkDir::new(config.dir)
        .min_depth(1)
        .max_depth(depth)
        .sort_by(move |a, b| {
            let a_metadata = a.metadata().ok();
            let b_metadata = b.metadata().ok();

            let a_created = a_metadata
                .as_ref()
                .and_then(|m| m.created().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            let b_created = b_metadata
                .as_ref()
                .and_then(|m| m.created().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            if config.reversed {
                return b_created.cmp(&a_created);
            }

            a_created.cmp(&b_created)
        });

    let entries = walker
        .into_iter()
        .filter_entry(|e| config.show_hidden || !is_hidden(e));

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e.source().map(|c| c.to_string()).unwrap_or_default());
                continue;
            }
        };

        if config.table {
            let table_entries = vec![entry];
            let table = render_as_table(table_entries, canonicalize, &unit);
            println!("{}", table);
            continue;
        } else {
            println!("{}", render_as_row(entry, canonicalize, &unit));
        }
    }
}

/// Get the file name with an icon
/// If canonicalize is true, return the canonicalized path
/// Otherwise, return just the file name
fn get_file_name(entry: walkdir::DirEntry, canonicalize: bool) -> Result<String, Box<dyn Error>> {
    let name = entry.file_name();
    let name = match name.to_str() {
        Some(n) => n,
        None => {
            return Err("Failed to convert file name to string".into());
        }
    };

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

    Ok(format!("{icon} {name}"))
}

/// Get an icon based on the file type
/// Directory:  (blue)
/// Symlink:  (cyan)
/// File:  (green)
fn get_file_icon(entry: walkdir::DirEntry) -> &'static str {
    match (
        entry.path().is_dir(),
        entry.path_is_symlink(),
        entry.path().is_file(),
    ) {
        (true, _, _) => "\x1b[34m\x1b[0m", // Directory
        (_, true, _) => "\x1b[36m\x1b[0m", // Symlink
        (_, _, true) => "\x1b[32m\x1b[0m", // File
        _ => "",
    }
}

/// Check if a file is hidden (starts with a dot)
/// Examples:
/// .hidden -> true
/// visible -> false
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Get the file creation date as a formatted string
/// If the creation date cannot be determined, return None
/// Examples:
/// 2023-10-01 12:34:56 -> "10/01/23 12:34:56"
/// If creation date is not available -> None
fn get_file_creation_date(entry: walkdir::DirEntry) -> Option<String> {
    let metadata = entry.path().metadata().ok()?;
    let system_time = metadata.created().ok()?;
    let datetime: DateTime<Local> = system_time.into();

    Some(datetime.formatl("%x %X", "").to_string())
}

/// Get the file permissions as a rwx string
/// Examples:
/// rwxr-xr-x -> "rwxr-xr-x"
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

/// Get the file size in the specified unit
/// If the file size cannot be determined, return None
/// Examples:
/// 1024 bytes with Unit::KB -> "1 kib"
/// 1048576 bytes with Unit::MB -> "1 mib"
fn get_file_size(entry: walkdir::DirEntry, unit: &Unit) -> Option<String> {
    match entry.path().metadata() {
        Ok(metadata) => {
            let size_in_bytes = metadata.len();
            let size = match unit {
                Unit::Bytes => size_in_bytes,
                Unit::KB => size_in_bytes / 1024,
                Unit::MB => size_in_bytes / (1024 * 1024),
                Unit::GB => size_in_bytes / (1024 * 1024 * 1024),
                Unit::TB => size_in_bytes / (1024 * 1024 * 1024 * 1024),
            };
            Some(format!(
                "{} {}",
                size,
                normalize_size_unit(&unit).to_string()
            ))
        }
        Err(_) => None,
    }
}

/// Render a single file entry as a formatted row
fn render_as_row(entry: walkdir::DirEntry, canonicalize: bool, unit: &Unit) -> String {
    let name = get_file_name(entry.clone(), canonicalize).map_err(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let creation_date = get_file_creation_date(entry.clone()).unwrap_or_else(|| "-".to_string());

    let permissions = get_file_permissions(entry.clone()).unwrap_or_else(|| "-".to_string());

    let size = get_file_size(entry.clone(), unit)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "-".to_string());

    let name_width = 35;
    let date_width = 20;
    let perm_width = 12;
    let size_width = 10;

    let name = name.ok().map(|n| n.to_string());
    let name = name.as_deref().unwrap_or("-");

    format!(
        "{:<name_width$} {:<date_width$} {:<perm_width$} {:>size_width$}",
        &name.truncate_ellipsis(name_width - 1).bold(),
        &creation_date.truncate_ellipsis(date_width - 1),
        &permissions.truncate_ellipsis(perm_width - 1),
        size
    )
}

/// Render multiple file entries as a formatted table
/// Experimental function as this stinks a lil
fn render_as_table(entries: Vec<walkdir::DirEntry>, canonicalize: bool, unit: &Unit) -> String {
    let mut table = String::new();

    for entry in entries {
        let row = render_as_row(entry, canonicalize, unit);
        table.push_str(&row);
        table.push('\n');
    }

    table
}
