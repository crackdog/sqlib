extern crate sqlib;
extern crate hyper;

use hyper::server::{Server, Request, Response, Handler};
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::header::ContentLength;
use std::io::Write;
use std::time;
use std::thread;
use std::sync::{Arc, RwLock};
use std::env;
use sqlib::prelude::*;

fn handle(channels: Arc<RwLock<String>>, req: Request, mut res: Response) {
    match req.method {
        Method::Get => {
            let body: String;
            {
                let channels = channels.read().unwrap();
                body = channels.clone();
            }
            res.headers_mut().set(ContentLength(body.len() as u64));
            let mut res = res.start().unwrap();
            // res.write_all(body).unwrap();
            let _ = write!(res, "{}\r\n", body);

            // println!("{:?}", req.headers);
        }
        _ => {
            println!("{:?}", req.method);
            send_error(res, StatusCode::MethodNotAllowed);
        }
    }
}

fn send_error(mut res: Response, code: StatusCode) {
    let body = match code.canonical_reason() {
        Some(reason) => format!("Error: {}", reason),
        None => "An Error happened!".to_string(),
    };
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = code;

    let mut res = res.start().unwrap();

    let _ = write!(res, "{}\r\n", body);
}

fn get_channels_interval(mut conn: Connection,
                         channels: Arc<RwLock<String>>,
                         secs: u64)
                         -> Result<()> {
    loop {
        let new_channellist = try!(conn.channellist_with_clients());
        let channellist_str = new_channellist.as_json();
        {
            let mut cls = try!(channels.write());
            *cls = channellist_str;
        }
        thread::sleep(time::Duration::from_secs(secs));
    }
}

struct AMCL {
    cl: Arc<RwLock<String>>,
}

impl Handler for AMCL {
    fn handle(&self, req: Request, res: Response) {
        handle(self.cl.clone(), req, res);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let login_name = args[1].clone();
    let password = args[2].clone();
    let nickname = match args.get(3) {
        Some(s) => s.clone(),
        None => "ts3-online".to_string(),
    };
    let server_id = 1; // args.next().unwrap().parse().unwrap();

    let channels = Arc::new(RwLock::new(String::new()));
    let handler_channels = AMCL { cl: channels.clone() };

    let mut conn = Connection::new("127.0.0.1:10011").unwrap();
    conn.use_server_id(server_id).unwrap();
    conn.login(&login_name, &password).unwrap();
    conn.change_nickname(&nickname).unwrap();

    let channel_thread = thread::spawn(move || get_channels_interval(conn, channels, 5));
    let _ = thread::spawn(move || {
        Server::http("127.0.0.1:8080")
            .unwrap()
            .handle(handler_channels)
            .unwrap();
    });
    println!("server started");
    let err = channel_thread.join();
    println!("{:?}", err);
}
