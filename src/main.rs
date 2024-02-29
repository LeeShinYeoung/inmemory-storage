use server::{Server, ServerConfig};

mod protocol;
mod server;
mod storage;
mod tcp;
mod thread_pool;

fn main() {
  let config = ServerConfig::from_path();
  let server = Server::new(config);
  server.start().unwrap();
}
