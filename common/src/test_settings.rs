pub(crate) struct TestSettings {
    signature: String,
    invalidator: String
}

pub(crate) struct TestSettingsProvider {
}

impl TestSettingsProvider {
    pub (crate) fn load(&self) -> TestSettings {
        todo!()
    }

    pub (crate) fn save(&self, settings: TestSettings) {
        todo!()
    }
}