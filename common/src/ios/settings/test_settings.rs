use std::mem::ManuallyDrop;
use tesseract_swift::utils::{string::CString, error::CError};
use crate::settings::TestSettings;

#[repr(C)]
pub struct CTestSettings {
    pub signature: CString,
    pub invalidator: CString
}

impl From<TestSettings> for CTestSettings {
    fn from(value: TestSettings) -> Self {
        CTestSettings {
            signature: value.signature.into(),
            invalidator: value.invalidator.into()
        }
    }
}

impl TryFrom<CTestSettings> for TestSettings {
    type Error = CError;

    fn try_from(value: CTestSettings) -> Result<Self, Self::Error> {
        Ok(TestSettings {
            signature: value.signature.try_into()?,
            invalidator: value.invalidator.try_into()?
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn wallet_test_settings_free(this: &mut ManuallyDrop<CTestSettings>) {
    drop(ManuallyDrop::take(this));
}
