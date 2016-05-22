// mod connection

use std::fmt;
use std::net;
use std::string::String;
use std::io::{self, BufReader, Error, ErrorKind};
use std::io::prelude::*;

#[derive(Debug)]
pub struct Connection {
    addr: net::SocketAddrV4,
    conn: BufReader<net::TcpStream>,
}

impl Connection {
    pub fn new(addr: String) -> io::Result<Connection> {
        let a = addr.parse().unwrap();
        let c = try!(net::TcpStream::connect(a));
        Ok(Connection {
            addr: a,
            conn: BufReader::new(c),
        })
    }

    fn get_stream_mut(&mut self) -> &mut net::TcpStream {
        self.conn.get_mut()
    }

    pub fn send_command<C: Command>(&mut self, cmd: C) -> Result<String, io::Error> {
        let cmd = cmd.string();
        if cmd.is_empty() {
            return Err(Error::new(ErrorKind::Other, "no command"));
        }

        try!(writeln!(self.get_stream_mut(), "{}", cmd));

        try!(self.get_stream_mut().flush());

        let mut line = String::new();
        let mut result = String::new();
        loop {
            try!(self.conn.read_line(&mut line));
            result = line + "\n";
            break;
        }

        Ok(result)
    }

    pub fn quit(&mut self) -> io::Result<()> {
        try!(self.send_command("quit").map(|_| ()));
        self.conn.get_ref().shutdown(net::Shutdown::Both)
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.addr)
    }
}

pub trait Command {
    fn string(&self) -> String;
}

impl<'a> Command for &'a str {
    fn string(&self) -> String {
        self.to_string()
    }
}

impl Command for String {
    fn string(&self) -> String {
        self.clone()
    }
}
