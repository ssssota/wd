use std::io::{self, Read};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "wd", about="a command line tool for print specified position of words")]
pub struct CliOptions {

  /// Position of words
  #[structopt(short, long)]
  pub number: Vec<u64>,

  /// Filepath of input
  #[structopt(short, long)]
  pub input: Option<String>
}

pub fn read_stdin() -> Result<String, String> {
  let mut buf = String::new();
  let mut stdin = io::stdin();
  match stdin.read_to_string(&mut buf) {
    Ok(_) => Ok(buf),
    Err(_) => Err("Input words into stdin.".to_string()),
  }
}

pub fn parse_input(input: String) -> Vec<String> {
  input.split_whitespace().map(|str| str.to_string()).collect()
}
