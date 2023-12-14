use std::{net::TcpStream, sync::mpsc::Sender};

pub mod thread_per_connection;

pub trait TcpConnectionStrategy {
  fn handle(&self, stream: TcpStream, sender: Sender<[u8; 512]>) -> std::io::Result<()>;
}
