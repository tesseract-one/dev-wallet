use super::Request;

pub (crate) struct TestError {
    pub transaction: String,
    pub error: String
}

impl Request for TestError {
}