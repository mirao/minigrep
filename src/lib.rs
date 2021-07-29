use std::error::Error;
use std::{env, fs};

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_valid_input() {
        let args = &[
            "minigrep".to_string(),
            "nobody".to_string(),
            "poem.txt".to_string(),
        ];
        assert_eq!(
            Config::new(args),
            Ok(Config {
                query: "nobody".to_string(),
                filename: "poem.txt".to_string(),
                case_sensitive: true
            })
        );
    }

    #[test]
    fn config_new_not_enough_arguments() {
        let vec: Vec<_> = vec!["minigrep".to_string(), "nobody".to_string()];
        assert_eq!(Config::new(&vec), Err("not enough arguments"));
    }

    #[test]
    fn run_file_not_found() {
        let config = Config {
            filename: "file_not_found.txt".to_string(),
            query: "nobody".to_string(),
            case_sensitive: true,
        };

        assert_eq!(
            run(config).unwrap_err().to_string(),
            "No such file or directory (os error 2)"
        );
    }

    #[test]
    fn run_invalid_content() {
        let config = Config {
            filename: "src/test_data/invalid.txt".to_string(),
            query: "nobody".to_string(),
            case_sensitive: true,
        };

        assert_eq!(
            run(config).unwrap_err().to_string(),
            "stream did not contain valid UTF-8"
        );
    }

    #[test]
    fn run_valid_content() {
        let config = Config {
            filename: "src/test_data/valid.txt".to_string(),
            query: "nobody".to_string(),
            case_sensitive: true,
        };
        assert_eq!(run(config).unwrap(), ());
    }

    #[test]
    fn search_case_sensitive_no_result() {
        let query = "ductivity";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.";

        assert_eq!(search(query, contents), vec![] as Vec<&str>);
        // Other syntax alternatives
        assert_eq!(search(query, contents), <Vec<&str>>::new());
        assert_eq!(search(query, contents), Vec::new() as Vec<&str>);
    }

    #[test]
    fn search_case_sensitive_one_result() {
        let query = "duct";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\n\
        Duct tape.";

        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn search_case_sensitive_more_results() {
        let query = "st";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.";

        assert_eq!(
            search(query, contents),
            vec!["Rust:", "safe, fast, productive."]
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\n\
        Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            super::search_case_insensitive(query, contents)
        );
    }
}
