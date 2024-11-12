use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Website to query
    pub website: String,

    /// Output file
    /// If not provided, the output will be printed to stdout
    #[arg(short, long)]
    pub output: Option<String>,
}
