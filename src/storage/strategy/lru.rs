use std::{collections::HashMap, ptr::NonNull, mem};

use crate::storage::list::{DoubleLinked, DoubleLinkedElement};

use super::Strategy;

pub struct LeastRecentUsed {
  table: HashMap<String, Page>,
  accessed: DoubleLinked<String>,
  allocated: usize,
}
impl Strategy for LeastRecentUsed {
  fn new() -> Self
  where
    Self: Sized,
  {
    Self {
      table: HashMap::new(),
      accessed: DoubleLinked::new(),
      allocated: 0,
    }
  }

  fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
    if let Some(page) = self.table.get(key).take() {
      unsafe {
        self.accessed.remove(page.accessed);
        self.accessed.push_back(page.accessed);
      };
      return Some(&page.value);
    }
    return None;
  }

  fn allocate(&mut self, key: &str, value: Vec<u8>) {
    if let Some(page) = self.table.get(key).take() {
      self.allocated -= page.size();
      unsafe { self.accessed.remove(page.accessed) };
      self.table.remove(key);
    }
    let page = Page::new(key.to_string(), value);
    unsafe { self.accessed.push_back(page.accessed) };
    self.allocated += page.size();
    self.table.insert(key.to_string(), page);
  }

  fn deallocate(&mut self, key: &str) {
    if let Some(page) = self.table.get(key).take() {
      self.allocated -= page.size();
      unsafe { self.accessed.remove(page.accessed) };
      self.table.remove(key);
    }
    self.table.remove(key);
  }

  fn evict(&mut self, size: usize) {
    loop {
      if self.allocated <= size {
        return;
      }
      if self.accessed.len() == 0 {
        return;
      }

      let key = self.accessed.pop_front().unwrap();
      self
        .table
        .remove(key.as_ref().into())
        .map(|page| self.allocated -= page.size());
    }
  }
}

#[derive(Debug)]
struct Page {
  accessed: NonNull<DoubleLinkedElement<String>>,
  value: Vec<u8>,
}
impl Page {
  fn new(k: String, v: Vec<u8>) -> Self {
    Self {
      accessed: DoubleLinkedElement::new_ptr(k),
      value: v,
    }
  }

  fn size(&self) -> usize {
    self.value.len()
  }
}
