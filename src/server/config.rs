use std::{fs::File, path::Path};

use crate::{
  protocol::{Error, Result},
  storage::MaxMemoryStrategy,
};

pub struct ServerConfig {
  pub max_memory_size: usize,
  pub max_memory_strategy: MaxMemoryStrategy,
}
impl ServerConfig {
  pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self> {
    let file = File::open(path).map_err(Error::IO)?;
    todo!()
  }
}
