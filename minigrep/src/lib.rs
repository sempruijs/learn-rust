use std::error::Error;
use std::fs;

pub struct Config {
    pub querry: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Requires a querry and a file path");
        }

        let querry = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { querry, file_path })
    }
}

pub fn search<'a>(querry: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(querry) {
            result.push(line);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    for line in search(&config.querry, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let querry = "fox";
        let contents = "\
the quick brown
fox jumps over
the lazy dog
        ";
        assert_eq!(vec!["fox jumps over"], search(querry, contents));
    }
}
