// mod channel

use client::{Client, ClientList};
use std::fmt;
use std::cmp;
use std::ops::Deref;
use std::str::FromStr;
use map::*;

/// # Example
/// ```
/// use sqlib;
///
/// let channel = sqlib::Channel::new(0, "test".to_string());
///
/// assert!(channel.is_empty());
/// assert_eq!("test".to_string(), format!("{}", channel));
/// ```
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Channel {
    pub cid: i64,
    pub channel_name: String,
    pub clients: Vec<Client>,
}

impl Default for Channel {
    fn default() -> Channel {
        Channel {
            cid: 0,
            channel_name: String::new(),
            clients: Vec::new(),
        }
    }
}

impl Channel {
    pub fn new(channel_id: i64, name: String) -> Channel {
        let mut channel = Channel::default();
        channel.cid = channel_id;
        channel.channel_name = name;
        channel
    }

    pub fn from_map(map: &StringMap) -> Channel {
        let mut channel = Channel::default();
        channel.mut_from_map(map);
        channel
    }

    pub fn update_from_map(channel: Channel, map: &StringMap) -> Channel {
        let mut channel = channel.clone();
        channel.mut_from_map(map);
        channel
    }

    pub fn mut_from_map(&mut self, map: &StringMap) {
        update_from_map(map, "cid", &mut self.cid);
        update_from_map(map, "channel_name", &mut self.channel_name);
    }

    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    pub fn clients_len(&self) -> usize {
        self.clients.len()
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

impl FromStr for Channel {
    type Err = super::Error;
    fn from_str(s: &str) -> super::Result<Self> {
        let map = to_map(s);
        Ok(Channel::from_map(&map))
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.channel_name));
        if !self.is_empty() {
            for client in self.clients.iter() {
                try!(write!(f, "\n  {}", client));
            }
        };
        Ok(())
    }
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.cid.eq(&other.cid)
    }
}

impl Eq for Channel {}

impl PartialOrd for Channel {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cid.partial_cmp(&other.cid)
    }
}

impl Ord for Channel {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.cid.cmp(&other.cid)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, RustcDecodable, RustcEncodable)]
pub struct ChannelList(Vec<Channel>);

impl ChannelList {
    pub fn vec(&self) -> &Vec<Channel> {
        let &ChannelList(ref v) = self;
        v
    }

    pub fn get_mut(&mut self) -> &mut Vec<Channel> {
        let &mut ChannelList(ref mut v) = self;
        v
    }

    pub fn get_vec(self) -> Vec<Channel> {
        let ChannelList(v) = self;
        v
    }

    pub fn from_maps(maps: &Vec<StringMap>) -> ChannelList {
        let mut vec = Vec::new();
        for map in maps.iter() {
            let channel = Channel::from_map(map);
            vec.push(channel);
        }
        ChannelList(vec)
    }

    pub fn insert_client(&mut self, client: &Client) {
        let cid = client.cid;
        for channel in self.get_mut().iter_mut() {
            if channel.cid == cid {
                channel.add_client(client.clone());
            }
        }
    }

    pub fn merge_clients(&mut self, clients: &ClientList) {
        for client in clients.iter() {
            self.insert_client(client);
        }
    }
}

impl Deref for ChannelList {
    type Target = Vec<Channel>;
    fn deref(&self) -> &Vec<Channel> {
        self.vec()
    }
}

impl FromStr for ChannelList {
    type Err = super::Error;
    fn from_str(s: &str) -> super::Result<Self> {
        let maps = s.split('|').map(to_map).collect();
        Ok(ChannelList::from_maps(&maps))
    }
}
