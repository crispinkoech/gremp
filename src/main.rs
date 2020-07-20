use std::{
    process,
    env,
};

use mgrep::Config;

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mgrep::run(&config) {
        eprintln!("Encountered an error: {}", e);
        process::exit(1);
    }
}
