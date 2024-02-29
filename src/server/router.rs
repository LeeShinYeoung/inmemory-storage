use crate::protocol::{Error, Method, Request, Response, ResponseCode, Result};
use crate::storage::{MaxMemoryStrategy, Storage};

pub struct RequestRouter {
  storage: Storage,
}

impl RequestRouter {
  pub fn new(storage_size: usize, max_memory_strategy: MaxMemoryStrategy) -> Self {
    Self {
      storage: Storage::new(storage_size, max_memory_strategy),
    }
  }

  pub fn handle(&mut self, request: Request) -> Result<Response> {
    let Request {
      method,
      key,
      value,
      ttl,
    } = request;

    println!("method: {:?}", &method);
    println!("key: {:?}", String::from_utf8_lossy(&key));
    println!(
      "value: {:?}, length: {}",
      String::from_utf8_lossy(&value),
      &value.len()
    );

    let storage = &mut self.storage;

    let key = String::from_utf8_lossy(&key).to_string();

    let result = match method {
      Method::Get => storage.get(&key).map_err(Error::IO),
      Method::Set => {
        storage.put(key, value.clone(), ttl).map_err(Error::IO)?;
        Ok(vec![])
      }
      Method::Delete => {
        storage.del(&key).map_err(Error::IO)?;
        Ok(vec![])
      }
    }?;

    Ok(Response {
      code: ResponseCode::Success,
      value: result,
    })
  }
}
