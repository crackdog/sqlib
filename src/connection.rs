//! The connection module contains the Connection struct, that provides an interface for a Server
//! Query connection.

use std::fmt;
use std::net;
use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use error::{Error, SQError};
use client::ClientList;
use channel::ChannelList;
use command::Command;
use map::*;
use error;

/// Connection provides an interface for a Server Query connection.
#[derive(Debug)]
pub struct Connection {
    addr: net::SocketAddrV4,
    conn: BufReader<net::TcpStream>,
}

impl Connection {
    /// creates a new Connection from an adress given as a string reference.
    pub fn new(addr: &str) -> error::Result<Connection> {
        let addr = addr.to_string();
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

    fn read_line<'a>(&mut self, buf: &'a mut String) -> error::Result<&'a str> {
        let _ = try!(self.conn.read_line(buf));
        Ok(buf.trim_left_matches(char::is_control))
    }

    fn get_stream_mut(&mut self) -> &mut net::TcpStream {
        self.conn.get_mut()
    }

    /// sends a given command to the Server Query server and returns the answer as a String, or
    /// the error.
    pub fn send_command<C>(&mut self, command: C) -> error::Result<String>
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

    pub fn send_command_to_map<C>(&mut self, command: C) -> error::Result<StringMap>
        where C: Command
    {
        let result = try!(self.send_command(command));
        Ok(to_map(&result))
    }

    pub fn send_command_vec<C>(&mut self, commands: C) -> error::Result<Vec<String>>
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

    /// sends the quit command to the server and shuts the Connection down.
    pub fn quit(&mut self) -> error::Result<()> {
        try!(self.send_command("quit"));
        try!(self.conn.get_ref().shutdown(net::Shutdown::Both));
        Ok(())
    }

    /// sends the use command with the given id to the server.
    pub fn use_server_id(&mut self, id: u64) -> error::Result<()> {
        self.send_command(format!("use {}", id)).map(|_| ())
    }

    /// sends the login command with the name and password to the server.
    pub fn login(&mut self, name: &str, pw: &str) -> error::Result<()> {
        self.send_command(format!("login {} {}", name, pw)).map(|_| ())
    }

    /// tries to change the nickname of the Server Query client.
    pub fn change_nickname(&mut self, nickname: &str) -> error::Result<()> {
        let map = try!(self.send_command_to_map("whoami"));
        let id = try!(map.get("client_id").ok_or("error at collecting client_id"));
        let cmd = format!("clientupdate clid={} client_nickname={}", id, nickname);
        let _ = try!(self.send_command(cmd));
        Ok(())
    }

    /// sends the clientlist command to the server and parses the result.
    pub fn clientlist(&mut self) -> error::Result<ClientList> {
        let s = try!(self.send_command("clientlist"));
        let cl = try!(s.parse());
        Ok(cl)
    }

    /// # common errors
    /// If a client disconnects between the getting of the clientlist and the getting of the client
    /// information, then there will be an error 512, because the client id is invalid.
    pub fn clientlist_with_info(&mut self) -> error::Result<ClientList> {
        let mut clients = try!(self.clientlist());
        for client in clients.as_mut().iter_mut() {
            let command = format!("clientinfo clid={}", client.clid);
            let str = try!(self.send_command(command));
            let map = to_map(&str);
            client.mut_from_map(&map);
        }
        Ok(clients)
    }

    /// sends the channellist command to the server and parses the result.
    pub fn channellist(&mut self) -> error::Result<ChannelList> {
        let s = try!(self.send_command("channellist"));
        let cl = try!(s.parse());
        Ok(cl)
    }

    /// # common errors
    /// If a client disconnects between the getting of the clientlist and the getting of the client
    /// information, then there will be an error 512, because the client id is invalid.
    pub fn channellist_with_clients(&mut self) -> error::Result<ChannelList> {
        let clients = try!(self.clientlist_with_info());
        let mut channels = try!(self.channellist());
        channels.merge_clients(&clients);
        Ok(channels)
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.addr)
    }
}
