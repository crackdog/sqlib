extern crate rustc_serialize;

mod client;
mod channel;
mod connection;
mod error;

use std::result;

pub use client::Client;
pub use channel::Channel;
pub use connection::Connection;
pub use error::{Error, SQError};

pub type Result<T> = result::Result<T, Error>;
