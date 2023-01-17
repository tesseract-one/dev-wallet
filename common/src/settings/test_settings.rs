use super::SettingsProvider;

pub(crate) struct TestSettings {
    pub signature: String,
    pub invalidator: String
}

pub(crate) trait TestSettingsProvider {
    fn load_test_settings(&self) -> TestSettings;
    fn save_test_settings(&self, settings: TestSettings);
}

impl TestSettingsProvider for SettingsProvider {
    fn load_test_settings(&self) -> TestSettings {
        self.read(|config| {
            let signature = config.get("test", "signature").unwrap_or_else(|| "signed_by_tesseract".to_owned());
            let invalidator = config.get("test", "invalidator").unwrap_or_else(|| "err".to_owned());
            TestSettings {
                signature: signature,
                invalidator: invalidator,
            }
        })
    }

    fn save_test_settings(&self, settings: TestSettings) {
        self.write(|config| {
            config.set("test", "signature", Some(settings.signature));
            config.set("test", "invalidator", Some(settings.invalidator));
        })
    }
}

/*impl TestSettingsProvider {
    pub (crate) fn load_test_settings(&self) -> TestSettings {
        let aa = self.config.load(&self.location).unwrap();

        let aaa = aa["TEST"];

        TestSettings {
            signature: "signed_by_rust_on_steroids".to_owned(),
            invalidator: "eee".to_owned(),
        }
    }

    pub (crate) fn save(&self, settings: TestSettings) {
        debug!("Sig: {}, Err: {}", settings.signature, settings.invalidator);
    }
}*/

