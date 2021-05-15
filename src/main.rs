mod input;
mod wd;

use std::fs;
use structopt::StructOpt;
use input::{CliOptions, parse_input, read_stdin};

fn main() {
    let opt = CliOptions::from_args();
    let input = match opt.input {
        Some(path) => fs::read_to_string(path).map_err(|e| e.to_string()),
        None => read_stdin(),
    };
    println!("{}", match input.map(parse_input) {
        Ok(words) => wd::wd(words, opt.number).join(" "),
        Err(err) => err,
    });
}
