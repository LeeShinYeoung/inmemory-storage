mod error;
mod stream;
pub use error::*;

use crate::storage::Key;

pub use self::stream::BufferedStream;

#[derive(Debug)]
pub enum Request {
  Get(GetRequest),
  Set(SetRequest),
  Delete(DeleteRequest),
}

trait Decodable
where
  Self: Sized,
{
  fn decode(stream: &mut BufferedStream) -> Result<Self>;
}

#[derive(Debug)]
pub struct GetRequest {
  pub key: Key,
}

impl Decodable for GetRequest {
  fn decode(stream: &mut BufferedStream) -> Result<Self> {
    let key_size = stream.read()?;
    let key = stream.read_n(key_size as usize)?.into();
    Ok(Self { key })
  }
}

#[derive(Debug)]
pub struct SetRequest {
  pub key: Key,
  pub value: Vec<u8>,
  pub ttl: Option<u64>,
}

impl Decodable for SetRequest {
  fn decode(stream: &mut BufferedStream) -> Result<Self> {
    let key_size = stream.read()?;
    let key = stream.read_n(key_size as usize)?.into();
    let value_len = stream.read_u32()?;
    let value = stream.read_n(value_len as usize)?;
    let ttl = match stream.read_u32()? {
      0 => None,
      i => Some(i as u64),
    };
    Ok(Self { key, value, ttl })
  }
}
#[derive(Debug)]
pub struct DeleteRequest {
  pub key: Key,
}

impl Decodable for DeleteRequest {
  fn decode(stream: &mut BufferedStream) -> Result<Self> {
    let key_size = stream.read()?;
    let key = stream.read_n(key_size as usize)?.into();
    Ok(Self { key })
  }
}

#[derive(Debug)]
pub struct Response {
  pub code: ResponseCode,
  pub value: Vec<u8>,
}
impl Response {
  pub fn success(value: Vec<u8>) -> Self {
    Self {
      code: ResponseCode::Success,
      value,
    }
  }
}

#[derive(Debug)]
pub enum ResponseCode {
  Success = 1,
  Fail = 2,
}

pub struct ProtocolParser {
  pub(crate) stream: BufferedStream,
}
impl ProtocolParser {
  pub fn encode(&mut self, response: Response) -> Result<()> {
    self.stream.write(&[response.code as u8])?;
    self
      .stream
      .write(&(response.value.len() as u32).to_be_bytes())?;
    self.stream.write(&response.value)?;

    self.stream.flush()
  }

  pub fn decode(&mut self) -> Result<Request> {
    let method = self.stream.read()?;

    let req = match method {
      1 => Request::Get(GetRequest::decode(&mut self.stream)?),
      2 => Request::Set(SetRequest::decode(&mut self.stream)?),
      3 => Request::Delete(DeleteRequest::decode(&mut self.stream)?),
      _ => return Err(Error::InvalidMethod),
    };

    Ok(req)
  }
}
