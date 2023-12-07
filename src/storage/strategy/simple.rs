use std::collections::HashMap;

use super::Strategy;

pub struct Simple {
  map: HashMap<String, Vec<u8>>,
}
impl Strategy for Simple {
  fn new() -> Self
  where
    Self: Sized,
  {
    Self {
      map: HashMap::new(),
    }
  }

  fn allocate(&mut self, key: &str, value: Vec<u8>) {
    self.map.insert(key.to_string(), value);
  }

  fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
    self.map.get(key)
  }

  fn deallocate(&mut self, key: &str) {
    self.map.remove(key);
  }

  fn evict(&mut self, _: usize) {
    return;
  }
}
