use std::{net::TcpStream, sync::mpsc::Sender};

use crate::protocol::{Request, Response};
use crate::tcp::strategy::thread_per_connection::RawRequest;

pub mod thread_per_connection;

pub trait TcpConnectionStrategy {
  fn handle(
    &self,
    stream: TcpStream,
    sender_to_handler: Sender<(RawRequest, Sender<Response>)>,
  ) -> std::io::Result<()>;
}
