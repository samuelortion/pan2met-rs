//! Error definition of pan2met-rs

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),
}

/// Alias of result
pub type Result<T> = anyhow::Result<T>;
