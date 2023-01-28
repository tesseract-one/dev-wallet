use std::mem::ManuallyDrop;

use tesseract_utils::{ptr::CAnyDropPtr, string::CString, string::CStringRef, future_impls::CFutureBool};

use crate::error::Result as DWResult;

#[repr(C)]
pub struct SUI {
    ptr: CAnyDropPtr,
    request_user_confirmation: unsafe extern "C" fn(&SUI, CStringRef) -> ManuallyDrop<CFutureBool>,
}

impl SUI {
    pub (crate) async fn request_user_confirmation(&self, tx: &str) -> DWResult<bool> {
        let cstr: CString = tx.into();

        let man_drop = unsafe {
            (self.request_user_confirmation)(self, cstr.as_ptr())
        };

        let future = ManuallyDrop::into_inner(man_drop);
        
        Ok(future.try_into_future()?.await?)
    }
}