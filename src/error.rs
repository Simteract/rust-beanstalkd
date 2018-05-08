use std::io;
use std::num;
use std::str;

#[derive(Debug, Fail)]
pub enum BeanstalkdError {
    #[fail(display = "Unable to establish connection {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Expected data was missing")]
    MissingData,
    #[fail(display = "The server responded with an unxpected status")]
    InvalidStatus,
    #[fail(display = "Input was invalid UTF-8: {}", _0)]
    Utf8Error(str::Utf8Error),
    #[fail(display = "Input was not a valid integer: {}", _0)]
    ParseIntError(num::ParseIntError),
}

impl From<io::Error> for BeanstalkdError {
    fn from(error: io::Error) -> Self {
        BeanstalkdError::Io(error)
    }
}

impl From<num::ParseIntError> for BeanstalkdError {
    fn from(error: num::ParseIntError) -> Self {
        BeanstalkdError::ParseIntError(error)
    }
}

impl From<str::Utf8Error> for BeanstalkdError {
    fn from(error: str::Utf8Error) -> Self {
        BeanstalkdError::Utf8Error(error)
    }
}

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
