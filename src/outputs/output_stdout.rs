use std::io::{self, Write};

use crate::structs::certificate::Certificate;

use super::output::Output;

pub struct OutputStdout {
    writer: Box<dyn Write>,
}

impl Output for OutputStdout {
    fn new(writer: Box<dyn Write>) -> Self {
        OutputStdout { writer }
    }

    fn write_output(&mut self, certificates: &[Certificate]) -> io::Result<()> {
        for cert in certificates {
            writeln!(self.writer, "Issuer CA ID: {}", cert.issuer_ca_id)?;
            writeln!(self.writer, "Issuer Name: {}", cert.issuer_name)?;
            writeln!(self.writer, "Common Name: {}", cert.common_name)?;
            writeln!(self.writer, "Name Value: {}", cert.name_value)?;
            writeln!(self.writer, "ID: {}", cert.id)?;
            writeln!(self.writer, "Entry Timestamp: {}", cert.entry_timestamp)?;
            writeln!(self.writer, "Not Before: {}", cert.not_before)?;
            writeln!(self.writer, "Not After: {}", cert.not_after)?;
            writeln!(self.writer, "Serial Number: {}", cert.serial_number)?;
            writeln!(self.writer, "Result Count: {}", cert.result_count)?;
            writeln!(self.writer, "-----------------------------------")?;
        }
        Ok(())
    }
}
