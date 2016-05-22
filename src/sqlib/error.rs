// mod error
use std::error;
use std::error::Error as Err;
use std::fmt::{self, Display};
use std::io;
use std::convert::From;

#[derive(Debug)]
pub struct SQError {
    id: u32,
    msg: String,
    full_msg: String,
}

impl SQError {
    pub fn new(id: u32, msg: String) -> SQError {
        let full_msg_str = format!("error id={} msg={}", id, &msg);
        SQError {
            id: id,
            msg: msg,
            full_msg: full_msg_str,
        }
    }

    // pub fn parse(s: String) -> Option<SQError> {

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

impl Display for SQError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_msg)
    }
}

impl error::Error for SQError {
    fn description(&self) -> &str {
        &self.full_msg
    }
}

/// SQError is a custom Error type for the sqlib.
#[derive(Debug)]
pub enum Error {
    /// wraps io::Error
    Io(io::Error),
    /// server query error messages
    SQ(SQError),
    /// other errors
    Other(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Io(ref err) => err.description(),
            &Error::SQ(ref err) => err.description(),
            &Error::Other(ref s) => s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<SQError> for Error {
    fn from(err: SQError) -> Error {
        Error::SQ(err)
    }
}
