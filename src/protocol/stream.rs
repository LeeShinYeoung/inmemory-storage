use std::io::Write;
use std::{io::Read, net::TcpStream};

use super::{Error, Result};

pub struct BufferedStream {
  stream: TcpStream,
  incoming_bytes: Vec<u8>,
  outgoing_bytes: Vec<u8>,
}
impl BufferedStream {
  pub fn new(stream: TcpStream) -> BufferedStream {
    BufferedStream {
      stream,
      incoming_bytes: Vec::new(),
      outgoing_bytes: Vec::new(),
    }
  }
  pub fn read_n(&mut self, n: usize) -> Result<Vec<u8>> {
    while self.incoming_bytes.len() < n {
      let mut b = Vec::with_capacity(512);
      let size = self.stream.read(b.as_mut()).map_err(Error::IO)?;
      if size == 0 {
        println!("Disconnected");
        return Err(Error::Disconnected);
      }

      self.incoming_bytes.append(&mut b.drain(..size).collect());
    }

    Ok(self.incoming_bytes.drain(..n).collect())
  }

  pub fn read(&mut self) -> Result<u8> {
    Ok(self.read_n(1)?[0])
  }

  pub fn read_u32(&mut self) -> Result<u32> {
    let mut b = [0; 4];
    b.copy_from_slice(self.read_n(4)?.as_ref());
    Ok(u32::from_be_bytes(b))
  }

  pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
    self.outgoing_bytes.extend_from_slice(bytes);

    if self.outgoing_bytes.len() < 512 {
      return Ok(());
    }

    self.flush()

    // self.stream.write_all(bytes).map_err(Error::IO)?;

    // self.stream.flush().map_err(Error::IO)
  }

  pub fn flush(&mut self) -> Result<()> {
    self
      .stream
      .write_all(&self.outgoing_bytes)
      .map_err(Error::IO)?;
    self.stream.flush().map_err(Error::IO)?;
    self.outgoing_bytes.clear();
    Ok(())
  }
}
