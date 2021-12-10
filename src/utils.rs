use std::{
    env::{self},
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::Result;

pub fn cd_into(path: &Path) -> Result<()> {
    env::set_current_dir(path)
        .with_context(|| format!("Failed to cd into '{}'", path.display()))?;

    Ok(())
}

pub fn cd_into_and_return_previous(path: &Path) -> Result<PathBuf> {
    // Save the folder we are currently in before changing it
    let previous_dir = env::current_dir().with_context(|| "Failed to read current directory.")?;

    // Set the current directory to be the one pointed to by `path`
    cd_into(path)?;

    // Return the folder we were in before
    Ok(previous_dir)
}
