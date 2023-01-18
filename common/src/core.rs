use std::sync::Arc;

use crate::settings::SettingsProvider;

pub (crate) struct Core {
    settings_provider: Arc<SettingsProvider>
}

impl Core {
    pub (crate) fn new(data_dir: &str) -> Self {
        let location = format!("{}/settings.ini", data_dir);
        let settings_provider = Arc::new(SettingsProvider::new(&location));

        Self { settings_provider: settings_provider }
    }

    pub (crate) fn settings_provider(&self) -> Arc<SettingsProvider> {
        Arc::clone(&self.settings_provider)
    }
}