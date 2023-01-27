use std::{sync::Mutex, fs::OpenOptions, io::Read};

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
        let mut config = self.config.lock()?;

        let mut file = OpenOptions::new().create(true).write(true).open(&self.location)?;
        let len = file.metadata()?.len();
        let mut content = String::with_capacity(len as usize);

        file.read_to_string(&mut content)?;

        config.read(content).map_err(|err| Error::Config(err))?;
        
        Ok(read(&config))
    }

    pub (super) fn write<F: FnOnce(&mut Ini)>(&self, write: F) -> Result<()> {
        let mut data = self.config.lock()?;
        
        write(&mut data);

        Ok(data.write(&self.location)?)
    }
}