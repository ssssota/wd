# wd

wd - a command line tool for print specified position of words

*Inpired by https://github.com/onsd/wd*

## Installation

```sh
cargo install wd
```

## Usage

### stdin

```sh
echo "a b c" | wd -n 1 2
# a b
echo "a b c" | wd -n 1 3
# a c
```

### file input

```sh
cat some.txt
# abc def
wd --input some.txt --number 1
# abc
wd -i some.txt -n 2
# def
```

### with exa

`exa` is modern `ls` made by Rust.

```sh
exa -l
# .rwxrwxrwx 5.9k ssssota 15 May 19:42 Cargo.lock
# .rwxrwxrwx  252 ssssota 15 May 19:42 Cargo.toml
# .rwxrwxrwx  335 ssssota 15 May 19:45 README.md
# drwxrwxrwx    - ssssota 15 May 18:19 src
# drwxrwxrwx    - ssssota 15 May 18:53 target
exa -l | wd -n 1 7
# .rwxrwxrwx Cargo.lock
# .rwxrwxrwx Cargo.toml
# .rwxrwxrwx README.md
# drwxrwxrwx src
# drwxrwxrwx target
```
