use std::mem::ManuallyDrop;

use tesseract_swift_utils::string::CString;

use crate::request::SubstrateAccount;

use super::{IntoCRequest, CRequest};

#[repr(C)]
pub struct CSubstrateAccount {
    pub algorithm: CString,
    pub path: CString,
    pub key: CString,
}

impl From<SubstrateAccount> for CSubstrateAccount {
    fn from(value: SubstrateAccount) -> Self {
        CSubstrateAccount {
            algorithm: value.algorithm.into(),
            path: value.path.into(),
            key: value.key.into(),
        }
    }
}

impl IntoCRequest for SubstrateAccount {
    fn into_crequest(self) -> CRequest {
        CRequest::SubstrateAccount(self.into())
    }
}

#[no_mangle]
pub unsafe extern "C" fn wallet_substrate_account_free(this: &mut ManuallyDrop<CSubstrateAccount>) {
    drop(ManuallyDrop::take(this));
}
