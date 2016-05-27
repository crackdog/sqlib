// mod connection

use std::fmt;
use std::net;
use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use error::{Error, SQError};
use command::Command;

#[derive(Debug)]
pub struct Connection {
    addr: net::SocketAddrV4,
    conn: BufReader<net::TcpStream>,
}

impl Connection {
    pub fn new(addr: String) -> super::Result<Connection> {
        let a = try!(addr.parse());
        let c = try!(net::TcpStream::connect(a));
        let mut connection = Connection {
            addr: a,
            conn: BufReader::new(c),
        };
        let mut tmp = String::new();
        try!(connection.conn.read_line(&mut tmp));
        if tmp.trim() != "TS3" {
            return Err(From::from("the given server is not a TS3 server"));
        }
        try!(connection.read_line(&mut tmp));
        Ok(connection)
    }

    fn read_line<'a>(&mut self, buf: &'a mut String) -> super::Result<&'a str> {
        let _ = try!(self.conn.read_line(buf));
        Ok(buf.trim_left_matches(char::is_control))
    }

    fn get_stream_mut(&mut self) -> &mut net::TcpStream {
        self.conn.get_mut()
    }

    pub fn send_command<C>(&mut self, command: C) -> super::Result<String>
        where C: Command
    {
        let command = command.string();
        if command.is_empty() {
            return Err(Error::from("no command"));
        }

        try!(writeln!(self.get_stream_mut(), "{}", command));

        try!(self.get_stream_mut().flush());

        let mut result = String::new();
        loop {
            let mut line = String::new();
            let line = try!(self.read_line(&mut line));
            let ok = try!(SQError::parse_is_ok(line));
            if ok {
                break;
            }
            result = result + line; // + "\n";
        }

        Ok(result)
    }

    pub fn send_command_vec<C>(&mut self, commands: C) -> super::Result<Vec<String>>
        where C: IntoIterator,
              C::Item: Command
    {
        let mut results = Vec::new();
        for cmd in commands {
            let res = try!(self.send_command(cmd));
            results.push(res);
        }
        Ok(results)
    }

    pub fn quit(&mut self) -> super::Result<()> {
        try!(writeln!(self.get_stream_mut(), "quit"));
        try!(self.get_stream_mut().flush());
        try!(self.conn.get_ref().shutdown(net::Shutdown::Both));
        Ok(())
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.addr)
    }
}
