use clap::{Parser, ValueEnum};

/// sort | count, count occurrences of lines, but faster and with less memory
///
/// Uses a hash map internally and only stores each field once.
/// Output order is undefined if not otherwise specified.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    // /// only print duplicate lines
    // #[clap(short = 'd', long = "repeated")]
    // pub repeated: bool,

    // // sort uses -f, uniq -i
    // /// ignore differences in case, will make everything lower case
    // #[clap(short = 'i', long = "ignore-case")]
    // pub ignore_case: bool,

    // /// only print unique lines
    // #[clap(short = 'u', long = "unique")]
    // pub unique: bool,

    // /// line delimiter is NUL, not newline
    // #[clap(short = 'z', long = "zero-terminated")]
    // pub zero_terminated: bool,

    // /// skip leading blanks
    // #[clap(short = 'b', long = "ignore-leading-blanks")]
    // pub ignore_leading_blanks: bool,
    /// order of output sorting
    ///
    /// if not specified, the order is undefined
    #[clap(short = 'o', long = "order")]
    pub order: Option<Order>,

    /// input file
    #[clap(default_value = "-")]
    pub file: String,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Order {
    /// show the entry with the least amount of counts first
    Count,
    /// show the entry with the most amount of counts first
    CountDesc,
}
