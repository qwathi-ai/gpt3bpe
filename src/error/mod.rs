use std::string::{FromUtf16Error, FromUtf8Error};
#[derive(Debug)]
pub struct BytePairEncodingError {
    pub grapheme: Vec<String>,
}
#[derive(Debug)]
pub struct BytePairDecodingError {
    pub grapheme: Vec<i32>,
}
#[derive(Debug)]
pub enum ERROR {
    FromUTF8(FromUtf8Error),
    FromUTF16(FromUtf16Error),
    Regex(regex::Error),
    IO(std::io::Error),
    Encoding(BytePairEncodingError),
    Decoding(BytePairDecodingError),
}

impl std::fmt::Display for BytePairEncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "character in grapheme {:?} could not be encoded.",
            self.grapheme
        )
    }
}

impl std::fmt::Display for BytePairDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "integer in grapheme {:?} could not be decoded.",
            self.grapheme
        )
    }
}

impl std::fmt::Display for ERROR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ERROR::FromUTF8(error) => write!(f, "{}", error),
            ERROR::FromUTF16(error) => write!(f, "{}", error),
            ERROR::Regex(error) => write!(f, "{}", error),
            ERROR::IO(error) => write!(f, "{}", error),
            ERROR::Encoding(error) => write!(f, "{}", error),
            ERROR::Decoding(error) => write!(f, "{}", error),
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

impl From<BytePairEncodingError> for ERROR {
    fn from(error: BytePairEncodingError) -> Self {
        ERROR::Encoding(error)
    }
}

impl From<BytePairDecodingError> for ERROR {
    fn from(error: BytePairDecodingError) -> Self {
        ERROR::Decoding(error)
    }
}
