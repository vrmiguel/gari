mod cleanable;

pub use cleanable::Cleanable;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read current directory.")]
    Cwd,
    #[error("{0}")]
    FileSystem(#[from] std::io::Error),
    #[error("{0}")]
    Walkdir(#[from] walkdir::Error),
}
