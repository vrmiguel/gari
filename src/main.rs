// TODO: add concept of folders it should never go into, e.g. `/Library` in macOS

mod cli;

use std::path::PathBuf;

use cli::parse_cli;
use gari::{Cleanable, Result};

use walkdir::{DirEntry, WalkDir};

const CLEANERS: &[&dyn Cleanable] = &[&CargoProject, &CMakeProject, &ZigProject];

pub struct CargoProject;
pub struct CMakeProject;
pub struct ZigProject;

impl Cleanable for CargoProject {
    fn to_remove(&self) -> &'static [&'static str] {
        &["target/"]
    }

    fn indicators(&self) -> &'static [&'static str] {
        &["Cargo.toml", "src/"]
    }

    fn context(&self) -> &'static str {
        "Cargo"
    }
}

impl Cleanable for CMakeProject {
    fn indicators(&self) -> &'static [&'static str] {
        &["CMakeLists.txt"]
    }

    fn to_remove(&self) -> &'static [&'static str] {
        &["build"]
    }

    fn context(&self) -> &'static str {
        "CMake"
    }
}

impl Cleanable for ZigProject {
    fn indicators(&self) -> &'static [&'static str] {
        &["build.zig", "src/"]
    }

    fn to_remove(&self) -> &'static [&'static str] {
        &["zig-out", "zig-cache"]
    }

    fn context(&self) -> &'static str {
        "Zig"
    }
}

fn clean(path: PathBuf, dry_run: bool) -> Result<()> {
    #[inline(always)]
    fn entry_is_directory(entry: &DirEntry) -> bool {
        entry.path().is_dir()
    }

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(entry_is_directory)
    {
        let entry = entry?;
        let path = entry.path();
        for cleaner in CLEANERS {
            cleaner.try_cleaning(path, dry_run)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = parse_cli();

    for file in cli.paths {
        clean(file, cli.check)?;
    }

    Ok(())
}
