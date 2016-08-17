use std::io;

pub trait Import {
    fn from_reader(mut reader: Box<io::Read>);
}