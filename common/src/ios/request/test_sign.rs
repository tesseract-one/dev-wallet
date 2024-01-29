use std::mem::ManuallyDrop;

use tesseract_swift::utils::string::CString;

use crate::request::TestSign;

use super::{IntoCRequest, CRequest};

#[repr(C)]
pub struct CTestSign {
    pub transaction: CString,
    pub signature: CString,
    pub result: CString,
}

impl From<TestSign> for CTestSign {
    fn from(value: TestSign) -> Self {
        CTestSign {
            transaction: value.transaction.into(),
            signature: value.signature.into(),
            result: value.result.into(),
        }
    }
}

impl IntoCRequest for TestSign {
    fn into_crequest(self) -> CRequest {
        CRequest::TestSign(self.into())
    }
}

#[no_mangle]
pub unsafe extern "C" fn wallet_test_sign_free(this: &mut ManuallyDrop<CTestSign>) {
    drop(ManuallyDrop::take(this));
}
