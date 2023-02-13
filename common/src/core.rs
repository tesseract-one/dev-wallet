use std::sync::Arc;

use tesseract::service::Tesseract;

use crate::{settings::SettingsProvider, service::TestService, service::SubstrateService, ui::UI};

pub (crate) struct Core {
    _tesseract: Tesseract,
    settings_provider: Arc<SettingsProvider>,
}

impl Core {
    pub (crate) fn new<F: FnOnce(Tesseract)->Tesseract>(ui:UI, data_dir: &str, apply_transports: F) -> Self {
        let ui = Arc::new(ui);
        let location = format!("{}/settings.ini", data_dir);
        let settings_provider = Arc::new(SettingsProvider::new(&location));

        let tesseract = apply_transports(Tesseract::new())
            .service(TestService::new(Arc::clone(&ui), Arc::clone(&settings_provider)))
            .service(SubstrateService::new(Arc::clone(&ui), Arc::clone(&settings_provider)));

        info!("Core created successfully");

        Self {
            _tesseract: tesseract,
            settings_provider: settings_provider
        }
    }

    pub (crate) fn settings_provider(&self) -> Arc<SettingsProvider> {
        Arc::clone(&self.settings_provider)
    }
}