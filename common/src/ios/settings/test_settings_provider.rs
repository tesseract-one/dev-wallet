use std::mem::ManuallyDrop;
use std::sync::Arc;

use tesseract_utils::{response::CResponse, panic::handle_exception_result, error::CError};

use crate::settings::{TestSettings, SettingsProvider, TestSettingsProvider};

use super::test_settings::CTestSettings;
use super::provider::CSettingsProvider;

#[no_mangle]
pub unsafe extern "C" fn wallet_test_settings_provider_load(provider: ManuallyDrop<CSettingsProvider>, value: &mut ManuallyDrop<CTestSettings>, error: &mut ManuallyDrop<CError>) -> bool {
    handle_exception_result(|| {
        let provider = provider.as_ref::<Arc<SettingsProvider>>()?;
        let settings = provider.load_test_settings().map_err(|err| Into::<CError>::into(err))?;

        Ok(settings.into())
    }).response(value, error)
}