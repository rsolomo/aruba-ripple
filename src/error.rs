use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};
use hyper;

/// Airwave API client errors
#[derive(Debug)]
pub enum Error {
    /// An authentication error
    Authentication,
    /// An HTTP error from Hyper
    Hyper(hyper::error::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Authentication => f.write_str("authentication failed"),
            Error::Hyper(ref e) => Display::fmt(e, f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Aruba client error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Authentication => None,
            Error::Hyper(ref e) => Some(e),
        }
    }
}
