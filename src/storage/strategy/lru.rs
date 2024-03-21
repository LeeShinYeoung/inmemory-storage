use std::{collections::HashMap, ptr::NonNull};

use crate::storage::{
  list::linked::{DoubleLinked, DoubleLinkedElement},
  Key,
};

use super::{Page, Strategy};

pub struct LeastRecentUsed {
  table: Box<HashMap<Key, LeastRecentUsedPage>>,
  queue: Box<DoubleLinked<Key>>,
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

  fn get(&mut self, key: &Key) -> Option<&Page> {
    self.table.get(key).map(|page| {
      unsafe {
        self.queue.remove(page.element);
        self.queue.push_back(page.element);
      };
      return &page.source;
    })
  }

  fn allocate(&mut self, key: Key, page: Page) -> Option<Page> {
    let page = LeastRecentUsedPage::new(key.to_owned(), page);
    unsafe { self.queue.push_back(page.element) };
    self.table.insert(key, page).map(|old| {
      unsafe { self.queue.remove(old.element) };
      return old.source;
    })
  }

  fn deallocate(&mut self, key: &Key) -> Option<Page> {
    self.table.remove(key).map(|old| {
      unsafe { self.queue.remove(old.element) };
      return old.source;
    })
  }

  fn evict(&mut self) -> Option<Key> {
    self.queue.front().map(|key| key.clone())
  }

  fn iter(&self) -> Box<dyn ExactSizeIterator<Item = (&Key, &Page)> + '_> {
    Box::new(self.table.iter().map(|(k, v)| (k, &v.source)))
  }
}
unsafe impl Send for LeastRecentUsed {}
unsafe impl Sync for LeastRecentUsed {}

struct LeastRecentUsedPage {
  element: NonNull<DoubleLinkedElement<Key>>,
  source: Page,
}
impl LeastRecentUsedPage {
  fn new(k: Key, page: Page) -> Self {
    Self {
      element: DoubleLinkedElement::new_ptr(k),
      source: page,
    }
  }
}
