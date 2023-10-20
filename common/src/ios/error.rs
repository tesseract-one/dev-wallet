use tesseract_swift_transports::error::TesseractSwiftError;
use tesseract_swift_utils::{panic::FromPanic, error::{CError, ErrorCode}};

/// cbindgen:add-sentinel
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum WalletErrorCode {
    Poison,
    IO,
    Config,
    Unknown
}

impl ErrorCode for WalletErrorCode {
    const FIRST: u32 = WalletErrorCode::Poison as u32;
    #[allow(non_upper_case_globals)]
    const Sentinel: u32 = WalletErrorCode::Unknown as u32 + 1;
}

impl From<CError> for crate::error::Error {
    fn from(value: CError) -> Self {
        Self::IOS(value.into())
    }
}

impl From<crate::error::Error> for CError {
    fn from(value: crate::error::Error) -> Self {
        let tesseract: TesseractSwiftError = value.into();
        tesseract.into()
    }
}

impl From<crate::error::Error> for TesseractSwiftError {
    fn from(value: crate::error::Error) -> Self {
        match value {
            crate::error::Error::IOS(err) => err,
            crate::error::Error::LoggerInit(logger) =>
                TesseractSwiftError::Logger(logger.to_string().into()),
            crate::error::Error::Config(message) =>
                TesseractSwiftError::Custom(WalletErrorCode::Config as u32, message.into()),
            crate::error::Error::Poison(message) => 
                TesseractSwiftError::Custom(WalletErrorCode::Poison as u32, message.into()),
            crate::error::Error::IO(err) =>
                TesseractSwiftError::Custom(WalletErrorCode::IO as u32, err.to_string().into()),
            crate::error::Error::Unknown => 
                TesseractSwiftError::Custom(WalletErrorCode::Unknown as u32, "".into()),
        }
    }
}

impl From<tesseract::Error> for crate::error::Error {
    fn from(value: tesseract::Error) -> Self {
        let ctes: TesseractSwiftError = value.into();
        ctes.into()
    }
}

impl FromPanic for crate::error::Error {
    fn from_panic(panic: &str) -> Self {
        CError::from_panic(panic).into()
    }
}
