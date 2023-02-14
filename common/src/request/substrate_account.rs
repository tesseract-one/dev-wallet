use super::Request;

pub (crate) struct SubstrateAccount {
    pub algorythm: String,
    pub path: String,
    pub key: String,
}

impl Request for SubstrateAccount {
}