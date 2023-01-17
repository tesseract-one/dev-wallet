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
        self.read(|settings| {
            TestSettings {
                signature: "signed_by_rust_on_steroids".to_owned(),
                invalidator: "eee1231".to_owned(),
            }
        })
    }

    fn save_test_settings(&self, settings: TestSettings) {
        panic!()
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

