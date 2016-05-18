// mod channel

use sqlib::client::Client;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, RustcDecodable, RustcEncodable)]
pub struct Channel {
    pub cid: i64,
    pub channel_name: String,
    pub clients: Vec<Client>,
}

impl Channel {
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    pub fn clients_len(&self) -> usize {
        self.clients.len()
    }

    pub fn new(channel_id: i64, name: String) -> Channel {
        Channel {
            cid: channel_id,
            channel_name: name,
            clients: Vec::new(),
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }

    pub fn sort_clients(&mut self) {
        self.clients.sort()
    }

    pub fn remove_sq_clients(&mut self) {
        let new_clients = self.clients
            .iter()
            .filter(|c| c.is_client())
            .map(|c| c.clone())
            .collect();
        self.clients = new_clients;
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.channel_name)
    }
}
