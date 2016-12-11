use std::io;

use Result;

pub struct CSV;

impl super::Format for CSV {
    fn from_reader<R: io::Read>(r: R) -> Result<CSV> {
        Ok(CSV)
    }
}