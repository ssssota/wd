mod input;
mod wd;

use input::{CliOptions, parse_input};
use structopt::StructOpt;

fn main() {
    let opt = CliOptions::from_args();
    let words_res = parse_input();
    println!("{}", match words_res {
        Ok(words) => wd::wd(words, opt.number).join(" "),
        Err(err) => err,
    });
}
