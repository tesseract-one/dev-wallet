use std::mem::ManuallyDrop;

use tesseract::service::Transport;
use tesseract_utils::error::CError;
use tesseract_utils::panic::handle_exception_result;
use tesseract_utils::{ptr::{CAnyRustPtr, IntoAnyPtr}, string::CStringRef, traits::TryAsRef, response::CResponse};

use crate::settings;
use crate::{core::Core, ios::settings::test_settings::CTestSettings, settings::TestSettings};

pub type CCore = CAnyRustPtr;

impl IntoAnyPtr for Core {
}

struct TR {
}

impl Transport for TR {
    fn bind(self, processor: std::sync::Arc<dyn tesseract::service::TransportProcessor + Send + Sync>) -> Box<dyn tesseract::service::BoundTransport> {
        todo!()
    }
}

//TODO: change to result
#[no_mangle]
pub unsafe extern "C" fn wallet_ccore_create(/*ui: ManuallyDrop<UI>,*/ data_dir: CStringRef, value: &mut ManuallyDrop<CCore>, error: &mut ManuallyDrop<CError>) -> bool {
    handle_exception_result(|| {
        let data_dir = data_dir.try_as_ref()?;

        let core = Core::new(super::UI {}, data_dir, || {TR {}});
    
        Ok(core.into())
    }).response(value, error)
}

#[no_mangle]
pub unsafe extern "C" fn wallet_seee() -> ManuallyDrop<CTestSettings> {
    let test_settings = TestSettings {
        signature: "sig".to_owned(),
        invalidator: "inv".to_owned(),
    };

    ManuallyDrop::new(test_settings.into())
}

#[no_mangle]
pub unsafe extern "C" fn wallet_s2(settings: ManuallyDrop<CTestSettings>) {
    let settings: TestSettings = ManuallyDrop::into_inner(settings).try_into().unwrap();

}

