//! escaping provides functions for the Server Query escaping.
//!
//! # Example
//! ```
//! use sqlib::escaping::{escape, unescape};
//!
//! let unescaped = "hello world/|\\".to_string();
//! let escaped = "hello\\sworld\\/\\p\\\\".to_string();
//!
//! let escaped_test = escape(&unescaped);
//! let unescaped_test = unescape(&escaped);
//!
//! assert_eq!(escaped_test, escaped);
//! assert_eq!(unescaped_test, unescaped);
//! ```

const ESCAPE_CHARS: [(char, &'static str); 11] = [('\\', r"\\"),
                                                  (' ', r"\s"),
                                                  ('/', r"\/"),
                                                  ('|', r"\p"),
                                                  (7 as char, r"\a"),
                                                  (8 as char, r"\b"),
                                                  (9 as char, r"\t"),
                                                  (10 as char, r"\n"),
                                                  (11 as char, r"\v"),
                                                  (12 as char, r"\f"),
                                                  (13 as char, r"\r")];

/// escapes all chars described in the server query manual
///
/// # Example
/// ```
/// use sqlib::escaping::escape;
///
/// let unescaped = "hello world/|\\".to_string();
/// let escaped = "hello\\sworld\\/\\p\\\\".to_string();
///
/// let s = escape(&unescaped);
///
/// assert_eq!(s, escaped);
/// ```
pub fn escape(s: &str) -> String {
    let mut new_string = s.to_string();
    for &(from, to) in ESCAPE_CHARS.iter() {
        new_string = new_string.replace(from, to);
    }
    new_string
}

/// unescapes all chars described in the server query manual
///
/// # Example
/// ```
/// use sqlib::escaping::unescape;
///
/// let unescaped = "hello world/|\\".to_string();
/// let escaped = "hello\\sworld\\/\\p\\\\".to_string();
///
/// let s = unescape(&escaped);
///
/// assert_eq!(s, unescaped);
/// ```
pub fn unescape(s: &str) -> String {
    let mut new_string = s.to_string();
    for &(to, from) in ESCAPE_CHARS.iter() {
        new_string = new_string.replace(from, &to.to_string());
    }
    new_string
}
