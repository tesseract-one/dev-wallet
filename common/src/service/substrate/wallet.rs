use std::str::FromStr;

use subxt::{
    ext::{
        sp_core::{
            crypto::SecretUri,
            sr25519::{self, Public},
            Encode, Pair,
        },
        sp_runtime::MultiSignature,
    },
    tx::{PairSigner, Signer},
    PolkadotConfig,
};

use super::error::{Result, Error, UnsupportedAccountType};

pub struct Wallet {
    pair: sr25519::Pair,
    signer: PairSigner<PolkadotConfig, sr25519::Pair>,
}

impl Wallet {
    pub (crate) fn new(phrase: &str) -> Result<Self> {
        let (pair, _) = sr25519::Pair::from_phrase(phrase, None)?;
        let signer = PairSigner::new(pair.clone());
        Ok(Self { pair, signer })
    }

    pub (crate) fn derive(&self, path: &str) -> Result<Public> {
        let uri = SecretUri::from_str(path)?;
        let path = uri.junctions.into_iter();
        let (pair, _) = self
            .pair
            .derive(path, None)
            .map_err(|_| {Error::Infolliable})?; //Who knows what is Infolliable?

        Ok(pair.public())
    }

    pub (crate) fn sign(&self, transaction: &[u8]) -> Result<Vec<u8>> {
        let multi_signature = self.signer.sign(transaction);
        match multi_signature {
            MultiSignature::Sr25519(signature) => Ok(signature.encode()),
            MultiSignature::Ed25519(_) => Err(Error::UnsupportedAccountType(UnsupportedAccountType::Ed25519)),
            MultiSignature::Ecdsa(_) => Err(Error::UnsupportedAccountType(UnsupportedAccountType::Ecdsa)),
        }
    }
}
