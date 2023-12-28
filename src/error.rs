
use std::io;
use std::result;
use std::error;

#[derive(Debug)]
pub enum Error
{
  StringErr(String),
  IOErr(io::Error),
}

impl From<io::Error> for Error
{
  fn from(err: io::Error) -> Error {
    Error::IOErr(err)
  }
}
impl From<String> for Error
{
  fn from(err: String) -> Error {
    Error::StringErr(err)
  }
}

pub type Result<T> = result::Result<T, Error>;
