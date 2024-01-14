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

  // handler 시작
  pub fn start(&self) -> std::io::Result<()> {
    // 각각의 client와 하나의 handler를 연결하는 tx, rx 생성 (1번만 생성하고, tx는 복제하여 각각의 client로 전달)
    let (sender_to_handler, receiver_from_client) = channel::<(RawRequest, Sender<Response>)>();
    self.background.schedule(move || {
      while let Ok((raw_request, sender_to_client)) = receiver_from_client.recv() {
        //
        // println!("{:?}", request.method);
        // println!("{:?}", String::from_utf8_lossy(&request.key));
        // println!("{:?}", String::from_utf8_lossy(&request.value));
        //
        // let response = Response {
        //   code: ResponseCode::Success,
        //   value: request.value,
        // };

        println!("server.start - {:?}", raw_request);

        let temp_handler = TempHandler::new();
        match TcpRouter::handle(raw_request, temp_handler) {
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
