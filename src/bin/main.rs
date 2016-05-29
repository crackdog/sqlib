extern crate sqlib;

use sqlib::Connection;

fn main() {
    let mut conn = Connection::new("127.0.0.1:10011".to_string()).unwrap();
    conn.send_command("use 1").unwrap();
    conn.send_command("login serveradmin hqjI+wn7").unwrap();

    let channels = conn.channellist_with_clients().unwrap();

    for channel in channels.iter() {
        if !channel.is_empty() {
            println!("{}", channel);
        }
    }
    let _ = conn.quit();
}
