use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use server::Server;
use tcp::strategy::thread_per_connection::ThreadPerConnection;
use tcp::{TcpServer, TcpServerConfig};

mod protocol;
mod server;
mod storage;
mod tcp;
mod thread_pool;

fn main() {
  let server = Server::new();
  server.start().unwrap();

  // let (tx, rx): (Sender<[u8; 512]>, Receiver<[u8; 512]>) = mpsc::channel();

  // thread::spawn(move || {
  //   while let Ok(message) = rx.recv() {
  //     println!("Message: {}", String::from_utf8_lossy(&message));
  //   }
  // });

  // let server = TcpServer::new(TcpServerConfig {
  //   strategy: Box::new(ThreadPerConnection::new(5)),
  //   sender: tx,
  // });

  // server.listen(8080).unwrap();
}
