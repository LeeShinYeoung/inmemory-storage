mod list;

pub mod strategy;

use strategy::*;

use self::strategy::{lru::LeastRecentUsed, simple::Simple};

pub enum MaxMemoryStrategy {
  Simple,
  LeastRecentUsed,
}

pub struct Storage {
  strategy: Box<dyn Strategy>,
  max_size: usize,
}
impl Storage {
  pub fn new(size: usize, str: MaxMemoryStrategy) -> Self {
    let str: Box<dyn Strategy> = match str {
      MaxMemoryStrategy::Simple => Box::new(Simple::new()),
      MaxMemoryStrategy::LeastRecentUsed => Box::new(LeastRecentUsed::new()),
    };
    Self {
      strategy: str,
      max_size: size,
    }
  }

  pub fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
    self.strategy.get(key)
  }

  pub fn put(&mut self, key: &str, value: Vec<u8>) {
    if self.max_size != 0 {
      if let Some(size) = self.max_size.checked_sub(value.len()) {
        self.strategy.evict(size);
      }
    }

    self.strategy.allocate(key, value)
  }

  pub fn del(&mut self, key: &str) {
    self.strategy.deallocate(key)
  }
}
