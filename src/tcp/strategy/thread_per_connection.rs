use std::{
  io::{Error, ErrorKind, Read, Write},
  net::TcpStream,
  sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
  },
};

use crate::{storage::size, thread_pool::ThreadPool};

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

impl TcpConnectionStrategy for ThreadPerConnection {
  fn handle(
    &self,
    mut stream: TcpStream,
    sender: Sender<([u8; 512], Sender<[u8; 512]>)>,
  ) -> std::io::Result<()> {
    let active_connection = Arc::clone(&self.active_connections);

    ThreadPerConnection::open_connection(&active_connection, self.max_connections)?;
    self.pool.schedule(move || loop {
      let mut buffer = [0; 512];
      let byte = stream
        .read(&mut buffer)
        .expect("Failed to read from stream");

      if byte == 0 {
        ThreadPerConnection::close_connection(&active_connection);
        break;
      }

      let (tx, rx) = channel();
      sender.send((buffer, tx)).unwrap();
      while let Ok(recv) = rx.recv() {
        match stream.write_all(&recv) {
          Ok(_) => continue,
          Err(_) => return,
        }
      }
      stream.flush().unwrap();
    });

    Ok(())
  }
}
