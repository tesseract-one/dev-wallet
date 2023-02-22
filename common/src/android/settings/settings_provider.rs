use crate::settings::SettingsProvider;

use interop_android::{JavaDesc, JavaWrappableDesc};

impl JavaDesc for SettingsProvider {
    fn java_class<'a>(&'a self) -> &'a str {
        panic!("Doesn't have a single java wrapper. Use custom Desc")
    }
}

impl JavaWrappableDesc for SettingsProvider {
}

pub (crate) enum SettingsProviderType {
    Test,
    Key
}

impl JavaDesc for SettingsProviderType {
    fn java_class<'a>(&'a self) -> &'a str {
        match self {
            SettingsProviderType::Test => "one/tesseract/devwallet/rust/TestSettingsProvider",
            SettingsProviderType::Key => "one/tesseract/devwallet/rust/KeySettingsProvider",
        }
    }
}