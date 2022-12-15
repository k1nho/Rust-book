use std::error::Error;
use std::fs;

pub struct Config {
    pub query : String,
    pub file_path: String,
}

impl Config {
    pub fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    } 
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{line}")
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    
        for line in contents.lines() {
            if line.contains(query) || line.contains(&query.to_lowercase()) {
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

        assert_eq!(vec!["Powerful, Ownership", "Insane speed, and powerful concurrency"], search(&query, &contents));
    }
}
