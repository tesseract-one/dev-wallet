use std::mem::ManuallyDrop;
use tesseract_swift::utils::{string::CString, error::CError};

use crate::settings::KeySettings;

#[repr(C)]
pub struct CKeySettings {
    pub mnemonic: CString,
}

impl From<KeySettings> for CKeySettings {
    fn from(value: KeySettings) -> Self {
        CKeySettings {
            mnemonic: value.mnemonic.unwrap_or_default().into(),
        }
    }
}

impl TryFrom<CKeySettings> for KeySettings {
    type Error = CError;

    fn try_from(value: CKeySettings) -> Result<Self, Self::Error> {
        Ok(KeySettings {
            mnemonic: Some(value.mnemonic.try_into()?),
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn wallet_key_settings_free(this: &mut ManuallyDrop<CKeySettings>) {
    drop(ManuallyDrop::take(this));
}
