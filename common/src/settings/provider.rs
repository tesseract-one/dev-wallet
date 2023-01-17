use std::sync::Mutex;

use configparser::ini::Ini;

pub(crate) struct SettingsProvider {
    config: Mutex<Ini>,
    location: String
}

impl SettingsProvider {
    pub (crate) fn new(location: &str) -> Self {
        Self {
            config: Mutex::new(Ini::new_cs()),
            location: location.to_owned()
        }
    }

    pub (super) fn read<T, F: FnOnce(&Ini)->T>(&self, read: F) -> T {
        let mut data = self.config.lock().unwrap();

        data.load(&self.location).unwrap();
        
        read(&data)
    }

    pub (super) fn write<F: FnOnce(&mut Ini)>(&self, write: F) {
        let mut data = self.config.lock().unwrap();
        
        write(&mut data);

        data.write(&self.location).unwrap()
    }
}