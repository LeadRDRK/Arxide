use std::fmt;

#[derive(Debug)]
pub enum Error {
    ArclibError(arclib::Error),
    IoError(std::io::Error),
    StripPrefixError(std::path::StripPrefixError),
    InvalidPath(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArclibError(e) => {
                e.fmt(f)
            },
            Self::IoError(e) => {
                write!(f, "I/O error: {e}")
            },
            Self::StripPrefixError(e) => {
                e.fmt(f)
            },
            Self::InvalidPath(message) => {
                write!(f, "Invalid path: {message}")
            }
        }
    }
}

impl From<arclib::Error> for Error {
    fn from(e: arclib::Error) -> Self {
        Error::ArclibError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(e: std::path::StripPrefixError) -> Self {
        Error::StripPrefixError(e)
    }
}