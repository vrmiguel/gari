use std::{
    env,
    path::Path,
};

use fs_err as fs;

use gari::utils::{cd_into, cd_into_and_return_previous};

pub type Result<T> = anyhow::Result<T>;

pub trait Cleanable {
    /// The paths that may be used to identify a (possibly different) folder of file that could be trashed.
    ///
    /// For example, Rust projects very often contain a `Cargo.toml` file, which is an indicator that a `target` folder
    /// in the same directory could be deleted in order to free up storage.
    const INDICATORS: &'static [&'static str];
    const FOLDERS_TO_CLEAN: &'static [&'static str];

    /// Checks if the folder given by `path` contains the [`indicator`](Cleanable::indicator) folder or file.
    fn contains_indicators(path: &Path) -> Result<bool> {
        let previous_dir = cd_into_and_return_previous(path)?;

        let contains_indicators = Self::INDICATORS.iter().all(|x| Path::new(x).exists());

        // Go back to the previous directory
        cd_into(&previous_dir)?;

        Ok(contains_indicators)
    }

    fn clean_function(path: &Path) -> Result<()> {
        let previous_folder = cd_into_and_return_previous(path)?;

        for folder in Self::FOLDERS_TO_CLEAN.iter() {
            fs::remove_dir_all(folder)?;
        }

        cd_into(&previous_folder)?;
        Ok(())
    }
}

pub struct CargoProject;

impl Cleanable for CargoProject {
    const INDICATORS: &'static [&'static str] = &["Cargo.toml"];
    const FOLDERS_TO_CLEAN: &'static [&'static str] = &["target/"];
}

fn gari(path: &Path) -> Result<()> {
    if CargoProject::contains_indicators(path)? {
        CargoProject::clean_function(path)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let files = env::args_os().skip(1);
    for file in files {
        gari(Path::new(file.as_os_str()))?;
    }
    Ok(())
}
