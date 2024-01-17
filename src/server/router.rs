use std::io::{Error, ErrorKind};
use crate::protocol::{Method, Request, Response, ResponseCode};
use crate::storage::{MaxMemoryStrategy, Storage};

pub struct RequestRouter {}

impl RequestRouter {
  pub fn handle(request: Request) -> Result<Response, ErrorKind> {
    let Request {method, key, value} = request;

    println!("{:?}", &method);
    println!("{:?}", String::from_utf8_lossy(&key));
    println!("{:?}", String::from_utf8_lossy(&value));

    let mut storage = Storage::new(512, MaxMemoryStrategy::TimeToLive);

    let result = match method {
      Method::Get => {
        storage.get(&String::from_utf8(key).unwrap())
      },
      Method::Set => {
        let s = storage.put(String::from_utf8(key).unwrap(), value.clone(), Some(100));
        Ok(vec![1])
      },
      Method::Delete => {
        storage.del(&String::from_utf8(key).unwrap());
        Ok(vec![1])
      },
      _ => {
        Err(Error::new(ErrorKind::InvalidInput, "Invalid method"))
      }
    };


    match result {
      Ok(_) => {
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
