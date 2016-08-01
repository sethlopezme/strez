use std::io;
use super::input::Reader;

pub struct CSV;

impl Reader for CSV {
    fn input_from_reader(mut reader: Box<io::Read>) {
        // write the input to stdout
        io::copy(&mut reader, &mut io::stdout());
    }
}