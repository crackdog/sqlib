// mod escaping

const ESCAPE_CHARS: [(char, &'static str); 11] = [('\\', "\\\\"),
                                                  (' ', "\\s"),
                                                  ('/', "\\/"),
                                                  ('|', "\\p"),
                                                  (7 as char, "\\a"),
                                                  (8 as char, "\\b"),
                                                  (9 as char, "\\t"),
                                                  (10 as char, "\\n"),
                                                  (11 as char, "\\v"),
                                                  (12 as char, "\\f"),
                                                  (13 as char, "\\r")];

/// escapes all chars described in the server query manual
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
