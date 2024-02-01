use std::{io::Read, net::TcpStream};
use std::io::Write;
use std::net::Shutdown;

use super::{Error, Result};

pub struct BufferedBytes {
  stream: TcpStream,
  buffered_bytes: Vec<u8>,
}
impl BufferedBytes {
  pub fn new(stream: TcpStream) -> BufferedBytes {
    BufferedBytes {
      stream,
      buffered_bytes: Vec::new(),
    }
  }
  pub fn read_n(&mut self, n: usize) -> Result<Vec<u8>> {
    while self.buffered_bytes.len() < n {
      let mut b = Vec::with_capacity(512);
      let size = self.stream.read(b.as_mut()).map_err(|err| Error::IO(err))?;
      //TODO: when size is 0 then disconnect stream
      if size == 0 {
        // self.stream.shutdown(Shutdown::Both).unwrap();
        break;
      }

      self.buffered_bytes.append(&mut b.drain(..size).collect());
    }

    Ok(self.buffered_bytes.drain(..n).collect())
  }

  pub fn read(&mut self) -> Result<u8> {
    Ok(self.read_n(1)?[0])
  }

  pub fn read_u32(&mut self) -> Result<u32> {
    let mut b = [0; 4];
    b.copy_from_slice(self.read_n(4)?.as_ref());
    Ok(u32::from_be_bytes(b))
  }
}
