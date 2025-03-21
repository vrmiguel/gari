use std::ops::Not;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use byte_unit::Byte;
use fs_err as fs;
use walkdir::WalkDir;

use crate::Result;

pub fn get_directory_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;

    if path.is_dir() {
        for entry in WalkDir::new(path).into_iter() {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() || metadata.is_symlink() {
                total_size += metadata.size()
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
        let mut indicators = self.indicators().iter().chain(self.to_remove().iter());

        let contains_indicators = indicators.all(|indicator| path.join(indicator).exists());

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
                Byte::from_u64(directory_size).get_appropriate_unit(byte_unit::UnitType::Decimal)
            );

            if dry_run.not() {
                self.clean(path)?;
            }
        }

        Ok(())
    }

    /// Removes the folders or files given by [`to_remove`](Cleanable::to_remove).
    fn clean(&self, path: &Path) -> Result<()> {
        for path_to_trash in self.to_remove() {
            let path_to_trash = path.join(path_to_trash);
            println!("Attempting to remove {}", path_to_trash.display());
            fs::remove_dir_all(path_to_trash)?;
        }

        Ok(())
    }
}
