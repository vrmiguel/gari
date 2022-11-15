use std::{env, ffi::OsString};

use gari::Cleanable;
use gari::Result;

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

fn clean(path: OsString) -> Result<()> {
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
            cleaner.try_cleaning(path)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    for file in env::args_os().skip(1) {
        clean(file)?;
    }

    Ok(())
}
