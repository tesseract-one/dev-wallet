use std::sync::Arc;

use tesseract::service::{Tesseract, Transport};

use crate::{settings::SettingsProvider, service::TestService, ui::UI};

pub (crate) struct Core {
    tesseract: Tesseract,
    settings_provider: Arc<SettingsProvider>,
}

impl Core {
    pub (crate) fn new<F: FnOnce(Tesseract)->Tesseract>(ui:UI, data_dir: &str, apply_transports: F) -> Self {
        let location = format!("{}/settings.ini", data_dir);
        let settings_provider = Arc::new(SettingsProvider::new(&location));

        let tesseract = apply_transports(Tesseract::new())
            .service(TestService::new(ui, Arc::clone(&settings_provider)));

        Self {
            tesseract: tesseract,
            settings_provider: settings_provider
        }
    }

    pub (crate) fn settings_provider(&self) -> Arc<SettingsProvider> {
        Arc::clone(&self.settings_provider)
    }
}