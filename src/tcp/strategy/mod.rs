use std::{
  net::TcpStream,
  sync::mpsc::{Receiver, Sender},
};

use crate::protocol::{Request, Response};

pub mod thread_per_connection;

pub trait TcpConnectionStrategy {
  fn handle(
    &self,
    stream: TcpStream,
    sender: Sender<(Request, Sender<Response>)>,
  ) -> std::io::Result<()>;
}
