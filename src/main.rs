#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use clap::Parser;
use sortcount::cli::Args;
use sortcount::run;
use std::error::Error;
use std::process::exit;

fn main() {
    let args = Args::parse();

    if let Err(error) = run(&args) {
        let mut message = format!("sortcount: {error}");

        let mut source_error_opt = error.source();
        while let Some(source_error) = source_error_opt {
            message = format!("{message}: {source_error}");
            source_error_opt = source_error.source();
        }
        eprintln!("{message}");

        exit(1);
    }
}
