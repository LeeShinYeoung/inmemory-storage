use std::collections::HashMap;

use crate::storage::Key;

use super::{Page, Strategy};

pub struct Simple {
  table: Box<HashMap<Key, Page>>,
}
impl Strategy for Simple {
  fn new() -> Self
  where
    Self: Sized,
  {
    Self {
      table: Box::new(HashMap::new()),
    }
  }

  fn allocate(&mut self, key: Key, page: Page) -> Option<Page> {
    self.table.insert(key, page)
  }

  fn get(&mut self, key: &Key) -> Option<&Page> {
    self.table.get(key)
  }

  fn deallocate(&mut self, key: &Key) -> Option<Page> {
    self.table.remove(key)
  }

  fn evict(&mut self) -> Option<Key> {
    None
  }

  fn iter(&self) -> Box<dyn ExactSizeIterator<Item = (&Key, &Page)> + '_> {
    Box::new(self.table.iter())
  }
}
