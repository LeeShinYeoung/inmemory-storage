mod config;
mod router;

use std::io::ErrorKind;
use std::sync::mpsc::{channel, Sender};

use crate::server::router::{Handler, TcpRouter, TempHandler};
use crate::tcp::strategy::thread_per_connection::RawRequest;
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
    }
  }

  pub fn start(&self) -> std::io::Result<()> {
    let (sender_to_handler, receiver_from_client) = channel::<(Request, Sender<Response>)>();
    self.background.schedule(move || {
      while let Ok((request, sender_to_client)) = receiver_from_client.recv() {

        let temp_handler = TempHandler::new();
        match TcpRouter::handle(request, temp_handler) {
          Ok(response) => {
            sender_to_client.send(response).unwrap();
          }
          Err(error) => panic!("Error: {:#?}", error),
        };
      }
    });

    self.transport.listen(sender_to_handler, 8080)
  }
}
