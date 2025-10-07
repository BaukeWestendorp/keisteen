use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),

    Eof,

    InvalidTagId(u8),
    UnexpectedTag(String),
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidLength { expected: usize, found: usize },
    MissingField(String),
    TrailingData,
    Io(std::io::Error),
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
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::InvalidTagId(id) => write!(formatter, "invalid tag id: {}", id),
            Error::UnexpectedTag(tag) => write!(formatter, "unexpected tag: {}", tag),
            Error::InvalidUtf8(err) => write!(formatter, "invalid UTF-8: {}", err),
            Error::InvalidLength { expected, found } => {
                write!(formatter, "invalid length: expected {}, found {}", expected, found)
            }
            Error::MissingField(field) => write!(formatter, "missing field: {}", field),
            Error::TrailingData => formatter.write_str("trailing data after deserialization"),
            Error::Io(error) => write!(formatter, "io error: {}", error),
        }
    }
}

impl std::error::Error for Error {}
