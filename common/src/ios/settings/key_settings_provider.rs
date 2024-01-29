use std::mem::ManuallyDrop;
use std::sync::Arc;

use tesseract_swift::utils::error::CError;
use tesseract_swift::utils::panic::PanicContext;

use crate::error::Error;
use crate::settings::{SettingsProvider, KeySettingsProvider};

use super::key_settings::CKeySettings;
use super::provider::CSettingsProvider;

#[no_mangle]
pub unsafe extern "C" fn wallet_key_settings_provider_load(
    provider: ManuallyDrop<CSettingsProvider>,
    ret: &mut ManuallyDrop<CKeySettings>, err: &mut ManuallyDrop<CError>
) -> bool {
    use tesseract_swift::utils::response::CMoveResponse;
    
    Error::panic_context(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        let settings = provider.load_key_settings()?;

        Ok(settings)
    }).response(ret, err)
}

#[no_mangle]
pub unsafe extern "C" fn wallet_key_settings_provider_save(
    provider: ManuallyDrop<CSettingsProvider>,
    value: ManuallyDrop<CKeySettings>, err: &mut ManuallyDrop<CError>
) -> bool {
    use tesseract_swift::utils::response::CVoidResponse;

    Error::panic_context(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        provider.save_key_settings(ManuallyDrop::into_inner(value).try_into()?)?;
        Ok(())
    }).response(err)
}