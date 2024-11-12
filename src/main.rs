mod apis;
mod handlers;
mod outputs;
mod structs;

use apis::api::CertificateFetcher;
use apis::crt_sh::CrtShFetcher;
use clap::Parser;
use handlers::create_output_writer;
use structs::args::Args;

fn main() {
    let args = Args::parse();

    let mut output = create_output_writer(args.output);

    let certificates =
        CrtShFetcher::fetch_certificates(&args.website).expect("Failed to fetch certificates");

    output
        .write_output(&certificates)
        .expect("Failed to write output");
}
