use std::io::{self, Write};

use crate::structs::certificate::Certificate;

use super::output::Output;
pub struct OutputJSON {
    writer: Box<dyn Write>,
}

impl Output for OutputJSON {
    fn new(writer: Box<dyn Write>) -> Self {
        OutputJSON { writer }
    }

    fn write_output(&mut self, certificates: &[Certificate]) -> io::Result<()> {
        let json = serde_json::to_string_pretty(certificates)?;
        writeln!(self.writer, "{}", json)?;
        Ok(())
    }
}
