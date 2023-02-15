use thiserror::Error;

use subxt::{error::SecretStringError, error::DecodeError};

#[derive(Debug)]
pub (crate) enum UnsupportedAccountType {
    Ed25519,
    Ecdsa
}

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[error("User cancelled request")]
    Cancelled,

    #[error(transparent)]
    WalletError(#[from] crate::error::Error),

    #[error("Failed to parse mnemonic: {0:?}")]
    SecretStringError(SecretStringError),

    #[error("Please, set your private key in the wallet settings")]
    MnemonicNotSet,

    #[error("ParityScaleCodec error: {0}")]
    ParityScaleCodec(#[from] subxt::ext::codec::Error),

    #[error("Substrate DecodeError error: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("Error converting parsed Substrate data to JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Infolliable")]
    Infolliable,

    #[error("Unsupported account type (should be implemented in the future): {0:?}")]
    UnsupportedAccountType(UnsupportedAccountType),
}

impl From<SecretStringError> for Error {
    fn from(value: SecretStringError) -> Self {
        Self::SecretStringError(value)
    }
}

pub (crate) type Result<T> = std::result::Result<T, Error>;

impl Into<tesseract::Error> for Error {
    fn into(self) -> tesseract::Error {
        match self {
            Self::Cancelled => tesseract::Error::kinded(tesseract::ErrorKind::Cancelled),
            other => tesseract::Error::nested(Box::new(other))
        }
    }
}