extern crate sqlib;
extern crate hyper;

use hyper::server::{Server, Request, Response};
use std::io::Write;
use hyper::header::ContentLength;

fn handle(req: Request, mut res: Response) {
    let body = "Hello World!";
    res.headers_mut().set(ContentLength(body.len() as u64));
    let mut res = res.start().unwrap();
    // res.write_all(body).unwrap();
    write!(req, "{:?}", body);

    println!("{:?}", req.remote_addr);
    println!("{:?}", req.uri);
}

fn main() {
    Server::http("0.0.0.0:8080").unwrap().handle(handle).unwrap();
}

// use sqlib::Connection;
//
// fn main() {
//    let mut conn = Connection::new("127.0.0.1:10011".to_string()).unwrap();
//    conn.send_command("use 1").unwrap();
//    conn.send_command("login serveradmin hqjI+wn7").unwrap();
//
//    let channels = conn.channellist_with_clients().unwrap();
//
//    for channel in channels.iter() {
//        if !channel.is_empty() {
//            println!("{}", channel);
//        }
//    }
//
//    println!("{}", channels.as_json());
//    let _ = conn.quit();
// }
