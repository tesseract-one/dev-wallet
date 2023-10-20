use std::mem::ManuallyDrop;

use tesseract_swift_utils::{ptr::CAnyDropPtr, future_impls::CFutureBool};

use crate::{error::Result, request::Request};

use super::super::request::CRequest;

#[repr(C)]
pub struct SUI {
    ptr: CAnyDropPtr,
    request_user_confirmation: unsafe extern "C" fn(&SUI, ManuallyDrop<CRequest>) -> ManuallyDrop<CFutureBool>,
}

impl SUI {
    pub (crate) async fn request_user_confirmation<R: Request>(&self, tx: R) -> Result<bool> {
        let crequest = ManuallyDrop::new(tx.into_crequest());

        let man_drop = unsafe {
            (self.request_user_confirmation)(self, crequest)
        };

        let future = ManuallyDrop::into_inner(man_drop);

        Ok(future.try_into_future()?.await?)
    }
}