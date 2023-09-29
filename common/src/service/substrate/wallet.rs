use std::str::FromStr;

use subxt::{config::PolkadotConfig, tx::Signer, utils::MultiSignature};
use subxt_signer::{SecretUri, sr25519::{self, PublicKey}, bip39::Mnemonic};
use super::error::{Error, Result, UnsupportedAccountType};

pub struct Wallet {
    pair: sr25519::Keypair
}

impl Wallet {
    pub (super) fn new(phrase: &str) -> Result<Self> {
        let mnemonic = Mnemonic::parse(phrase)?;
        let pair = sr25519::Keypair::from_phrase(&mnemonic, None)?;
        Ok(Self { pair })
    }

    pub (super) fn derive(&self, path: &str) -> Result<PublicKey> {
        let uri = SecretUri::from_str(path)?;
        let path = uri.junctions.into_iter();
        let pair= self
            .pair
            .derive(path);

        Ok(pair.public_key())
    }

    pub (super) fn sign(&self, transaction: &[u8]) -> Result<Vec<u8>> {
        let signer: &dyn Signer<PolkadotConfig> = &self.pair; 
        let multi_signature = signer.sign(transaction);
        match multi_signature {
            MultiSignature::Sr25519(signature) => Ok(signature.into()),
            MultiSignature::Ed25519(_) => Err(Error::UnsupportedAccountType(UnsupportedAccountType::Ed25519)),
            MultiSignature::Ecdsa(_) => Err(Error::UnsupportedAccountType(UnsupportedAccountType::Ecdsa)),
        }
    }
}
