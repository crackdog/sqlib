// mod error
use std::error::{self, Error as Err};
use std::fmt::{self, Display};
use std::io;
use std::convert::From;
use std::net::AddrParseError;
use std::sync::PoisonError;
use escaping::unescape;

/// # Example
/// ```
/// use sqlib::SQError;
///
/// let line = "error id=0 msg=ok".to_string();
///
/// let err_option = SQError::parse(&line);
/// let err = match err_option {
///     Some(err) => err,
///     None => { panic!("no error found"); },
/// };
/// assert_eq!(0, err.id());
/// ```
#[derive(Debug)]
pub struct SQError {
    id: u32,
    msg: String,
    full_msg: String,
}

// helping function for SQError::parse
fn is_seperator(c: char) -> bool {
    c == '=' || c.is_whitespace()
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

    pub fn ok() -> SQError {
        SQError::new(0, "ok".to_string())
    }

    pub fn parse_is_ok(s: &str) -> super::Result<bool> {
        let err = match SQError::parse(s) {
            None => {
                return Ok(false);
            }
            Some(err) => err,
        };
        if err == SQError::ok() {
            Ok(true)
        } else {
            Err(Error::from(err))
        }
    }

    /// try to parse a String to a SQError
    pub fn parse(s: &str) -> Option<SQError> {
        // the str shouldn't be trimmed, because a real error is without
        // whitespace in the beginning
        let parts: Vec<&str> = s.splitn(6, is_seperator).collect();
        if parts.len() < 5 {
            return None;
        }
        if parts[0] != "error" {
            return None;
        }
        if parts[1] != "id" {
            return None;
        }
        let id_result = parts[2].clone().parse::<u32>();
        let id = match id_result {
            Err(_) => {
                return None;
            }
            Ok(val) => val,
        };
        if parts[3] != "msg" {
            return None;
        }
        let msg = parts[4].to_string().clone();
        Some(SQError::new(id, unescape(&msg)))
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

impl PartialEq for SQError {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SQError {}


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

impl<'a> From<&'a str> for Error {
    fn from(err: &'a str) -> Error {
        Error::Other(err.to_string())
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

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Error {
        Error::Other(err.description().to_string())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Error {
        Error::Other(format!("{}", err))
    }
}
