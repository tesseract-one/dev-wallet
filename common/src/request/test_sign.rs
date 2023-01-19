use super::Request;

pub (crate) struct TestSign {
    pub transaction: String,
    pub signature: String,
    pub result: String,
}

impl Request for TestSign {
}