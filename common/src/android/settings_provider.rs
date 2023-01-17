use crate::settings::SettingsProvider;

use super::interop::{JavaDesc, JavaWrappableDesc};

impl JavaDesc for SettingsProvider {
    fn java_class<'a>(&'a self) -> &'a str {
        panic!("Doesn't have a single java wrapper. Use custom Desc")
    }
}

impl JavaWrappableDesc for SettingsProvider {
}

pub (super) enum SettingsProviderType {
    test,
    substrate
}

impl JavaDesc for SettingsProviderType {
    fn java_class<'a>(&'a self) -> &'a str {
        match self {
            SettingsProviderType::test => "one/tesseract/devwallet/rust/TestSettingsProvider",
            SettingsProviderType::substrate => "one/tesseract/devwallet/rust/SubstrateSettingsProvider",
        }
    }
}