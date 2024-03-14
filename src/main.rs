use server::{Server, ServerConfig};

mod protocol;
mod server;
mod storage;
mod tcp;
mod thread_pool;

fn main() {
  let path = env!("CONFIG_PATH");
  dbg!(path);
  let config = ServerConfig::from_path(path).unwrap();
  let server = Server::new(config);
  server.start().unwrap();
}
