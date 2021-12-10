mod cleanable;
pub mod utils;

pub use cleanable::Cleanable;

pub type Result<T> = anyhow::Result<T>;
