use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // Our error values will always be string literals that have the 'static lifetime.
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // We’re using the is_ok method on the Result to check whether the environment variable is set, which means the program should do a case-insensitive search.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

// with explicit lifetime 'a we are saying that the returned vector should contain string slices
// that reference slices of the argument contents (rather than the argument query).
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
        let query = "love";
        let contents = "\
just a test
with some text
I love you
coisa linda";

        assert_eq!(vec!["I love you"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "LoVe";
        let contents = "\
just a test
with some text
I love you
coisa linda";

        assert_eq!(vec!["I love you"], search_case_insensitive(query, contents));
    }
}
