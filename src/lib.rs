/*!
 * # Gremp
 * 
 * `gremp` is a utility for searching for a string pattern in a file.
 */

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
        mut args: impl Iterator<Item=String>,
    ) -> Result<Self, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(p) => p,
            None => return Err("Didn't get pattern to match"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self { pattern, filename, case_sensitive })
    }
}

/// Searches for a token in a string. It is case sensitive.
///
/// # Examples
/// 
/// ```
/// use gremp::*;
/// 
/// let pattern = "ust";
/// let contents = "Rust. Effective.\nWithout DUST.";
/// 
/// let result = search(pattern, contents);
/// 
/// assert_eq!(result, vec![(1, "Rust. Effective.")]);
/// ```

pub fn search<'a>(
    pattern: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    contents.lines()
        .enumerate()
        .map(|(idx, line)| (idx + 1, line))
        .filter(|(_, line)| line.contains(pattern))
        .collect()
}

/// Searches for a token in a string. It is NOT case sensitive.
///
/// # Examples
/// 
/// ```
/// use gremp::*;
/// 
/// let pattern = "ust";
/// let contents = "Rust. Effective.\nWithout DUST.";
/// 
/// let result = search_case_insensitive(pattern, contents);
/// 
/// assert_eq!(result, vec![(1, "Rust. Effective."), (2, "Without DUST.")]);
/// ```

pub fn search_case_insensitive<'a>(
    pattern: &str,
    contents: &'a str,
) -> Vec<(usize, &'a str)> {
    contents.lines()
        .enumerate()
        .map(|(idx, line)| (idx + 1, line))
        .filter(|(_, line)| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

/// Searches for a pattern in a file.
///
/// # Examples
/// 
/// ```
/// use gremp::*;
/// 
/// let args = vec![
///     String::from("/path/to/binary"),
///     String::from("pattern"),
///     String::from("sample.txt"),
/// ];
///
/// let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
///     panic!(err);
/// });
///
/// let result = run(&config);
/// assert!(result.is_ok(), "Should have accepted input");
/// ```

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
        let args = vec![
            String::from("/path/to/binary"),
        ];

        let result = Config::new(args.into_iter());
        assert!(result.is_err(), "Missing all arguments");

        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern_here"),
        ];

        let result = Config::new(args.into_iter());
        assert!(result.is_err(), "Missing file argument");
    }

    #[test]
    fn it_creates_a_config() {
        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern"),
            String::from("filename"),
        ];

        let result = Config::new(args.into_iter());
        assert!(result.is_ok(), "Should have accepted configuration");
    }

    #[test]
    fn it_checks_file_existence() {
        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern"),
            String::from("filename"),
        ];

        let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
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
            String::from("sample.txt"),
        ];

        let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
            panic!(err);
        });

        let result = run(&config);
        assert!(result.is_ok(), "Should have accepted input");

        // Search case insensitive
        env::set_var("CASE_INSENSITIVE", "1");

        let args = vec![
            String::from("/path/to/binary"),
            String::from("pattern"),
            String::from("sample.txt"),
        ];

        let config = Config::new(args.into_iter()).unwrap_or_else(|err| {
            panic!(err);
        });

        let result = run(&config);
        assert!(result.is_ok(), "Should have accepted input");
    }
}
