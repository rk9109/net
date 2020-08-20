use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NetError<'a> {
    IoError(io::Error),
    ParseError(&'a str),
    UnsupportedError(&'a str),
}

impl<'a> fmt::Display for NetError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NetError::*;

        match self {
            IoError(err) => err.fmt(f),
            ParseError(err_message) => write!(f, "ParseError: {}", err_message),
            UnsupportedError(err_message) => write!(f, "UnsupportedError: {}", err_message),
        }
    }
}

impl<'a> From<io::Error> for NetError<'a> {
    fn from(err: io::Error) -> NetError<'a> {
        NetError::IoError(err)
    }
}
