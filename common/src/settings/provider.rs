use std::sync::Mutex;

use configparser::ini::Ini;

use crate::error::{Error, Result};

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

    pub (super) fn read<T, F: FnOnce(&Ini)->T>(&self, read: F) -> Result<T> {
        let mut data = self.config.lock()?;

        data.load(&self.location).map_err(|err| Error::Config(err))?;
        
        Ok(read(&data))
    }

    pub (super) fn write<F: FnOnce(&mut Ini)>(&self, write: F) -> Result<()> {
        let mut data = self.config.lock()?;
        
        write(&mut data);

        Ok(data.write(&self.location)?)
    }
}