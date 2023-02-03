use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // the first argument is the program name, we don't need that so we skip it
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_uppercase().contains(&query.to_uppercase()))
        .collect()
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
