// mod command

/// A trait for a Command, that can be send to a server.
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
