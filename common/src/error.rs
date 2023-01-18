use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[cfg(target_os = "android")]
    #[error("JNI error")]
    JNI(#[from] jni::errors::Error),
    #[error("Unknown error")]
    Unknown,
}

pub (crate) type Result<T> = std::result::Result<T, Error>;