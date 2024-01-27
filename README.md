# minigrep

This project is based on [Rust official tutorial: An I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
There are tiny differences that I modified for improve usage(e.g., using command-line option instead of an environment variable).


## Dependencies

- [clap](https://docs.rs/clap/latest/clap/index.html)

## Usage

```
Usage: minigrep [OPTIONS] <SEARCH_TERM> <FILE_PATH>

Arguments:
  <SEARCH_TERM>
  <FILE_PATH>

Options:
  -m, --match-case  searching case-sensitivity
  -h, --help        Print help
  -V, --version     Print version
```
