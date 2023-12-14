use std::{
  collections::{BinaryHeap, HashMap},
  time::SystemTime,
};

use super::{Page, Strategy};

pub struct TimeToLive {
  table: Box<HashMap<String, Page>>,
  queue: Box<BinaryHeap<Item>>,
}
impl Strategy for TimeToLive {
  fn new() -> Self
  where
    Self: Sized,
  {
    Self {
      table: Box::new(HashMap::new()),
      queue: Box::new(BinaryHeap::new()),
    }
  }

  fn get(&mut self, key: &str) -> Option<&Page> {
    self.table.get(key)
  }

  fn allocate(&mut self, key: String, page: Page) -> Option<Page> {
    if let Some(ex) = page.expired_at() {
      self.queue.push(Item::new(key.to_owned(), ex));
    }
    self.table.insert(key, page)
  }

  fn deallocate(&mut self, key: &str) -> Option<Page> {
    self.table.remove(key)
  }

  fn evict(&mut self) -> Option<String> {
    while let Some(item) = self.queue.pop() {
      if let Some(page) = self.table.get(&item.key) {
        if page.is_expired() {
          return Some(item.key);
        }

        if let Some(ex) = page.expired_at() {
          if ex.eq(&item.expired_at) {
            return Some(item.key);
          }
        }
      }
    }

    return None;
  }

  fn iter(&self) -> Box<dyn ExactSizeIterator<Item = (&String, &Page)> + '_> {
    Box::new(self.table.iter())
  }
}

struct Item {
  expired_at: SystemTime,
  key: String,
}
impl Item {
  fn new(key: String, expired_at: SystemTime) -> Self {
    Self { expired_at, key }
  }
}
impl PartialEq for Item {
  fn eq(&self, other: &Self) -> bool {
    self.expired_at == other.expired_at
  }
}
impl Eq for Item {}
impl PartialOrd for Item {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(other.expired_at.cmp(&self.expired_at))
  }
}
impl Ord for Item {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.expired_at.cmp(&self.expired_at)
  }
}
