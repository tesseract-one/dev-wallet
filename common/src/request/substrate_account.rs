use super::Request;

pub (crate) struct SubstrateAccount {
    pub algorithm: String,
    pub path: String,
    pub key: String,
}

impl Request for SubstrateAccount {
}