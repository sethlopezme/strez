use std::io;

pub trait Deserialize {
    fn from_reader(mut reader: Box<io::Read>);
}

pub trait Serialize {
    fn to_file();
}