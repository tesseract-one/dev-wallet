use std::mem::ManuallyDrop;

use super::test_sign::CTestSign;
use super::test_error::CTestError;
use super::substrate_account::CSubstrateAccount;
use super::substrate_sign::CSubstrateSign;

#[repr(C)]
pub enum CRequest {
    TestSign(CTestSign),
    TestError(CTestError),
    SubstrateAccount(CSubstrateAccount),
    SubstrateSign(CSubstrateSign)
}

#[no_mangle]
pub unsafe extern "C" fn wallet_crequest_free(this: &mut ManuallyDrop<CRequest>) {
    drop(ManuallyDrop::take(this));
}