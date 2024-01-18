use std::{net::TcpStream, sync::mpsc::Sender};

use crate::protocol::{Request, Response};

pub mod thread_per_connection;

pub trait TcpConnectionStrategy {
  fn handle(
    &self,
    stream: TcpStream,
    sender_to_handler: Sender<(Request, Sender<Response>)>,
  ) -> std::io::Result<()>;
}
