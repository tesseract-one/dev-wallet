use std::mem::ManuallyDrop;

use tesseract_utils::{ptr::CAnyDropPtr, string::CString, string::CStringRef, future_impls::CFutureBool};

#[repr(C)]
pub struct SUI {
    ptr: CAnyDropPtr,
    request_user_confirmation: unsafe extern "C" fn(&SUI, CStringRef) -> ManuallyDrop<CFutureBool>,
}