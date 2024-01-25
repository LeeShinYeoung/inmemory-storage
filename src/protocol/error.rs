pub enum Error {
  IO(std::io::Error),
  InvalidMethod,
}
pub type Result<T> = core::result::Result<T, Error>;
