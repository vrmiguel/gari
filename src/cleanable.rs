use std::ops::Not;
use std::path::Path;

use byte_unit::Byte;
use fs_err as fs;

use crate::utils::{cd_into, cd_into_and_return_previous};
use crate::Result;

pub fn get_directory_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                total_size += get_directory_size(&path)?;
            } else {
                total_size += entry.metadata()?.len();
            }
        }
    }

    Ok(total_size)
}

pub trait Cleanable {
    /// The paths that may be used to identify a (possibly different) folder of file that could be trashed.
    ///
    /// For example, Rust projects very often contain a `Cargo.toml` file, which is an indicator that a `target` folder
    /// in the same directory could be deleted in order to free up storage.
    fn indicators(&self) -> &'static [&'static str];

    /// If the folder given by path [contains the indicator](Cleanable::contains_indicators),
    /// [to_remove](Cleanable::to_remove) indicates the file or folder that should/could be deleted
    fn to_remove(&self) -> &'static [&'static str];

    /// The context for what we're trashing (e.g. indicates if we're trashing a Rust project, a TypeScript project, and so on).
    fn context(&self) -> &'static str;

    /// Checks if the folder given by `path` contains the [`indicator`](Cleanable::indicators) folders or files.
    fn contains_indicators(&self, path: &Path) -> Result<bool> {
        let previous_dir = cd_into_and_return_previous(path)?;

        let mut files = self.indicators().iter().chain(self.to_remove().iter());

        let contains_indicators = files.all(|x| Path::new(x).exists());

        // Go back to the previous directory
        cd_into(&previous_dir)?;

        Ok(contains_indicators)
    }

    /// Checks if the folder given by `path` [contains](Cleanable::contains_indicators)
    /// the [indicators](Cleanable::indicators), if so, removes the files given by [`to_remove`](Cleanable::to_remove).
    fn try_cleaning(&self, path: &Path, dry_run: bool) -> Result<()> {
        if self.contains_indicators(path)? {
            let directory_size = get_directory_size(path)?;
            println!(
                "{} project found in {} ({})",
                self.context(),
                path.display(),
                Byte::from_u64(directory_size)
            );

            if dry_run.not() {
                self.clean(path)?;
            }
        }

        Ok(())
    }

    /// Removes the folders or files given by [`to_remove`](Cleanable::to_remove).
    fn clean(&self, path: &Path) -> Result<()> {
        let previous_folder = cd_into_and_return_previous(path)?;

        for path_to_trash in self.to_remove() {
            println!("Attempting to remove {}/{}", path.display(), path_to_trash);
            fs::remove_dir_all(path_to_trash)?;
        }

        cd_into(&previous_folder)?;
        Ok(())
    }
}
