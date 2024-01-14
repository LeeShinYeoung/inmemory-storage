use std::{
  io::{Error, ErrorKind, Read, Write},
  net::TcpStream,
  sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
  },
};

use crate::{
  protocol::{decode, encode, Request, Response},
  storage::size,
  thread_pool::ThreadPool,
};

use super::TcpConnectionStrategy;

pub struct ThreadPerConnection {
  active_connections: Arc<Mutex<u32>>,
  max_connections: u32,
  pool: Box<ThreadPool>,
}

impl ThreadPerConnection {
  pub fn new(max_connections: u32) -> Self {
    ThreadPerConnection {
      active_connections: Arc::new(Mutex::new(0)),
      max_connections,
      pool: Box::new(ThreadPool::new(max_connections as usize, size::mb(10))),
    }
  }

  fn open_connection(
    active_connections: &Arc<Mutex<u32>>,
    max_connections: u32,
  ) -> std::io::Result<()> {
    let mut active_connections = active_connections.lock().unwrap();

    if *active_connections >= max_connections {
      return Err(Error::new(
        ErrorKind::ConnectionRefused,
        "Max connections reached",
      ));
    }

    *active_connections += 1;
    Ok(())
  }

  fn close_connection(active_connections: &Arc<Mutex<u32>>) {
    let mut active_connections = active_connections.lock().unwrap();

    if *active_connections == 0 {
      println!("No connections to close");
      return;
    }

    *active_connections -= 1;
  }
}

#[derive(Debug)]
pub struct RawRequest {
  pub data: [u8; 512],
}

impl RawRequest {
  fn new() -> Self {
    RawRequest { data: [0; 512] }
  }
}

// client
impl TcpConnectionStrategy for ThreadPerConnection {
  fn handle(
    &self,
    mut stream: TcpStream,
    sender_to_handler: Sender<(RawRequest, Sender<Response>)>,
  ) -> std::io::Result<()> {
    let active_connection = Arc::clone(&self.active_connections);

    ThreadPerConnection::open_connection(&active_connection, self.max_connections)?;

    self.pool.schedule(move || loop {
      // let mut buffer = [0; 512];
      let mut raw_request = RawRequest::new();
      // println!("raw_request: {:#?}", raw_request);
      let mut buffer = [0; 512];
      let byte = stream
        // .read(&mut raw_request.data)
        .read(&mut buffer)
        .expect("Failed to read from stream");

      println!("{:?} : buffer", buffer);

      if byte == 0 {
        ThreadPerConnection::close_connection(&active_connection);
        break;
      }

      let (sender_to_client, receiver_from_handler) = channel();

      println!("handler - {:?}", raw_request);
      // let request = decode(buffer);
      sender_to_handler
        .send((raw_request, sender_to_client))
        .unwrap();
      while let Ok(response) = receiver_from_handler.recv() {
        let buffer = encode(response);
        match stream.write_all(&buffer) {
          Ok(_) => continue,
          Err(_) => return,
        }
      }
      stream.flush().unwrap();
    });

    Ok(())
  }
}
