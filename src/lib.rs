use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_vector_when_contents_is_empty() {
        let query = "duct";
        let contents = "";
        let empty_vector: Vec<&str> = Vec::from([]);

        assert_eq!(empty_vector, search(query, contents));
    }

    #[test]
    fn returns_one_line_when_contents_contain_query() {
        let query = "duct";
        let contents = "cacafuti ductionary";

        assert_eq!(Vec::from(["cacafuti ductionary"]), search(query, contents));
    }

    #[test]
    fn returns_two_lines_when_contents_contain_query() {
        let query = "duct";
        let contents = "cacafuti ductionary
        Duck duct Go,
        call me
        ";

        assert_eq!(
            Vec::from(["cacafuti ductionary", "Duck duct Go,"]),
            search(query, contents)
        );
    }
}
