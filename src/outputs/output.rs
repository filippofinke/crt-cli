use std::io;

use crate::structs::certificate::Certificate;

pub trait Output {
    fn write_output(&mut self, certificates: &[Certificate]) -> io::Result<()>;
    fn new(output: Box<dyn io::Write>) -> Self
    where
        Self: Sized;
}
