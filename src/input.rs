use std::io;

pub trait Reader {
    fn input_from_reader(mut reader: Box<io::Read>);
}