use std::{
    env::{self},
    path::{Path, PathBuf},
};

use crate::{Error, Result};

pub fn cd_into(path: &Path) -> Result<()> {
    env::set_current_dir(path).map_err(|_| Error::ChangeDir(path.into()))
}

pub fn cd_into_and_return_previous(path: &Path) -> Result<PathBuf> {
    // Save the folder we are currently in before changing it
    let previous_dir = env::current_dir().map_err(|_| Error::Cwd)?;

    // Set the current directory to be the one pointed to by `path`
    cd_into(path)?;

    // Return the folder we were in before
    Ok(previous_dir)
}
