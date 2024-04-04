use ipnet::IpNet;
use serde::Deserialize;
use std::io::Read;
use std::{fs::File, path::Path};

use crate::{
  protocol::{Error, Result},
  storage::MaxMemoryStrategy,
};

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
  pub max_memory_size: usize,
  pub max_memory_strategy: MaxMemoryStrategy,
  // TODO
  // pub ip_whitelist: Vec<IpNet>,
  // pub ip_blacklist: Vec<IpNet>,
}

impl ServerConfig {
  pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self> {
    let mut file = File::open(path).map_err(Error::IO)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(Error::IO)?;

    toml::from_str(&contents).map_err(|_| Error::Unknown)
  }
}
