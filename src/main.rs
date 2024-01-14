use server::Server;
use std::io::{Read, Write};

mod protocol;
mod server;
mod storage;
mod tcp;
mod thread_pool;

fn main() {
  let server = Server::new();
  server.start().unwrap();
}
