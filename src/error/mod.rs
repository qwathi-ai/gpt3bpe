use std::string::{FromUtf16Error, FromUtf8Error};

#[derive(Debug)]
pub enum ERROR {
    FromUTF8(FromUtf8Error),
    FromUTF16(FromUtf16Error),
    Regex(regex::Error),
    IO(std::io::Error),
}

impl std::fmt::Display for ERROR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERROR::FromUTF8(error) => write!(f, "{}", error),
            ERROR::FromUTF16(error) => write!(f, "{}", error),
            ERROR::Regex(error) => write!(f, "{}", error),
            ERROR::IO(error) => write!(f, "{}", error),
        }
    }
}
impl std::error::Error for ERROR {}

impl From<FromUtf8Error> for ERROR {
    fn from(error: FromUtf8Error) -> Self {
        ERROR::FromUTF8(error)
    }
}
impl From<FromUtf16Error> for ERROR {
    fn from(error: FromUtf16Error) -> Self {
        ERROR::FromUTF16(error)
    }
}
impl From<regex::Error> for ERROR {
    fn from(error: regex::Error) -> Self {
        ERROR::Regex(error)
    }
}
impl From<std::io::Error> for ERROR {
    fn from(error: std::io::Error) -> Self {
        ERROR::IO(error)
    }
}
