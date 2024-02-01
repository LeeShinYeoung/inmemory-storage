mod bytes;
mod error;
pub use error::*;

pub use self::bytes::BufferedBytes;


#[derive(Debug)]
pub enum Method {
  Get,
  Set,
  Delete,
}
impl TryFrom<u8> for Method {
  type Error = Error;
  fn try_from(value: u8) -> std::prelude::v1::Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Get),
      1 => Ok(Self::Set),
      2 => Ok(Self::Delete),
      _ => Err(Error::InvalidMethod),
    }
  }
}

#[derive(Debug)]
pub struct RawRequest {
  pub value: [u8; 512],
}

impl RawRequest {
  pub fn new() -> Self {
    RawRequest { value: [0; 512] }
  }
}

#[derive(Debug)]
pub struct Request {
  pub method: Method,
  pub key: Vec<u8>,
  pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct Response {
  pub code: ResponseCode,
  pub value: Vec<u8>,
}

#[derive(Debug)]
pub enum ResponseCode {
  Success = 0,
  Fail = 1,
}

pub fn decode(raw_request: RawRequest) -> Request {
  let message = raw_request.value;

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
    key: key.to_vec(),
    value: value.to_vec(),
  }
}

pub fn encode(response: Response) -> [u8; 512] {
  let code = response.code as u8;
  let value_length = response.value.len();

  let mut buffer = [0; 512];
  buffer[0] = code;
  buffer[1] = value_length as u8;
  buffer[2..2 + value_length].copy_from_slice(&response.value[..]);

  buffer
}
pub struct ProtocolParser {
  pub(crate) bytes: BufferedBytes,
}
impl ProtocolParser {
  pub fn encode(&mut self, response: Response) {
    //TODO: serialize response and write buffer to stream
  }

  pub fn decode(&mut self) -> Result<Request> {
    let method = self.bytes.read()?;
    let key_size = self.bytes.read()?;
    let key = self.bytes.read_n(key_size as usize)?;
    let value_len = self.bytes.read_u32()?;
    let value = self.bytes.read_n(value_len as usize)?;

    Ok(Request {
      method: Method::try_from(method)?,
      key,
      value,
    })
  }
}
