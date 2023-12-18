use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[cfg(target_os = "android")]
    #[error("Android error")]
    Android(#[from] tesseract_android::error::TesseractAndroidError),

    #[cfg(target_os = "ios")]
    #[error("Logger initialization error")]
    LoggerInit(#[from] log::SetLoggerError),

    #[cfg(target_os = "ios")]
    #[error("IOS error {0}")]
    IOS(#[from] tesseract_swift_transports::error::TesseractSwiftError),

    #[error("Lock poison error: {0}")]
    Poison(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("A string was returned as error: {0}")]
    Config(String),

    #[allow(dead_code)]
    #[error("Unknown error")]
    Unknown,
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(value: std::sync::PoisonError<T>) -> Self {
        Self::Poison(value.to_string())
    }
}

pub (crate) type Result<T> = std::result::Result<T, Error>;

impl Into<tesseract_one::Error> for Error {
    fn into(self) -> tesseract_one::Error {
        match self {
            #[cfg(target_os = "android")]
            Error::Android(e) => e.into(),
            #[cfg(target_os = "ios")]
            Error::IOS(e) => e.into(),
            Error::IO(e) => {
                let description = format!("IOError: {}", e);
                tesseract_one::Error::described(tesseract_one::ErrorKind::Weird, &description)
            },
            e => {
                let description = format!("Wallet error: {}", e);
                tesseract_one::Error::described(tesseract_one::ErrorKind::Weird, &description)
            }
        }
    }
}
