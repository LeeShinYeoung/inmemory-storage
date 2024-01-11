mod config;
mod router;

use std::sync::{
  mpsc::{channel, Sender},
  Arc, Mutex,
};

use crate::{
  protocol::{Request, Response, ResponseCode},
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
    let (tx, rx) = channel::<(Request, Sender<Response>)>();
    // let cs = Arc::clone(&self.storage);
    self.background.schedule(move || {
      while let Ok((msg, res)) = rx.recv() {
        println!("{:?}", msg.method);
        println!("{:?}", String::from_utf8_lossy(&msg.key));
        println!("{:?}", String::from_utf8_lossy(&msg.value));

        let response = Response {
          code: ResponseCode::Success,
          value: msg.value,
        };

        res.send(response).unwrap();
      }
    });

    self.transport.listen(tx, 8080)
  }
}
