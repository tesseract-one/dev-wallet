use std::mem::ManuallyDrop;

use tesseract_utils::string::CString;

use crate::request::TestError;

use super::{IntoCRequest, CRequest};

#[repr(C)]
pub struct CTestError {
    pub transaction: CString,
    pub error: CString,
}

impl From<TestError> for CTestError {
    fn from(value: TestError) -> Self {
        CTestError {
            transaction: value.transaction.into(),
            error: value.error.into(),
        }
    }
}

impl IntoCRequest for TestError {
    fn into_crequest(self) -> CRequest {
        CRequest::TestError(self.into())
    }
}

#[no_mangle]
pub unsafe extern "C" fn wallet_test_error_free(this: &mut ManuallyDrop<CTestError>) {
    drop(ManuallyDrop::take(this));
}