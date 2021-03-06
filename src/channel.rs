//! The channel module contains the Channel and ChannelList structs.
//!
//! The are representations of a TS3 channel and channellist.

use client::{Client, ClientList};
use error;
use escaping::*;
use map::*;
use rustc_serialize::json;
use std::cmp;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

/// Channel contains a TeamSpeak 3 channel.
/// # Example
/// ```
/// use sqlib::channel;
///
/// let channel = sqlib::channel::Channel::new(0, "test".to_string());
///
/// assert!(channel.is_empty());
/// assert_eq!("test".to_string(), format!("{}", channel));
/// ```
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Channel {
    /// channel id
    pub cid: i64,
    /// channel name
    pub channel_name: String,
    /// A vector of clients, who are in the channel.
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
    /// Create an empty Channel from a channel id and a name.
    pub fn new(channel_id: i64, name: String) -> Channel {
        let mut channel = Channel::default();
        channel.cid = channel_id;
        channel.channel_name = name;
        channel.unescape();
        channel
    }

    fn unescape(&mut self) {
        self.channel_name = unescape(&self.channel_name);
    }

    /// Create a new Channel from a given map.
    pub fn from_map(map: &StringMap) -> Channel {
        let mut channel = Channel::default();
        channel.mut_from_map(map);
        channel
    }

    /// Create a new Channel from a given Channel and a map.
    pub fn update_from_map(channel: &Channel, map: &StringMap) -> Channel {
        let mut channel = channel.clone();
        channel.mut_from_map(map);
        channel
    }

    /// Mutate self from a given map.
    pub fn mut_from_map(&mut self, map: &StringMap) {
        update_from_map(map, "cid", &mut self.cid);
        update_from_map(map, "channel_name", &mut self.channel_name);
        self.unescape();
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

    /// Remove all Server Query Clients from the Channel.
    pub fn remove_sq_clients(&mut self) {
        let new_clients = self
            .clients
            .iter()
            .filter(|c| c.is_client())
            .cloned()
            .collect();
        self.clients = new_clients;
    }

    /// Creates a JSON String from self.
    pub fn as_json(&self) -> String {
        json::encode(self).unwrap_or_default()
    }
}

impl FromStr for Channel {
    type Err = error::Error;
    fn from_str(s: &str) -> error::Result<Self> {
        let map = to_map(s);
        Ok(Channel::from_map(&map))
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.channel_name)?;
        if !self.is_empty() {
            for client in &self.clients {
                write!(f, "\n  {}", client)?;
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

/// ChannelList contains a list of Channels.
///
/// # Example
/// ```
/// use sqlib::channel::{Channel, ChannelList};
///
/// let channel1 = Channel::new(1, "test1".to_string());
/// let channel2 = Channel::new(2, "test2".to_string());
///
/// let channels = vec![channel1, channel2];
/// let channellist = ChannelList::from(channels.clone());
///
/// assert_eq!(&channels, channellist.as_ref());
///
/// let channels2: Vec<_> = channellist.into();
/// assert_eq!(channels, channels2);
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, RustcDecodable, RustcEncodable)]
pub struct ChannelList(Vec<Channel>);

impl Default for ChannelList {
    fn default() -> ChannelList {
        ChannelList(Vec::new())
    }
}

impl ChannelList {
    /// Converts a ChannelList into a vector of Channel's.
    pub fn into_inner(self) -> Vec<Channel> {
        let ChannelList(channel) = self;
        channel
    }

    /// creates a ChannelList from a Vec of StringMaps
    pub fn from_maps(maps: &[StringMap]) -> ChannelList {
        let mut vec = Vec::new();
        for map in maps.iter() {
            let channel = Channel::from_map(map);
            vec.push(channel);
        }
        ChannelList(vec)
    }

    pub fn insert_client(&mut self, client: &Client) {
        let cid = client.cid;
        for channel in self.as_mut().iter_mut() {
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

    /// creates a JSON String from a ChannelList
    pub fn as_json(&self) -> String {
        json::encode(self.as_ref()).unwrap_or_default()
    }
}

impl Deref for ChannelList {
    type Target = Vec<Channel>;
    fn deref(&self) -> &Vec<Channel> {
        self.as_ref()
    }
}

impl AsRef<Vec<Channel>> for ChannelList {
    fn as_ref(&self) -> &Vec<Channel> {
        let &ChannelList(ref channel) = self;
        channel
    }
}

impl AsMut<Vec<Channel>> for ChannelList {
    fn as_mut(&mut self) -> &mut Vec<Channel> {
        let &mut ChannelList(ref mut channel) = self;
        channel
    }
}

impl FromStr for ChannelList {
    type Err = error::Error;
    fn from_str(s: &str) -> error::Result<Self> {
        let maps: Vec<_> = s.split('|').map(to_map).collect();
        Ok(ChannelList::from_maps(&maps))
    }
}

impl From<Vec<Channel>> for ChannelList {
    fn from(channels: Vec<Channel>) -> Self {
        ChannelList(channels)
    }
}

impl From<ChannelList> for Vec<Channel> {
    fn from(channels: ChannelList) -> Self {
        channels.into_inner()
    }
}
