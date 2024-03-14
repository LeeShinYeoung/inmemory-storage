mod error;
mod stream;
pub use error::*;

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
  fn decode(stream: &BufferedStream) -> Result<Self>;
}

#[derive(Debug)]
struct GetRequest {
  key: Vec<u8>,
}

impl Decodable for GetRequest {
  fn decode(stream: &BufferedStream) -> Result<Self> {
    let key_size = stream.read()?;
    let key = stream.read_n(key_size as usize)?;
    Ok(Self { key })
  }
}

#[derive(Debug)]
struct SetRequest {
  pub key: Vec<u8>,
  pub value: Vec<u8>,
  pub ttl: Option<u64>,
}

impl Decodable for SetRequest {
  fn decode(stream: &BufferedStream) -> Result<Self> {
    let key_size = stream.read()?;
    let key = stream.read_n(key_size as usize)?;
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
struct DeleteRequest {
  key: Vec<u8>,
}

impl Decodable for DeleteRequest {
  fn decode(stream: &BufferedStream) -> Result<Self> {
    let key_size = stream.read()?;
    let key = stream.read_n(key_size as usize)?;
    Ok(Self { key })
  }
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

    Ok(match method {
      0 => Request::Get(GetRequest::decode(&self.stream)?),
      1 => Request::Set(SetRequest::decode(&self.stream)?),
      2 => Request::Delete(DeleteRequest::decode(&self.stream)?),
      _ => todo!(),
    })
  }
}
