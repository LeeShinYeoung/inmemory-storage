use std::{net::TcpListener, sync::mpsc::Sender};

use crate::protocol::{Request, Response};

use self::strategy::TcpConnectionStrategy;

pub mod strategy;

pub struct TcpServerConfig {
  pub(crate) strategy: Box<dyn TcpConnectionStrategy>,
  // pub sender: Sender<[u8; 512]>,
}

pub struct TcpServer {
  config: TcpServerConfig,
}

impl TcpServer {
  pub fn new(config: TcpServerConfig) -> TcpServer {
    TcpServer { config }
  }

  pub fn listen(
    &self,
    sender: Sender<(Request, Sender<Response>)>,
    port: u16,
  ) -> std::io::Result<()> {
    let host = "localhost";
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
      // let sender = self.config.sender.clone();
      let sender = sender.clone();
      if let Err(error) = self.config.strategy.handle(stream?, sender) {
        println!("Error: {}", error);
      }
    }

    Ok(())
  }
}
