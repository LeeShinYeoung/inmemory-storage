use serde::Deserialize;
use std::io::Read;
use std::{fs::File, path::Path};

use crate::protocol::Error;
use crate::tcp::IpNetList;
use crate::{protocol::Result, storage::MaxMemoryStrategy};

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
  pub max_memory_size: usize,
  pub max_memory_strategy: MaxMemoryStrategy,
  pub ip_whitelist: Option<IpNetList>,
  pub ip_blacklist: Option<IpNetList>,
}

impl ServerConfig {
  pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self> {
    let mut file = File::open(path).map_err(Error::IO)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(Error::IO)?;

    toml::from_str(&contents).map_err(|_| Error::Unknown)
  }
}
