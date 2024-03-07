#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidMethod,
    Disconnected,
    Unknown,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::IO(error) => error.to_string(),
            Error::InvalidMethod => format!("invalid method"),
            Error::Disconnected => format!("disconnected"),
            Error::Unknown => format!("unknown error"),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
