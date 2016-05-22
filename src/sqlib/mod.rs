mod client;
mod channel;
pub mod connection;
mod error;

use std::result;

pub use sqlib::client::Client;
pub use sqlib::channel::Channel;
pub use sqlib::connection::Connection;
pub use sqlib::error::{Error, SQError};

pub type Result<T> = result::Result<T, Error>;
