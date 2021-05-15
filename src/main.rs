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
    match input.map(parse_input) {
        Ok(lines) => {
            for words in lines {
                println!("{}", wd::wd(words, opt.number.clone()).join(" "));
            }
        },
        Err(err) => println!("{}", err),
    };
}
