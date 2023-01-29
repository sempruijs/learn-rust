use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Requires a query and a file path");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_uppercase().contains(&query.to_uppercase()) {
            result.push(line);
        }
    }
    result
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let lines = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search_case_sensitive(&config.query, &contents),
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "fox";
        let contents = "\
the quick brown
fox jumps over
the lazy dog
        ";
        assert_eq!(
            vec!["fox jumps over"],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "animal";
        let contents = "\
1 flower
2 flowers
1 aNiMaL
3 flowers
4 fLoWeRs
";

        assert_eq!(vec!["1 aNiMaL"], search_case_insensitive(query, contents))
    }
}
