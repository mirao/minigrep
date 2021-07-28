use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use crate::{run, Config};

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
                filename: "poem.txt".to_string()
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
        };
        assert_eq!(run(config).unwrap(), ());
    }
}
