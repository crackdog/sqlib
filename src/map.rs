// mod map

use std::collections::HashMap;
use std::str::FromStr;

pub type StringMap = HashMap<String, String>;

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

pub fn update_from_map<T>(map: &StringMap, key: &str, value: &mut T)
    where T: FromStr
{
    if let Some(v) = map.get(key) {
        let r: Result<T, _> = v.parse();
        match r {
            Ok(v) => {
                *value = v;
            }
            Err(_) => {}
        }
    }
}
