use serde::de;
use std::fmt::{self, Display};
use std::{error, io, result};

/// A convenience `Result` type for this crate.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Deserialize(String),
    EofWhileParsingValue,
    ExpectedSomeValue,
    FromUtf8Error(std::string::FromUtf8Error),
    InvalidByteStrLen,
    InvalidInteger,
    InvalidDict,
    InvalidList,
    IoError(io::Error),
    KeyMustBeAByteStr,
    ParseIntError(std::num::ParseIntError),
    TrailingData,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Deserialize(_) => None,
            Error::EofWhileParsingValue => None,
            Error::ExpectedSomeValue => None,
            Error::FromUtf8Error(err) => Some(err),
            Error::InvalidByteStrLen => None,
            Error::InvalidInteger => None,
            Error::InvalidDict => None,
            Error::InvalidList => None,
            Error::IoError(err) => Some(err),
            Error::KeyMustBeAByteStr => None,
            Error::ParseIntError(err) => Some(err),
            Error::TrailingData => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Deserialize(str) => f.write_str(str),
            Error::EofWhileParsingValue => f.write_str("eof while parsing value"),
            Error::ExpectedSomeValue => f.write_str("expected some value"),
            Error::FromUtf8Error(err) => Display::fmt(&*err, f),
            Error::InvalidByteStrLen => f.write_str("invalid byte string length"),
            Error::InvalidInteger => f.write_str("invalid integer"),
            Error::InvalidDict => f.write_str("invalid dictionary"),
            Error::InvalidList => f.write_str("invalid list"),
            Error::IoError(err) => Display::fmt(&*err, f),
            Error::KeyMustBeAByteStr => f.write_str("key must be a byte string"),
            Error::ParseIntError(err) => Display::fmt(&*err, f),
            Error::TrailingData => f.write_str("trailing data error"),
        }
    }
}

impl From<Error> for io::Error {
    fn from(other: Error) -> Self {
        match other {
            Error::IoError(e) => e,
            _ => io::Error::from(io::ErrorKind::Other),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(other: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(other)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(other: std::num::ParseIntError) -> Self {
        Error::ParseIntError(other)
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Deserialize(msg.to_string())
    }

    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::Deserialize(format!(
            "unexpected type error. invalid_type={}, expected_type={}",
            unexp, exp
        ))
    }
}
