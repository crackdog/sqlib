// mod client

use rustc_serialize::json;
use std::fmt;
use std::cmp;
use std::ops::Deref;
use std::str::FromStr;
use map::*;
use escaping::*;

/// Client contains information about a TeamSpeak 3 client.
/// # Example
/// ```
/// use sqlib;
///
/// let client = sqlib::Client {
///     clid: 1,
///     cid: 1,
///     client_database_id: 1,
///     client_nickname: "John Doe".to_string(),
///     client_type: 0,
///     connection_connected_time: 0
/// };
///
/// assert!(client.is_client());
///
/// let client_print = format!("{}", client);
///
/// assert_eq!("John Doe (0)".to_string(), client_print);
/// ```
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Client {
    /// client id
    pub clid: i64,
    /// channel id
    pub cid: i64,
    /// client database id
    pub client_database_id: i64,
    /// client nickname
    pub client_nickname: String,
    /// client type: 0 is client and 1 is query client
    pub client_type: i64,
    /// connection time in milliseconds
    pub connection_connected_time: i64,
}

impl Default for Client {
    fn default() -> Client {
        Client {
            clid: 0,
            cid: 0,
            client_database_id: 0,
            client_nickname: String::new(),
            client_type: 0,
            connection_connected_time: 0,
        }
    }
}

impl Client {
    /// creates a new client from a client id and a nickname
    pub fn new(client_id: i64, nickname: String) -> Client {
        let mut client = Client::default();
        client.clid = client_id;
        client.client_nickname = nickname;
        client.unescape();
        client
    }
    /// checks if it is a real client
    pub fn is_client(&self) -> bool {
        self.client_type == 0
    }

    fn unescape(&mut self) {
        self.client_nickname = unescape(&self.client_nickname);
    }

    pub fn from_map(map: &StringMap) -> Client {
        let mut client = Client::default();
        client.mut_from_map(map);
        client
    }

    pub fn update_from_map(client: Client, map: &StringMap) -> Client {
        let mut client = client.clone();
        client.mut_from_map(map);
        client
    }

    pub fn mut_from_map(&mut self, map: &StringMap) {
        update_from_map(map, "clid", &mut self.clid);
        update_from_map(map, "cid", &mut self.cid);
        update_from_map(map, "client_database_id", &mut self.client_database_id);
        update_from_map(map, "client_nickname", &mut self.client_nickname);
        update_from_map(map, "client_type", &mut self.client_type);
        update_from_map(map,
                        "connection_connected_time",
                        &mut self.connection_connected_time);
        self.unescape();
    }

    fn connection_connected_time_string(&self) -> String {
        let mut raw_seconds = self.connection_connected_time / 1000;
        let hours = raw_seconds / 3600;
        raw_seconds = raw_seconds % 3600;
        let minutes = raw_seconds / 60;
        let seconds = raw_seconds % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{:02}:{:02}", minutes, seconds)
        } else {
            format!("{}", seconds)
        }
    }

    pub fn as_json(&self) -> String {
        json::encode(self).unwrap_or(String::new())
    }
}

impl FromStr for Client {
    type Err = super::Error;
    fn from_str(s: &str) -> super::Result<Self> {
        let map = to_map(s);
        Ok(Client::from_map(&map))
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} ({})",
               &self.client_nickname,
               self.connection_connected_time_string())
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Client) -> bool {
        self.clid.eq(&other.clid)
    }
}

impl Eq for Client {}

impl PartialOrd for Client {
    fn partial_cmp(&self, other: &Client) -> Option<cmp::Ordering> {
        self.clid.partial_cmp(&other.clid)
    }
}

impl Ord for Client {
    fn cmp(&self, other: &Client) -> cmp::Ordering {
        self.clid.cmp(&other.clid)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, RustcDecodable, RustcEncodable)]
pub struct ClientList(Vec<Client>);

impl ClientList {
    pub fn vec(&self) -> &Vec<Client> {
        let &ClientList(ref v) = self;
        v
    }

    pub fn get_mut(&mut self) -> &mut Vec<Client> {
        let &mut ClientList(ref mut v) = self;
        v
    }

    pub fn get_vec(self) -> Vec<Client> {
        let ClientList(v) = self;
        v
    }

    pub fn filter_clients(&self) -> ClientList {
        let new_vec = self.iter().map(Clone::clone).filter(|c| c.is_client()).collect();
        ClientList(new_vec)
    }

    pub fn from_maps(maps: &Vec<StringMap>) -> ClientList {
        let mut vec = Vec::new();
        for map in maps.iter() {
            let client = Client::from_map(map);
            vec.push(client);
        }
        ClientList(vec)
    }

    pub fn as_json(&self) -> String {
        json::encode(self.vec()).unwrap_or(String::new())
    }
}

impl FromStr for ClientList {
    type Err = super::Error;
    fn from_str(s: &str) -> super::Result<Self> {
        let maps = s.split('|').map(to_map).collect();
        Ok(ClientList::from_maps(&maps))
    }
}

impl Deref for ClientList {
    type Target = Vec<Client>;
    fn deref(&self) -> &Vec<Client> {
        self.vec()
    }
}

impl fmt::Display for ClientList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "[ "));
        let mut cls = self.vec().iter();
        if cls.len() > 0 {
            try!(write!(f, "{}", cls.next().unwrap()));
        }
        for client in cls {
            try!(write!(f, ", {}", client));
        }
        write!(f, " ]")
    }
}
