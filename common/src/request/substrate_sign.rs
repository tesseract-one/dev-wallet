use super::Request;

pub (crate) struct SubstrateSign {
    pub algorithm: String,
    pub path: String,
    pub key: String,
    pub data: String,
}

impl Request for SubstrateSign {
}