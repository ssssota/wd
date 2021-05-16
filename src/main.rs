mod input;
mod wd;

use input::{parse_input, read_from_file, read_stdin, CliOptions};
use structopt::StructOpt;

fn main() {
    let opt = CliOptions::from_args();
    let delimiter = opt.delimiter;
    let number = opt.number;
    let input = match opt.input {
        Some(path) => read_from_file(path),
        None => read_stdin(),
    };

    let parsed = input.map(|s| parse_input(&s, delimiter.as_deref()));
    match parsed {
        Ok(lines) => {
            for words in lines {
                println!("{}", wd::wd(words, number.clone()).join(" "));
            }
        }
        Err(err) => println!("{}", err),
    };
}
