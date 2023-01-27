use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub querry: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Requires a querry and a file path");
        }

        let querry = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            querry,
            file_path,
            ignore_case,
        })
    }
}

pub fn search_case_sensitive<'a>(querry: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(querry) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(querry: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_uppercase().contains(&querry.to_uppercase()) {
            result.push(line);
        }
    }
    result
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let lines = match config.ignore_case {
        true => search_case_insensitive(&config.querry, &contents),
        false => search_case_sensitive(&config.querry, &contents),
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
        let querry = "fox";
        let contents = "\
the quick brown
fox jumps over
the lazy dog
        ";
        assert_eq!(
            vec!["fox jumps over"],
            search_case_sensitive(querry, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let querry = "animal";
        let contents = "\
1 flower
2 flowers
1 aNiMaL
3 flowers
4 fLoWeRs
";

        assert_eq!(vec!["1 aNiMaL"], search_case_insensitive(querry, contents))
    }
}
