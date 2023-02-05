#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use crate::cli::Args;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::{fmt, io};

pub mod cli;

fn open_reader(args: &Args) -> Result<Box<dyn BufRead>> {
    match args.file.as_str() {
        "-" => Ok(Box::new(stdin().lock())),
        path => {
            let file = File::open(path).map_err(Error::IoInput)?;
            Ok(Box::new(BufReader::new(file)))
        }
    }
}

/// Run with provided parameters
///
/// # Errors
///
/// Everything defined in `Error` can happen here
pub fn run(args: &Args) -> Result<()> {
    let reader = open_reader(args)?;

    let mut counts: HashMap<String, u64> = HashMap::new();

    let lines = reader.lines();
    for line_res in lines {
        let line = line_res.map_err(Error::IoInput)?;

        *counts.entry(line).or_default() += 1;
    }

    for (word, count) in counts {
        println!("{count:7} {word}");
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IoInput(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoInput(_) => write!(f, "cannot read"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoInput(ref err) => Some(err),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;
}
