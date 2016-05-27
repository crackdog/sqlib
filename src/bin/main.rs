extern crate sqlib;

use sqlib::{Connection, ChannelList, ClientList};
use sqlib::map::*;

fn main() {
    let mut conn = Connection::new("127.0.0.1:10011".to_string()).unwrap();
    conn.send_command("use 1").unwrap();
    conn.send_command("login serveradmin hqjI+wn7").unwrap();
    let mut clients = conn.send_command("clientlist").unwrap().parse::<ClientList>().unwrap();

    for client in clients.get_mut().iter_mut() {
        let command = format!("clientinfo clid={}", client.clid);
        let str = conn.send_command(command).unwrap();
        let map = to_map(&str);
        client.mut_from_map(&map);
    }

    let mut channels = conn.send_command("channellist").unwrap().parse::<ChannelList>().unwrap();
    channels.merge_clients(&clients);
    for channel in channels.iter() {
        println!("{}", channel);
    }
    let _ = conn.quit();
}
