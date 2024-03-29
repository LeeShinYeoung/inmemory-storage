use std::{
  io::{Error, ErrorKind},
  net::TcpStream,
  sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
  },
};

use crate::protocol::{BufferedStream, ProtocolParser};
use crate::{
  protocol::{Request, Response},
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

// client
impl TcpConnectionStrategy for ThreadPerConnection {
  fn handle(
    &self,
    stream: TcpStream,
    sender_to_handler: Sender<(Request, Sender<Response>)>,
  ) -> std::io::Result<()> {
    let active_connection = Arc::clone(&self.active_connections);

    ThreadPerConnection::open_connection(&active_connection, self.max_connections)?;

    let buffered_bytes = BufferedStream::new(stream);
    let mut protocol_parser = ProtocolParser {
      stream: buffered_bytes,
    };

    self.pool.schedule(move || loop {
      let request = match protocol_parser.decode() {
        Ok(request) => request,
        Err(crate::protocol::Error::Disconnected) => {
          ThreadPerConnection::close_connection(&active_connection);
          return;
        }
        Err(error) => {
          dbg!(error);
          break;
        }
      };

      println!("Request: {:?}", request);
      let (sender_to_client, receiver_from_handler) = channel();

      sender_to_handler.send((request, sender_to_client)).unwrap();

      while let Ok(response) = receiver_from_handler.recv() {
        protocol_parser.encode(response).unwrap();
      }
    });

    Ok(())
  }
}
