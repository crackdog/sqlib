// mod channel

use sqlib::client::Client;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Channel {
    pub cid: i64,
    pub channel_name: String,
    pub clients: Vec<Client>,
}

impl Channel {
    pub fn is_empty(&self) -> bool {
        self.clients.len() > 0
    }

    pub fn clients_len(&self) -> usize {
        self.clients.len()
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.channel_name)
    }
}
