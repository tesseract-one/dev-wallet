use std::mem::ManuallyDrop;

use tesseract_utils::string::CString;

use crate::request::SubstrateSign;

use super::{IntoCRequest, CRequest};

#[repr(C)]
pub struct CSubstrateSign {
    pub algorithm: CString,
    pub path: CString,
    pub key: CString,
    pub data: CString,
}

impl From<SubstrateSign> for CSubstrateSign {
    fn from(value: SubstrateSign) -> Self {
        CSubstrateSign {
            algorithm: value.algorithm.into(),
            path: value.path.into(),
            key: value.key.into(),
            data: value.data.into(),
        }
    }
}

impl IntoCRequest for SubstrateSign {
    fn into_crequest(self) -> CRequest {
        CRequest::SubstrateSign(self.into())
    }
}

#[no_mangle]
pub unsafe extern "C" fn wallet_substrate_sign_free(this: &mut ManuallyDrop<CSubstrateSign>) {
    drop(ManuallyDrop::take(this));
}
