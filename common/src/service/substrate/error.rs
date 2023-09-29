use thiserror::Error;

use subxt::ext::scale_decode::Error as DecodeError;

#[derive(Debug)]
pub (super) enum UnsupportedAccountType {
    Ed25519,
    Ecdsa
}

#[derive(Error, Debug)]
pub (super) enum Error {
    #[error("User cancelled request")]
    Cancelled,

    #[error(transparent)]
    WalletError(#[from] crate::error::Error),

    #[error("Failed to parse derivation path: {0:?}")]
    SecretUriError(#[from] subxt_signer::SecretUriError),

    #[error("Failed to parse mnemonic: {0:?}")]
    Bip39Error(#[from] subxt_signer::bip39::Error),

    #[error("Please, set your private key in the wallet settings")]
    MnemonicNotSet,

    #[error("ParityScaleCodec error: {0}")]
    ParityScaleCodec(#[from] subxt::ext::codec::Error),

    #[error("sr25519 error: {0}")]
    Sr25519Error(#[from] subxt_signer::sr25519::Error),

    #[error("Substrate DecodeError error: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("Error converting parsed Substrate data to JSON: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Unsupported account type (should be implemented in the future): {0:?}")]
    UnsupportedAccountType(UnsupportedAccountType),
}

impl From<subxt::ext::scale_decode::visitor::DecodeError> for Error {
    fn from(value: subxt::ext::scale_decode::visitor::DecodeError) -> Self {
        DecodeError::from(value).into()
    }
}

pub (super) type Result<T> = std::result::Result<T, Error>;

impl Into<tesseract::Error> for Error {
    fn into(self) -> tesseract::Error {
        match self {
            Self::Cancelled => tesseract::Error::kinded(tesseract::ErrorKind::Cancelled),
            other => tesseract::Error::nested(Box::new(other))
        }
    }
}