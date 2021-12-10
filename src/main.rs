use std::{env, ffi::OsString};

use gari::Cleanable;
use walkdir::{DirEntry, WalkDir};

pub type Result<T> = anyhow::Result<T>;

const CLEANERS: &[&dyn Cleanable] = &[&CargoProject];

pub struct CargoProject;

impl Cleanable for CargoProject {
    fn to_remove(&self) -> &'static [&'static str] {
        &["target/"]
    }

    fn indicators(&self) -> &'static [&'static str] {
        &["Cargo.toml"]
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
