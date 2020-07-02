use std::{
    fs,
    env,
    error::Error,
};

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(
        args: &[String],
    ) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let pattern= args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Self { pattern, filename, case_sensitive })
    }
}

pub fn search<'a>(
    pattern: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            results.push((idx + 1, line));
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    pattern: &str,
    contents: &'a str,
)-> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    let pattern = pattern.to_lowercase();

    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&pattern) {
            results.push((idx + 1, line));
        }
    }

    results
}

pub fn run(
    config: &Config
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.pattern, &contents)
    } else {
        search_case_insensitive(&config.pattern, &contents)
    };

    for (line_no, line) in results {
        println!("{}. {}", line_no, line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_cli_arguments() {
        let mut args = vec![
            String::from("/path/to/binary"),
        ];

        let result = Config::new(&args);
        assert!(result.is_err(), "Missing all arguments");

        args.push(String::from("pattern_here"));

        let result = Config::new(&args);
        assert!(result.is_err(), "Missing file argument");
    }

    #[test]
    fn it_creates_a_config() {
        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern"),
            String::from("filename"),
        ];

        let result = Config::new(&args);
        assert!(result.is_ok(), "Should have accepted configuration");
    }

    #[test]
    fn it_checks_file_existence() {
        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern"),
            String::from("filename"),
        ];

        let config = Config::new(&args).unwrap_or_else(|err| {
            panic!(err);
        });

        let result = run(&config);
        assert!(result.is_err(), "Specified file does not exist");
    }

    #[test]
    fn it_searches_case_sensitive() {
        let pattern = "duct";
        let contents = "Rust:\nSafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(
            vec![
                (2, "Safe, fast, productive."),
            ],
            search(pattern, contents),
        );
    }

    #[test]
    fn it_searches_case_insensitive() {
        let pattern = "rUsT";
        let contents = "Rust:\nSafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec![
                (1, "Rust:"),
                (4, "Trust me.",)
            ],
            search_case_insensitive(pattern, contents),
        );
    }

    #[test]
    fn it_searches_in_file() {
        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern"),
            String::from("poem.txt"),
        ];

        let config = Config::new(&args).unwrap_or_else(|err| {
            panic!(err);
        });

        let result = run(&config);
        assert!(result.is_ok(), "Should have accepted input");
    }
}
