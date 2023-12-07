pub mod lru;
pub mod simple;

pub trait Strategy {
  fn new() -> Self
  where
    Self: Sized;
  fn get(&mut self, key: &str) -> Option<&Vec<u8>>;
  fn allocate(&mut self, key: &str, value: Vec<u8>);
  fn deallocate(&mut self, key: &str);
  fn evict(&mut self, size: usize);
}
