use std::io;

use super::resource::ResourceCollection;

pub trait Input {
    fn from_reader(mut reader: Box<io::Read>) -> Result<ResourceCollection, &'static str>;
}

pub trait Output {
    fn to_file();
}