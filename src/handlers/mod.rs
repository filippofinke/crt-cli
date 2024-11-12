use std::collections::HashMap;
use std::fs::File;
use std::io;

use crate::outputs::output::Output;
use crate::outputs::output_csv::OutputCSV;
use crate::outputs::output_json::OutputJSON;
use crate::outputs::output_stdout::OutputStdout;

pub fn create_output_writer(output_file: Option<String>) -> Box<dyn Output> {
    match output_file {
        Some(file) => create_file_output(file),
        None => Box::new(OutputStdout::new(Box::new(io::stdout()))),
    }
}

fn create_file_output(file: String) -> Box<dyn Output> {
    let extension = file.split('.').last().unwrap_or_default();

    // Map file extensions to respective lambda functions
    let mut handlers: HashMap<&str, Box<dyn Fn(Box<dyn io::Write>) -> Box<dyn Output>>> =
        HashMap::new();
    handlers.insert("json", Box::new(|w| Box::new(OutputJSON::new(w))));
    handlers.insert("csv", Box::new(|w| Box::new(OutputCSV::new(w))));

    // Try to get the handler for the extension, or panic if not found
    handlers
        .get(extension)
        .map(|handler| {
            let file = File::create(&file).expect("Failed to create file");
            handler(Box::new(file))
        })
        .unwrap_or_else(|| {
            // List of supported output formats
            let supported_formats = handlers
                .keys()
                .map(|k| *k)
                .collect::<Vec<&str>>()
                .join(", ");
            panic!(
                "Unsupported output format: {}. Supported formats: {}",
                extension, supported_formats
            );
        })
}
