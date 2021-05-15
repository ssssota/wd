use std::io::{self, Read};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "wd")]
pub struct CliOptions {

  #[structopt(short, long)]
  pub number: Vec<u64>,
}

fn read_stdin() -> Result<String, String> {
  let mut buf = String::new();
  let mut stdin = io::stdin();
  match stdin.read_to_string(&mut buf) {
    Ok(_) => Ok(buf),
    Err(_) => Err("Input words into stdin.".to_string()),
  }
}

pub fn parse_input() -> Result<Vec<String>, String> {
  let input = read_stdin()?;
  Ok(input.split_whitespace().map(|str| str.to_string()).collect())
}
