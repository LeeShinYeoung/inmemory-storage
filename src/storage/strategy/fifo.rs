use std::{collections::HashMap, ptr::NonNull};

use crate::storage::{
  list::linked::{DoubleLinked, DoubleLinkedElement},
  Key,
};

use super::{Page, Strategy};

pub struct FirstInFirstOut {
  table: Box<HashMap<Key, FirstInFirstOutPage>>,
  queue: Box<DoubleLinked<Key>>,
}
impl Strategy for FirstInFirstOut {
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
    self.table.get(key).map(|page| &page.source)
  }

  fn allocate(&mut self, key: Key, page: Page) -> Option<Page> {
    let page = FirstInFirstOutPage::new(key.to_owned(), page);
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
unsafe impl Send for FirstInFirstOut {}
unsafe impl Sync for FirstInFirstOut {}

struct FirstInFirstOutPage {
  source: Page,
  element: NonNull<DoubleLinkedElement<Key>>,
}
impl FirstInFirstOutPage {
  fn new(key: Key, v: Page) -> Self {
    Self {
      source: v,
      element: DoubleLinkedElement::new_ptr(key),
    }
  }
}
