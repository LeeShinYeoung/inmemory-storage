mod config;
mod router;

use std::sync::{
  mpsc::{channel, Sender},
  Arc, Mutex,
};

use crate::{
  protocol::parse,
  storage::size::{self, mb},
  // storage::{size::mb, Storage},
  tcp::{strategy::thread_per_connection::ThreadPerConnection, TcpServer, TcpServerConfig},
  thread_pool::ThreadPool,
};

pub struct Server {
  transport: TcpServer,
  background: ThreadPool,
  // storage: Arc<Mutex<Storage>>,
}
impl Server {
  pub fn new() -> Self {
    Self {
      transport: TcpServer::new(TcpServerConfig {
        strategy: Box::new(ThreadPerConnection::new(5)),
      }),
      background: ThreadPool::new(5, size::mb(10)),
      // storage: Arc::new(Mutex::new(Storage::new(size, str)))
    }
  }

  pub fn start(&self) -> std::io::Result<()> {
    let (tx, rx) = channel::<([u8; 512], Sender<[u8; 512]>)>();
    // let cs = Arc::clone(&self.storage);
    self.background.schedule(move || {
      while let Ok((msg, res)) = rx.recv() {
        let request = parse(msg);
        println!("{:?}", request.method);
        println!("{:?}", String::from_utf8_lossy(&request.key));
        println!("{:?}", String::from_utf8_lossy(&request.value));

        res.send(msg).unwrap();
      }
    });

    self.transport.listen(tx, 8080)
  }
}
