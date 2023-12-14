use std::{collections::HashMap, ptr::NonNull};

use crate::storage::list::linked::{DoubleLinked, DoubleLinkedElement};

use super::{Page, Strategy};

pub struct LeastRecentUsed {
  table: Box<HashMap<String, LeastRecentUsedPage>>,
  queue: Box<DoubleLinked<String>>,
}
impl Strategy for LeastRecentUsed {
  fn new() -> Self
  where
    Self: Sized,
  {
    Self {
      table: Box::new(HashMap::new()),
      queue: Box::new(DoubleLinked::new()),
    }
  }

  fn get(&mut self, key: &str) -> Option<&Page> {
    self.table.get(key).map(|page| {
      unsafe {
        self.queue.remove(page.element);
        self.queue.push_back(page.element);
      };
      return &page.source;
    })
  }

  fn allocate(&mut self, key: String, page: Page) -> Option<Page> {
    let page = LeastRecentUsedPage::new(key.to_owned(), page);
    unsafe { self.queue.push_back(page.element) };
    self.table.insert(key, page).map(|old| {
      unsafe { self.queue.remove(old.element) };
      return old.source;
    })
  }

  fn deallocate(&mut self, key: &str) -> Option<Page> {
    self.table.remove(key).map(|old| {
      unsafe { self.queue.remove(old.element) };
      return old.source;
    })
  }

  fn evict(&mut self) -> Option<String> {
    self.queue.front().map(|key| key.to_string())
  }

  fn iter(&self) -> Box<dyn ExactSizeIterator<Item = (&String, &Page)> + '_> {
    Box::new(self.table.iter().map(|(k, v)| (k, &v.source)))
  }
}
unsafe impl Send for LeastRecentUsed {}
unsafe impl Sync for LeastRecentUsed {}

struct LeastRecentUsedPage {
  element: NonNull<DoubleLinkedElement<String>>,
  source: Page,
}
impl LeastRecentUsedPage {
  fn new(k: String, page: Page) -> Self {
    Self {
      element: DoubleLinkedElement::new_ptr(k),
      source: page,
    }
  }
}