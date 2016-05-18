// mod client
use std::fmt;

/// Client contains information about a TeamSpeak 3 client.
/// # Example
/// ```
/// use ts3_online::sqlib;
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
/// assert_eq!("John Doe".to_string(), client_print);
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
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

impl Client {
    /// checks if it is a real client
    pub fn is_client(&self) -> bool {
        self.client_type == 0
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.client_nickname)
    }
}
