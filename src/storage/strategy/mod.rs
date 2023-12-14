use std::{
  ops::Add,
  time::{Duration, SystemTime},
};

pub mod fifo;
pub mod lru;
pub mod simple;
pub mod ttl;

pub trait Strategy: Send + Sync {
  fn new() -> Self
  where
    Self: Sized;
  fn get(&mut self, key: &str) -> Option<&Page>;
  fn allocate(&mut self, key: String, page: Page) -> Option<Page>;
  fn deallocate(&mut self, key: &str) -> Option<Page>;
  fn evict(&mut self) -> Option<String>;
  fn iter(&self) -> Box<dyn ExactSizeIterator<Item = (&String, &Page)> + '_>;
}

#[derive(Clone, Debug)]
pub struct Page {
  ttl: Option<Duration>,
  created_at: SystemTime,
  value: Box<Vec<u8>>,
}

impl Page {
  pub fn new(value: Vec<u8>, ttl: Option<u64>) -> Self {
    let now = SystemTime::now();
    Self {
      created_at: now,
      ttl: ttl.map(|ttl| Duration::from_millis(ttl)),
      value: Box::new(value),
    }
  }

  pub fn expired_at(&self) -> Option<SystemTime> {
    self.ttl.map(|ttl| self.created_at.add(ttl))
  }

  pub fn is_expired(&self) -> bool {
    match self.expired_at() {
      Some(ex) => ex <= SystemTime::now(),
      None => false,
    }
  }

  pub fn get_value(&self) -> Vec<u8> {
    self.value.to_vec()
  }

  pub fn size(&self) -> usize {
    self.value.len()
  }

  pub fn has_expiry(&self) -> bool {
    self.ttl.is_some()
  }
}
