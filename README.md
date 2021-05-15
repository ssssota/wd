# wd

wd - a command line tool for print specified position of words

*Inpired by https://github.com/onsd/wd*

## Installation

```sh
cargo install wd
```

## Usage

```sh
echo "a b c" | wd -n 1 2
# a b
echo "a b c" | wd -n 1 3
# a c
```

```sh
cat some.txt
# abc def
wd --input some.txt --number 1
# abc
wd -i some.txt -n 2
# def
```
