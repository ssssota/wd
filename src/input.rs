use std::{
    fs,
    io::{self, Read},
    path::Path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "wd",
    about = "a command line tool for print specified position of words"
)]
pub struct CliOptions {
    /// Position of words
    #[structopt(short, long)]
    pub number: Vec<u64>,

    /// Filepath of input
    #[structopt(short, long)]
    pub input: Option<String>,

    #[structopt(short, long)]
    pub delimiter: Option<String>,
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, String> {
    fs::read_to_string(path)
        .map(|s| s.trim_end().to_string())
        .map_err(|e| e.to_string())
}

pub fn read_stdin() -> Result<String, String> {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buf) {
        Ok(_) => Ok(buf.trim_end().to_string()),
        Err(_) => Err("Input words into stdin.".to_string()),
    }
}

pub fn split_words(line: &str, delimiter: Option<&str>) -> Vec<String> {
    match delimiter {
        Some(d) => line.split(d).map(|w| w.to_string()).collect(),
        None => line.split_whitespace().map(|w| w.to_string()).collect(),
    }
}

pub fn parse_input(lines: &str, delimiter: Option<&str>) -> Vec<Vec<String>> {
    lines
        .split("\n")
        .map(str::trim)
        .map(|v| split_words(v, delimiter))
        .map(|v| v.into_iter().map(|w| w.trim().to_string()))
        .map(Iterator::collect)
        .collect()
}
