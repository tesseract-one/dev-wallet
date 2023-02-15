use std::mem::ManuallyDrop;
use std::sync::Arc;

use tesseract_utils::{response::CResponse, panic::handle_exception_result, error::CError};

use crate::settings::{SettingsProvider, KeySettingsProvider};

use super::key_settings::CKeySettings;
use super::provider::CSettingsProvider;

#[no_mangle]
pub unsafe extern "C" fn wallet_key_settings_provider_load(provider: ManuallyDrop<CSettingsProvider>, ret: &mut ManuallyDrop<CKeySettings>, err: &mut ManuallyDrop<CError>) -> bool {
    handle_exception_result(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        let settings = provider.load_key_settings().map_err(|err| Into::<CError>::into(err))?;

        Ok(settings.into())
    }).response(ret, err)
}

#[no_mangle]
pub unsafe extern "C" fn wallet_key_settings_provider_save(provider: ManuallyDrop<CSettingsProvider>, value: ManuallyDrop<CKeySettings>, err: &mut ManuallyDrop<CError>) -> bool {
    handle_exception_result(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        provider.save_key_settings(ManuallyDrop::into_inner(value).try_into()?)
            .map_err(|err| Into::<CError>::into(err))
    }).response((), err)
}