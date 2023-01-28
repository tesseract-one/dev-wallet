use tesseract_utils::error::CError::ErrorCode;

#[repr(u32)]
enum DWErrorCodes {
    LoggerInit = 0x1000,
    Poison,
    IO,
    Config,
    Unknown
}

impl Into<tesseract_utils::error::CError> for crate::error::Error  {
    fn into(self) -> tesseract_utils::error::CError {
        match self {
            crate::error::Error::CError(cerror) => cerror,
            crate::error::Error::LoggerInit(logger_error) => ErrorCode(DWErrorCodes::LoggerInit as u32, logger_error.to_string().into()),
            crate::error::Error::Poison(message) => ErrorCode(DWErrorCodes::Poison as u32, message.into()),
            crate::error::Error::IO(io_error) => ErrorCode(DWErrorCodes::IO as u32, io_error.to_string().into()),
            crate::error::Error::Config(message) => ErrorCode(DWErrorCodes::Config as u32, message.into()),
            crate::error::Error::Unknown => ErrorCode(DWErrorCodes::Unknown as u32, "No one knows".into()),
        }
    }
}