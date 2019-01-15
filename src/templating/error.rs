use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    BadFormat(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::IOError(e) => format!("IOError: {}", e),
                Error::BadFormat(e) => format!("BadFormat: {}", e),
            }
        )
    }
}

impl std::error::Error for Error {}
