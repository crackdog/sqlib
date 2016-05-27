// mod command

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
