use server::{Server, ServerConfig};
use std::env;

mod protocol;
mod server;
mod storage;
mod tcp;
mod thread_pool;

fn main() {
  let path = env::var("CONFIG_PATH").unwrap_or(String::from("./example/config.toml"));
  let config = ServerConfig::from_path(path).unwrap();
  let server = Server::new(config);
  server.start().unwrap();
}
