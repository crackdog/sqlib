//! map provides functions around StringMap (HashMap<String, String>).
//!
//! # Example
//! ```
//! use sqlib::map::{to_map, update_from_map};
//!
//! let map = to_map("key1=2");
//! let mut integer = 1i32;
//!
//! update_from_map(&map, "key1", &mut integer);
//!
//! assert_eq!(integer, 2);
//! ```

use std::collections::HashMap;
use std::str::FromStr;

/// A small newtype for a HashMap of Strings.
pub type StringMap = HashMap<String, String>;

/// creates a new StringMap from a &str.
///
/// # Example
/// ```
/// use sqlib::map::to_map;
///
/// let string = " key1=value1 key2=value2 ";
/// let map = to_map(string);
///
/// assert_eq!(map.get("key1").unwrap(), "value1");
/// assert_eq!(map.get("key2").unwrap(), "value2");
/// ```
pub fn to_map(string: &str) -> StringMap {
    let pair_seperator = char::is_whitespace;
    let key_value_seperator = '=';
    let mut map = HashMap::new();

    let pairs = string.split(pair_seperator);
    for pair in pairs {
        let kv: Vec<&str> = pair.splitn(2, key_value_seperator).collect();
        if kv.len() < 2 {
            continue;
        }
        let key = kv[0].to_string();
        let value = kv[1].to_string();
        map.insert(key, value);
    }

    map
}

/// This function gets the value to the key from the map, then parses it and mutates the given
/// pointer.
///
/// # Example
/// ```
/// use sqlib::map::{to_map, update_from_map};
///
/// let map = to_map("key1=2");
/// let mut integer = 1i32;
///
/// update_from_map(&map, "key1", &mut integer);
///
/// assert_eq!(integer, 2);
/// ```
pub fn update_from_map<T>(map: &StringMap, key: &str, value: &mut T)
where
    T: FromStr,
{
    if let Some(v) = map.get(key) {
        let r: Result<T, _> = v.parse();
        if let Ok(v) = r {
            *value = v;
        }
    }
}
