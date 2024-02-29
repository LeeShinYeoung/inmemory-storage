mod error;
mod stream;
pub use error::*;

pub use self::stream::BufferedStream;

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
pub struct Request {
  pub method: Method,
  pub key: Vec<u8>,
  pub value: Vec<u8>,
  pub ttl: Option<u64>,
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

pub struct ProtocolParser {
  pub(crate) stream: BufferedStream,
}
impl ProtocolParser {
  pub fn encode(&mut self, response: Response) -> Result<()> {
    //TODO: serialize response and write buffer to stream

    self.stream.write(&[response.code as u8])?;
    self
      .stream
      .write(&(response.value.len() as u32).to_be_bytes())?;
    self.stream.write(&response.value)?;

    self.stream.flush()
  }

  pub fn decode(&mut self) -> Result<Request> {
    let method = self.stream.read()?;
    let key_size = self.stream.read()?;
    let key = self.stream.read_n(key_size as usize)?;
    let value_len = self.stream.read_u32()?;
    let value = self.stream.read_n(value_len as usize)?;
    let ttl = match self.stream.read_u32()? {
      0 => None,
      i => Some(i as u64),
    };

    Ok(Request {
      method: Method::try_from(method)?,
      key,
      value,
      ttl,
    })
  }
}
