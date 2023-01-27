use std::mem::ManuallyDrop;
use std::sync::Arc;

use tesseract_utils::Nothing;
use tesseract_utils::{response::CResponse, panic::handle_exception_result, error::CError};

use crate::settings::{SettingsProvider, TestSettingsProvider};

use super::test_settings::CTestSettings;
use super::provider::CSettingsProvider;

#[no_mangle]
pub unsafe extern "C" fn wallet_test_settings_provider_load(provider: ManuallyDrop<CSettingsProvider>, ret: &mut ManuallyDrop<CTestSettings>, err: &mut ManuallyDrop<CError>) -> bool {
    handle_exception_result(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        let settings = provider.load_test_settings().map_err(|err| Into::<CError>::into(err))?;

        Ok(settings.into())
    }).response(ret, err)
}

//TODO: make Nothing=>CType and fix
#[no_mangle]
pub unsafe extern "C" fn wallet_test_settings_provider_save(provider: ManuallyDrop<CSettingsProvider>, value: ManuallyDrop<CTestSettings>, ret: &mut ManuallyDrop<bool>, err: &mut ManuallyDrop<CError>) -> bool {
    handle_exception_result(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        provider.save_test_settings(ManuallyDrop::into_inner(value).try_into()?)
            .map_err(|err| Into::<CError>::into(err))
            .map(|_| true)
    }).response(ret, err)
}