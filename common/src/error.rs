use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[cfg(target_os = "android")]
    #[error("JNI error")]
    JNI(#[from] jni::errors::Error),

    #[error("Lock poison error: {0}")]
    Poison(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("A string was returned as error: {0}")]
    Config(String),

    #[error("Unknown error")]
    Unknown,
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(value: std::sync::PoisonError<T>) -> Self {
        Self::Poison(value.to_string())
    }
}

pub (crate) type Result<T> = std::result::Result<T, Error>;