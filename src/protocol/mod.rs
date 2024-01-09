#[derive(Debug)]
pub enum Method {
  Get,
  Set,
  Delete,
}

#[derive(Debug)]
pub struct Request {
  pub method: Method,
  pub key: Box<[u8]>,
  pub value: Box<[u8]>,
}

struct Response {}

pub fn parse(message: [u8; 512]) -> Request {
  let method = message[0];
  let key_length = message[1] as usize;
  let key = &message[2..2 + key_length];
  let value_length = message[2 + key_length] as usize;
  let value = &message[3 + key_length..3 + key_length + value_length];

  let method = match method {
    0 => Method::Get,
    1 => Method::Set,
    2 => Method::Delete,
    _ => panic!("Invalid method"),
  };

  Request {
    method,
    key: key.to_vec().into_boxed_slice(),
    value: value.to_vec().into_boxed_slice(),
  }
}

pub fn serialize() {}
