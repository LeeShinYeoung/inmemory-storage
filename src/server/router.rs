use crate::protocol::{decode, encode, Request, Response, ResponseCode};
use crate::tcp::strategy::thread_per_connection::RawRequest;
use std::io::ErrorKind;
use std::sync::mpsc::Sender;

pub trait Handler {
  fn new() -> Box<Self>
  where
    Self: Sized;
  fn handle(&self, request: Request) -> Result<Response, ErrorKind>;
}

pub struct TempHandler {}

impl Handler for TempHandler {
  fn new() -> Box<Self>
  where
    Self: Sized,
  {
    Box::new(TempHandler {})
  }
  fn handle(&self, request: Request) -> Result<Response, ErrorKind> {
    println!("{:?}", request.method);
    println!("{:?}", String::from_utf8_lossy(&request.key));
    println!("{:?}", String::from_utf8_lossy(&request.value));

    let response = Response {
      code: ResponseCode::Success,
      value: request.value,
    };

    Ok(response)
  }
}

pub struct TcpRouter {}

impl TcpRouter {
  pub fn handle(raw_request: RawRequest, handler: Box<dyn Handler>) -> Result<Response, ErrorKind> {
    let request = decode(raw_request);
    let request_value = request.value.clone();
    let result = handler.handle(request);
    match result {
      Ok(_) => {
        let response = Response {
          code: ResponseCode::Success,
          value: request_value,
        };
        Ok(response)
      }
      Err(error) => {
        println!("Error: {:#?}", error);
        Err(error)
      }
    }
  }
}
