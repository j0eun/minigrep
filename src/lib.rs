use clap::Parser;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let matches = if config.match_case {
        search_case_sensitive(&config.search_term, &contents)
    } else {
        search_case_insensitive(&config.search_term, &contents)
    };

    for line in matches {
        println!("{line}");
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    search_term: String,
    file_path: String,

    /// searching case-sensitivity
    #[arg(short, long, default_value_t = false)]
    match_case: bool,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let config = Config::parse();

        Ok(config)
    }
}

pub fn search_case_sensitive<'a>(search_term: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(search_term) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(search_term: &str, contents: &'a str) -> Vec<&'a str> {
    let search_term = search_term.to_lowercase();
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&search_term) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_term = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(search_term, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let search_term = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(search_term, contents)
        );
    }
}
