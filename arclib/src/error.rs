use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidKeyLength(usize, usize),
    IoError(std::io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKeyLength(expected, got) => {
                write!(f, "Invalid key length: expected {expected}, got {got}")
            },
            Self::IoError(e) => {
                write!(f, "I/O error: {e}")
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}