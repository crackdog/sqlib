extern crate sqlib;

use sqlib::Connection;

fn main() {
    let mut conn = Connection::new("127.0.0.1:10011".to_string()).unwrap();
    let str = conn.send_command("help login").unwrap();
    println!("{}", str);
    let _ = conn.quit();
}
