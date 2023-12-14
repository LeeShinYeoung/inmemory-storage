use std::collections::HashMap;

use super::{Page, Strategy};

pub struct Simple {
  table: Box<HashMap<String, Page>>,
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

  fn allocate(&mut self, key: String, page: Page) -> Option<Page> {
    self.table.insert(key, page)
  }

  fn get(&mut self, key: &str) -> Option<&Page> {
    self.table.get(key)
  }

  fn deallocate(&mut self, key: &str) -> Option<Page> {
    self.table.remove(key)
  }

  fn evict(&mut self) -> Option<String> {
    None
  }

  fn iter(&self) -> Box<dyn ExactSizeIterator<Item = (&String, &Page)> + '_> {
    Box::new(self.table.iter())
  }
}
