use std::fmt::{self, Display};
use std::{self, io};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    Eof,
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::Io(io_error)
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Io(error) => write!(formatter, "{error}"),
            Error::Eof => formatter.write_str("unexpected end of input"),
            /* and so forth */
        }
    }
}

impl std::error::Error for Error {}
