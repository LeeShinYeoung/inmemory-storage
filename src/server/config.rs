use std::{fs::File, path::Path};
use std::io::Read;
use serde::Deserialize;

use crate::{
    protocol::{Error, Result},
    storage::MaxMemoryStrategy,
};

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub max_memory_size: usize,
    pub max_memory_strategy: MaxMemoryStrategy,

}


impl ServerConfig {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self> {
        let mut file = File::open(path).map_err(Error::IO)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(Error::IO)?;

        toml::from_str(&contents).map_err(|_| Error::Unknown)
    }
}
