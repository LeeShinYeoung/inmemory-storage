#[derive(Debug)]
pub enum Error {
  IO(std::io::Error),
  InvalidMethod,
  Disconnected,
}
pub type Result<T> = core::result::Result<T, Error>;
