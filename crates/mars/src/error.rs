//! Error type for this crate.

/// Everything this crate can fail with.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A name was empty or contained only whitespace.
    #[error("name must not be blank")]
    BlankName,

    /// A name exceeded [`crate::Greeting::MAX_NAME_LEN`] bytes.
    #[error("name is {len} bytes, limit is {max}")]
    NameTooLong {
        /// Length of the offending name, in bytes.
        len: usize,
        /// The maximum allowed length, in bytes.
        max: usize,
    },
}

/// Result alias pinned to this crate's [`Error`].
pub type Result<T> = core::result::Result<T, Error>;
