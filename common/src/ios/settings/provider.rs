use tesseract_utils::{ptr::{CAnyRustPtr, IntoAnyPtr}, string::CStringRef, traits::TryAsRef, response::CResponse};

pub type CSettingsProvider = CAnyRustPtr; //wraps Arc<SettingsProvider>