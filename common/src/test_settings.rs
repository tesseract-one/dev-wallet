pub(crate) struct TestSettings {
    pub signature: String,
    pub invalidator: String
}

pub(crate) struct TestSettingsProvider {
}

impl TestSettingsProvider {
    pub (crate) fn load(&self) -> TestSettings {
        TestSettings {
            signature: "signed_by_rust_on_steroids".to_owned(),
            invalidator: "eee".to_owned(),
        }
    }

    pub (crate) fn save(&self, settings: TestSettings) {
        debug!("Sig: {}, Err: {}", settings.signature, settings.invalidator);
    }
}

