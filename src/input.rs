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

fn split_words(line: &str, delimiter: Option<&str>) -> Vec<String> {
    match delimiter {
        Some(d) => line.split(d).map(|w| w.to_string()).collect(),
        None => line.split_whitespace().map(|w| w.to_string()).collect(),
    }
}

pub fn parse_input(input: &str, delimiter: Option<&str>) -> Vec<Vec<String>> {
    input
        .split("\n")
        .map(str::trim)
        .map(|v| split_words(v, delimiter))
        .map(|v| v.into_iter().map(|w| w.trim().to_string()))
        .map(Iterator::collect)
        .collect()
}

#[test]
fn test_split_words() {
    struct TestCase {
        line: &'static str,
        delimiter: Option<&'static str>,
        expected: Vec<String>,
    }
    let testcases = vec![
        TestCase {
            line: "1 2 3 a b c",
            delimiter: None,
            expected: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
        },
        TestCase {
            line: "abc def ghi jkl",
            delimiter: None,
            expected: vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jkl".to_string(),
            ],
        },
        TestCase {
            line: "||hi|",
            delimiter: Some("|"),
            expected: vec![
                "".to_string(),
                "".to_string(),
                "hi".to_string(),
                "".to_string(),
            ],
        },
    ];

    for testcase in testcases {
        assert_eq!(
            split_words(testcase.line, testcase.delimiter),
            testcase.expected
        );
    }
}

#[test]
fn test_parse_input() {
    struct TestCase {
        input: &'static str,
        delimiter: Option<&'static str>,
        expected: Vec<Vec<String>>,
    }

    let testcases = vec![
        TestCase {
            input: "a b c",
            delimiter: None,
            expected: vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]],
        },
        TestCase {
            input: "abc def\nghi jkl",
            delimiter: None,
            expected: vec![
                vec!["abc".to_string(), "def".to_string()],
                vec!["ghi".to_string(), "jkl".to_string()],
            ],
        },
        TestCase {
            input: "Name,Age,Height,Weight\nTaro,5,180,30\nJiro,3,150,50\nSaburo,1,90,3",
            delimiter: Some(","),
            expected: vec![
                vec![
                    "Name".to_string(),
                    "Age".to_string(),
                    "Height".to_string(),
                    "Weight".to_string(),
                ],
                vec![
                    "Taro".to_string(),
                    "5".to_string(),
                    "180".to_string(),
                    "30".to_string(),
                ],
                vec![
                    "Jiro".to_string(),
                    "3".to_string(),
                    "150".to_string(),
                    "50".to_string(),
                ],
                vec![
                    "Saburo".to_string(),
                    "1".to_string(),
                    "90".to_string(),
                    "3".to_string(),
                ],
            ],
        },
    ];

    for testcase in testcases {
        assert_eq!(
            parse_input(testcase.input, testcase.delimiter),
            testcase.expected
        );
    }
}
