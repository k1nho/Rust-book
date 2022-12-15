use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query : String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case =  env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path , ignore_case})
    } 
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };
    
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    
        for line in contents.lines() {
            if line.contains(query) {
                matches.push(line);
            }
        }
    matches
}

pub fn search_case_insensitive<'a>(query: &str , contents : &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    let query = query.to_lowercase();
    
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                matches.push(line);
            }
        }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_match() {
        let query = "erful";
        let contents = "\
Rustlang for the win!
Powerful, Ownership
Insane speed, and powerful concurrency";

        assert_eq!(vec!["Powerful, Ownership", "Insane speed, and powerful concurrency"], search_case_sensitive(&query, &contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "next";
        let contents = "\
More next to be made
efficient to the moon
Next 13 is out
Full rust based bundler
a new next era";

        assert_eq!(vec!["More next to be made", "Next 13 is out", "a new next era"], search_case_insensitive(&query, &contents));
    }

    #[test]
    fn test_case_sensitive() {
        let query = "next";
        let contents = "\
More next to be made
efficient to the moon
Next 13 is out
Full rust based bundler
a new next era";

        assert_eq!(vec!["More next to be made", "a new next era"], search_case_sensitive(&query, &contents));
    }
}
