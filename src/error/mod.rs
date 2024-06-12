use std::string::{FromUtf16Error, FromUtf8Error};

#[derive(Debug)]
pub enum Error {
    FromUTF8(FromUtf8Error),
    FromUTF16(FromUtf16Error),
    Regex(regex::Error),
    IO(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FromUTF8(error) => write!(f, "{}", error),
            Error::FromUTF16(error) => write!(f, "{}", error),
            Error::Regex(error) => write!(f, "{}", error),
            Error::IO(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::FromUTF8(error)
    }
}
impl From<FromUtf16Error> for Error {
    fn from(error: FromUtf16Error) -> Self {
        Error::FromUTF16(error)
    }
}
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::Regex(error)
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}
