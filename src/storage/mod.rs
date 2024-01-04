mod list;
pub mod size;

pub mod strategy;

use std::{
  io::{Error, ErrorKind, Result},
  sync::{Arc, Mutex},
  thread::{sleep, spawn},
  time::Duration,
};

use rand::{thread_rng, Rng};

use self::strategy::{
  fifo::FirstInFirstOut, lru::LeastRecentUsed, simple::Simple, ttl::TimeToLive, Page, Strategy,
};

pub enum MaxMemoryStrategy {
  Simple,
  LeastRecentUsed,
  FirstInFirstOut,
  TimeToLive,
}

pub struct Storage {
  strategy: Arc<Mutex<dyn Strategy>>,
  max_size: usize,
  allocated: Arc<Mutex<usize>>,
}

impl Storage {
  pub fn new(size: usize, str: MaxMemoryStrategy) -> Self {
    let str: Arc<Mutex<dyn Strategy>> = match str {
      MaxMemoryStrategy::Simple => Arc::new(Mutex::new(Simple::new())),
      MaxMemoryStrategy::LeastRecentUsed => Arc::new(Mutex::new(LeastRecentUsed::new())),
      MaxMemoryStrategy::FirstInFirstOut => Arc::new(Mutex::new(FirstInFirstOut::new())),
      MaxMemoryStrategy::TimeToLive => Arc::new(Mutex::new(TimeToLive::new())),
    };
    let s = Self {
      strategy: str,
      max_size: size,
      allocated: Arc::new(Mutex::new(0)),
    };
    s.start();
    s
  }

  pub fn get(&mut self, key: &str) -> Result<Vec<u8>> {
    if let Some(page) = {
      let mut s = self.strategy.lock().unwrap();
      s.get(key).map(|page| page.clone())
    } {
      if page.is_expired() {
        let mut s = self.strategy.lock().unwrap();
        let mut a = self.allocated.lock().unwrap();
        *a -= s.deallocate(&key).map(|old| old.size()).unwrap_or(0);
        return Err(Error::from(ErrorKind::NotFound));
      }
      return Ok(page.get_value());
    }
    return Err(Error::from(ErrorKind::NotFound));
  }

  pub fn put(&mut self, key: String, value: Vec<u8>, ttl: Option<u64>) -> Result<()> {
    if self.max_size < value.len() {
      return Err(Error::from(ErrorKind::OutOfMemory));
    }

    let page = Page::new(value, ttl);
    if self.max_size == 0 {
      let mut s = self.strategy.lock().unwrap();
      let mut a = self.allocated.lock().unwrap();
      *a += page.size();
      *a -= s.allocate(key, page).map(|old| old.size()).unwrap_or(0);
      return Ok(());
    }

    while {
      let a = self.allocated.lock().unwrap();
      self.max_size - *a < page.size()
    } {
      let mut s = self.strategy.lock().unwrap();
      match s.evict() {
        None => break,
        Some(key) => {
          let mut a = self.allocated.lock().unwrap();
          *a -= s.deallocate(&key).map(|old| old.size()).unwrap_or(0);
        }
      }
    }

    let mut s = self.strategy.lock().unwrap();
    let mut a = self.allocated.lock().unwrap();
    *a += page.size();
    *a -= s.allocate(key, page).map(|old| old.size()).unwrap_or(0);
    return Ok(());
  }

  pub fn del(&mut self, key: &str) -> Result<()> {
    let mut s = self.strategy.lock().unwrap();
    let mut a = self.allocated.lock().unwrap();
    *a -= s.deallocate(&key).map(|old| old.size()).unwrap_or(0);
    return Ok(());
  }

  fn start(&self) {
    let c = Arc::clone(&self.strategy);
    let a = Arc::clone(&self.allocated);
    spawn(move || loop {
      sleep(Duration::from_millis(100));
      loop {
        let keys: Vec<String> = {
          let s = c.lock().unwrap();
          let iter = s.iter();
          let rand = thread_rng().gen_range(0..iter.len().checked_sub(20).unwrap_or(0));
          iter
            .filter(|(_, p)| p.has_expiry())
            .skip(rand)
            .take(20)
            .filter(|(_, p)| p.is_expired())
            .map(|(k, _)| k.to_owned())
            .collect()
        };

        for key in keys.iter() {
          let mut s = c.lock().unwrap();
          let mut a = a.lock().unwrap();
          *a -= s.deallocate(&key).map(|old| old.size()).unwrap_or(0);
        }
        if keys.len() < 5 {
          break;
        }
      }
    });
  }
}
