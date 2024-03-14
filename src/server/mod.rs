mod config;

pub use config::*;

mod router;

use std::sync::mpsc::{channel, Sender};

use crate::protocol::ResponseCode;
use crate::server::router::RequestRouter;
use crate::{
  protocol::{Request, Response},
  storage::size::{self},
  tcp::{strategy::thread_per_connection::ThreadPerConnection, TcpServer, TcpServerConfig},
  thread_pool::ThreadPool,
};

pub struct Server {
  transport: TcpServer,
  background: ThreadPool,
  config: ServerConfig,
}

impl Server {
  pub fn new(config: ServerConfig) -> Self {
    Self {
      transport: TcpServer::new(TcpServerConfig {
        strategy: Box::new(ThreadPerConnection::new(5)),
      }),
      background: ThreadPool::new(5, size::mb(10)),
      config,
    }
  }

  pub fn start(&self) -> std::io::Result<()> {
    let (sender_to_handler, receiver_from_client) = channel::<(Request, Sender<Response>)>();
    let max_memory_size = self.config.max_memory_size;
    let max_memory_strategy = self.config.max_memory_strategy.clone();
    self.background.schedule(move || {
      let mut router = RequestRouter::new(max_memory_size, max_memory_strategy);
      while let Ok((request, sender_to_client)) = receiver_from_client.recv() {
        match router.handle(request) {
          Ok(response) => {
            sender_to_client.send(response).unwrap();
          }
          Err(error) => {
            sender_to_client
              .send(Response {
                code: ResponseCode::Fail,
                value: error.to_string().into_bytes(),
              })
              .unwrap();
          }
        };
      }
    });

    self.transport.listen(sender_to_handler, 8080)
  }
}
