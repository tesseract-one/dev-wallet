use std::mem::ManuallyDrop;

use tesseract_swift_utils::{
    error::CError, panic::PanicContext, ptr::{CAnyRustPtr, IntoAnyPtr},
    string::CStringRef, traits::TryAsRef, response::CMoveResponse
};

use crate::Core;
use super::{ui::SUI, settings::CSettingsProvider};

pub type CCore = CAnyRustPtr;

impl IntoAnyPtr for Core {}

#[no_mangle]
pub unsafe extern "C" fn wallet_ccore_create_app(
    ui: SUI, data_dir: CStringRef,
    ret: &mut ManuallyDrop<CCore>, err: &mut ManuallyDrop<CError>
) -> bool {
    crate::error::Error::panic_context(|| {
        super::logger::init()?;

        let data_dir = data_dir.try_as_ref()?;

        let core = Core::new(super::UI::new(ui), data_dir, |tesseract| tesseract);

        Ok(core)
    }).response(ret, err)
}

#[no_mangle]
pub unsafe extern "C" fn wallet_ccore_create_extension(
    ui: SUI, data_dir: CStringRef,
    transport: tesseract_swift_transports::service::ServiceTransport,
    ret: &mut ManuallyDrop<CCore>, err: &mut ManuallyDrop<CError>
) -> bool {
    crate::error::Error::panic_context(|| {
        super::logger::init()?;

        let data_dir = data_dir.try_as_ref()?;

        let core = Core::new(super::UI::new(ui), data_dir, |tesseract| {
            tesseract.transport(transport)
        });

        Ok(core)
    }).response(ret, err)
}

#[no_mangle]
pub unsafe extern "C" fn wallet_ccore_test_settings_provider(
    ccore: ManuallyDrop<CCore>,
    value: &mut ManuallyDrop<CSettingsProvider>, error: &mut ManuallyDrop<CError>
) -> bool {
    crate::error::Error::panic_context(|| {
        let core = ccore.as_ref::<Core>()?;
        let provider = core.settings_provider();

        let rust_ptr = CAnyRustPtr::new(provider);

        Ok(rust_ptr)
    }).response(value, error)
}
