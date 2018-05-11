//! error provides newtypes and Error's for sqlib.

use escaping::unescape;
use std::convert::From;
use std::error::{self, Error as Err};
use std::fmt::{self, Display};
use std::io;
use std::net::AddrParseError;
use std::result;
use std::sync::PoisonError;

/// The standart result type of sqlib.
pub type Result<T> = result::Result<T, Error>;

/// A SQError contains a TS3 Server Query error.
///
/// # Example
/// ```
/// use sqlib::error::SQError;
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

    /// Creates an OK error.
    pub fn ok() -> SQError {
        SQError::new(0, "ok".to_string())
    }

    /// This function tries to parse a string into an Error.
    ///
    /// # Return values
    ///
    /// - If it fails to parse it returns Ok(false).
    /// - If it parses the string as an OK error it returns Ok(true).
    /// - If it parses the string as another error it returns Err(error).
    ///
    /// # Example
    /// ```
    /// use sqlib::error::{Error, SQError};
    ///
    /// let ok_str = "error id=0 msg=ok";
    /// let no_err_str = "this is no error";
    /// let err_str = "error id=1 msg=test";
    ///
    /// let test_err = SQError::new(1, "test".to_string());
    /// let to_test_error = match SQError::parse_is_ok(err_str).unwrap_err() {
    ///     Error::SQ(e) => e,
    ///     _ => SQError::ok(),
    /// };
    ///
    /// assert_eq!(SQError::parse_is_ok(ok_str).unwrap(), true);
    /// assert_eq!(SQError::parse_is_ok(no_err_str).unwrap(), false);
    /// assert_eq!(to_test_error, test_err);
    /// ```
    pub fn parse_is_ok(s: &str) -> Result<bool> {
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
        let id_result = parts[2].parse::<u32>();
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

/// Error is a custom Error type for the sqlib.
#[derive(Debug)]
pub enum Error {
    /// wraps io::Error
    Io(io::Error),
    /// server query error messages
    SQ(SQError),
    /// other errors
    Other(String),
}

impl Error {
    pub fn is_io(&self) -> bool {
        match *self {
            Error::Io(_) => true,
            _ => false,
        }
    }

    pub fn is_sq(&self) -> bool {
        match *self {
            Error::SQ(_) => true,
            _ => false,
        }
    }

    pub fn is_other(&self) -> bool {
        match *self {
            Error::Other(_) => true,
            _ => false,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::SQ(ref err) => err.description(),
            Error::Other(ref s) => s,
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
