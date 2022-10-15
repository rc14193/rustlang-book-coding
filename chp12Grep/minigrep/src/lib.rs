use std::fs;
use std::env;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = read_file(config.filename)?; 

    //println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        // challenge portion cli precedence, fall back env
        let case_sensitive: bool = if args.contains(&String::from("sensitive")) {
            true
        }else if args.contains(&String::from("insensitive")){
            false
        }else {
            !env::var("CASE_SENSITIVE").is_err()
        };
        println!("case {}", case_sensitive);

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn read_file(filename: String) -> Result<String, Box<dyn Error>>{
    Ok(fs::read_to_string(filename)?)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}