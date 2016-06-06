extern crate sqlib;
extern crate hyper;

use hyper::server::{Server, Request, Response, Handler};
use hyper::method::Method;
use hyper::status::StatusCode;
use hyper::header::ContentLength;
use std::io::Write;
use std::time;
use std::thread;
use std::sync::{Arc, Mutex};
use std::env;
use sqlib::*;

fn handle(channels: Arc<Mutex<ChannelList>>, req: Request, mut res: Response) {
    match req.method {
        Method::Get => {
            let body: String;
            {
                let channels = channels.lock().unwrap();
                body = channels.as_json();
            }
            res.headers_mut().set(ContentLength(body.len() as u64));
            let mut res = res.start().unwrap();
            // res.write_all(body).unwrap();
            let _ = write!(res, "{}", body);

            // println!("{:?}", req.headers);
        }
        _ => {
            println!("{:?}", req.method);
            send_error(res);
        }
    }
}

fn send_error(mut res: Response) {
    let body = "Error: Internal Server Error";
    res.headers_mut().set(ContentLength(body.len() as u64));
    *res.status_mut() = StatusCode::InternalServerError;

    let mut res = res.start().unwrap();

    let _ = write!(res, "{}", body);
}

fn get_channels_interval(mut conn: Connection,
                         channels: Arc<Mutex<ChannelList>>,
                         secs: u64)
                         -> sqlib::Result<()> {
    loop {
        let new_channellist = try!(conn.channellist_with_clients());
        {
            let mut cls = try!(channels.lock());
            *cls = new_channellist;
        }
        thread::sleep(time::Duration::from_secs(secs));
    }
}

struct AMCL {
    cl: Arc<Mutex<ChannelList>>,
}

impl Handler for AMCL {
    fn handle(&self, req: Request, res: Response) {
        handle(self.cl.clone(), req, res);
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap(); // program name
    let login_name = args.next().unwrap();
    let password = args.next().unwrap();
    let nickname = args.next().unwrap();
    let server_id = 1; // args.next().unwrap().parse().unwrap();

    let channels = Arc::new(Mutex::new(sqlib::ChannelList::default()));
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
