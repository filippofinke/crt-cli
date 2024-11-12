use std::io::{self, Write};

use crate::structs::certificate::Certificate;

use super::output::Output;
pub struct OutputCSV {
    writer: Box<dyn Write>,
}

impl Output for OutputCSV {
    fn new(writer: Box<dyn Write>) -> Self {
        OutputCSV { writer }
    }
    fn write_output(&mut self, certificates: &[Certificate]) -> io::Result<()> {
        let mut wtr = csv::Writer::from_writer(self.writer.by_ref());
        for cert in certificates {
            wtr.serialize(cert)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
