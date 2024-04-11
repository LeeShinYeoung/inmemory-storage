mod ip;
pub use ip::*;

use std::{net::TcpListener, sync::mpsc::Sender};

use crate::protocol::{Request, Response};

use self::strategy::TcpConnectionStrategy;

pub mod strategy;

pub struct TcpServerConfig {
  pub(crate) strategy: Box<dyn TcpConnectionStrategy>,
  pub whitelist: Option<IpNetList>,
  pub blacklist: Option<IpNetList>,
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
    sender_to_handler: Sender<(Request, Sender<Response>)>,
    port: u16,
  ) -> std::io::Result<()> {
    let host = "localhost";
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(address)?;

    loop {
      let (stream, addr) = listener.accept()?;
      if let Some(whitelist) = &self.config.whitelist {
        if !whitelist.contains(&addr) {
          continue;
        }
      }

      if let Some(blacklist) = &self.config.blacklist {
        if blacklist.contains(&addr) {
          continue;
        }
      }

      let sender_to_handler = sender_to_handler.clone();
      if let Err(error) = self.config.strategy.handle(stream, sender_to_handler) {
        println!("Error: {}", error);
      }
    }
  }
}
