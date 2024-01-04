mod config;
mod router;

use std::sync::{mpsc::channel, Arc, Mutex};

use crate::{storage::Storage, tcp::TcpServer, thread_pool::ThreadPool};

pub struct Server {
  transport: TcpServer,
  background: ThreadPool,
  storage: Arc<Mutex<Storage>>,
}
impl Server {
  fn start(&self) -> std::io::Result<()> {
    let (tx, rx) = channel();
    let cs = Arc::clone(&self.storage);
    self.background.schedule(|| {
      let parser = Parser::new();
      while let Ok((msg, res)) = rx.recv() {
        parser.parse(msg);
        res.send()
      }
    });

    self.transport.listen(tx, 8080)
  }
}
