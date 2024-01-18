use crate::protocol::{Method, Request, Response, ResponseCode};
use crate::storage::{MaxMemoryStrategy, Storage};
use std::io::{Error, ErrorKind};

pub struct RequestRouter {
  storage: Storage,
}

impl RequestRouter {
  pub fn new() -> Self {
    Self {
      storage: Storage::new(512, MaxMemoryStrategy::Simple),
    }
  }

  pub fn handle(&mut self, request: Request) -> Result<Response, ErrorKind> {
    let Request { method, key, value } = request;

    println!("method: {:?}", &method);
    println!("key: {:?}", String::from_utf8_lossy(&key));
    println!(
      "value: {:?}, length: {}",
      String::from_utf8_lossy(&value),
      &value.len()
    );

    let storage = &mut self.storage;

    let result = match method {
      Method::Get => storage.get(&String::from_utf8(key).unwrap()),
      Method::Set => {
        storage.put(
          String::from_utf8(key.clone()).unwrap(),
          value.clone(),
          Some(10000),
        );
        Ok(vec![1])
      }
      Method::Delete => {
        storage.del(&String::from_utf8(key).unwrap());
        Ok(vec![1])
      }
      _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid method")),
    };

    match result {
      Ok(value) => {
        let response = Response {
          code: ResponseCode::Success,
          value,
        };
        Ok(response)
      }
      Err(error) => {
        println!("Error: {:#?}", error);
        Err(error.kind())
      }
    }
  }
}
