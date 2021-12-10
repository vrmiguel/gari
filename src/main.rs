use std::{env, path::Path};

use fs_err as fs;

use gari::utils::{cd_into, cd_into_and_return_previous};

pub type Result<T> = anyhow::Result<T>;

const CLEANERS: &[&dyn Cleanable] = &[&CargoProject];


pub trait Cleanable {
    /// The paths that may be used to identify a (possibly different) folder of file that could be trashed.
    ///
    /// For example, Rust projects very often contain a `Cargo.toml` file, which is an indicator that a `target` folder
    /// in the same directory could be deleted in order to free up storage.
    // const INDICATORS: &'static [&'static str];
    // const FOLDERS_TO_CLEAN: &'static [&'static str];
    fn indicators(&self) -> &'static [&'static str];

    /// If the folder given by path [contains the indicator](Trashable::contains_indicator),
    /// [to_remove](Trashable::to_remove) indicates the file or folder that should/could be deleted
    fn to_remove(&self) -> &'static [&'static str];

    /// Checks if the folder given by `path` contains the [`indicator`](Cleanable::INDICATORS) folders or files.
    fn contains_indicators(&self, path: &Path) -> Result<bool> {
        let previous_dir = cd_into_and_return_previous(path)?;

        let contains_indicators = self.indicators().iter().all(|x| Path::new(x).exists());

        // Go back to the previous directory
        cd_into(&previous_dir)?;

        Ok(contains_indicators)
    }

    fn try_cleaning(&self, path: &Path) -> Result<()> {
        if self.contains_indicators(path)? {
            self.clean(path)?;
        }
        Ok(())
    }

    fn clean(&self, path: &Path) -> Result<()> {
        let previous_folder = cd_into_and_return_previous(path)?;

        for folder in self.to_remove().iter() {
            fs::remove_dir_all(folder)?;
        }

        cd_into(&previous_folder)?;
        Ok(())
    }
}

pub struct CargoProject;

impl Cleanable for CargoProject {
    fn to_remove(&self) -> &'static [&'static str] {
        &["target/"]
    }

    fn indicators(&self) -> &'static [&'static str] {
        &["Cargo.toml"]
    }
}

fn gari(path: &Path) -> Result<()> {
    if CargoProject.contains_indicators(path)? {
        CargoProject.clean(path)?;
    }
    Ok(())
}

// now commit, e aí tomorrow we faz as coisas hmm

fn main() -> Result<()> {
    let cleanables: &[&dyn Cleanable] = &[&CargoProject];

    let files = env::args_os().skip(1);
    for file in files {
        gari(file.as_ref())?;
    }
    Ok(())
}
