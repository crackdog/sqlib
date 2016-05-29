extern crate rustc_serialize;

mod client;
mod channel;
mod connection;
mod command;
mod error;
pub mod map;
pub mod escaping;

use std::result;

pub use client::{Client, ClientList};
pub use channel::{Channel, ChannelList};
pub use connection::Connection;
pub use command::Command;
pub use error::{Error, SQError};

pub type Result<T> = result::Result<T, Error>;
