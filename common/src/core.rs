use std::sync::Arc;

use tesseract::service::{Tesseract, Transport};

use crate::{settings::SettingsProvider, service::TestService};

pub (crate) struct Core {
    tesseract: Tesseract,
    settings_provider: Arc<SettingsProvider>,
}

impl Core {
    pub (crate) fn new<T: Transport, F: FnOnce()->T>(data_dir: &str, ipc: F) -> Self {
        let location = format!("{}/settings.ini", data_dir);
        let settings_provider = Arc::new(SettingsProvider::new(&location));

        let tesseract = Tesseract::new()
            .transport(ipc())
            .service(TestService::new(/*ui,*/ Arc::clone(&settings_provider)));

        Self {
            tesseract: tesseract,
            settings_provider: settings_provider
        }
    }

    pub (crate) fn settings_provider(&self) -> Arc<SettingsProvider> {
        Arc::clone(&self.settings_provider)
    }
}