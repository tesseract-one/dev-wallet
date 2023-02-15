use crate::error::Result;

use super::SettingsProvider;

pub(crate) struct KeySettings {
    pub mnemonic: Option<String>,
}

pub(crate) trait KeySettingsProvider {
    fn load_key_settings(&self) -> Result<KeySettings>;
    fn save_key_settings(&self, settings: KeySettings) -> Result<()>;
}

impl KeySettingsProvider for SettingsProvider {
    fn load_key_settings(&self) -> Result<KeySettings> {
        self.read(|config| {
            let mnemonic = config.get("key", "mnemonic");
            KeySettings {
                mnemonic: mnemonic,
            }
        })
    }

    fn save_key_settings(&self, settings: KeySettings) -> Result<()> {
        self.write(|config| {
            config.set("key", "mnemonic", settings.mnemonic);
        })
    }
}