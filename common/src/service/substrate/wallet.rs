use std::{error::Error, str::FromStr};

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

pub struct Wallet {
    pair: sr25519::Pair,
    signer: PairSigner<PolkadotConfig, sr25519::Pair>,
}

impl Wallet {
    pub fn new(phrase: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (pair, _) = sr25519::Pair::from_phrase(phrase, None).map_err(|e| format!("{:?}", e))?;
        let signer = PairSigner::new(pair.clone());
        Ok(Self { pair, signer })
    }

    pub fn derive(&self, path: &str) -> Result<Public, Box<dyn Error + Send + Sync>> {
        let uri = SecretUri::from_str(path).map_err(|e| format!("{:?}", e))?;
        let path = uri.junctions.into_iter();
        let (pair, _) = self
            .pair
            .derive(path, None)
            .map_err(|e| format!("{:?}", e))?;
        Ok(pair.public())
    }

    pub fn sign(&self, transaction: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let multi_signature = self.signer.sign(transaction);
        match multi_signature {
            MultiSignature::Sr25519(signature) => Ok(signature.encode()),
            _ => Err("Should be Sr25519 signature".into()),
        }
    }
}
