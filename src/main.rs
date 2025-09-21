use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

struct Config {
    pattern: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skipping "target/debug/grep-rs"
        args.next();

        let pattern = match args.next() {
            Some(p) => p,
            None => return Err("Pattern not found"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("File path not found"),
        };

        let mut ignore_case = false;
        for arg in args {
            if arg == "-i" || arg == "--ignore-case" {
                ignore_case = true;
            } else {
                return Err("Unknown argument");
            }
        }

        Ok(Config {
            pattern,
            filename,
            ignore_case,
        })
    }
}

fn print_usage() {
    eprintln!("Usage: mygrep <pattern> <file_path> [-i | --ignore-case]");
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&config.filename)?;
    let reader = BufReader::new(file);

    for (i, line_res) in reader.lines().enumerate() {
        let line = line_res?;
        if config.ignore_case {
            if line.to_lowercase().contains(&config.pattern.to_lowercase()) {
                println!("{}: {}", i+1, line);
            }
        } else {
            if line.contains(&config.pattern) {
                println!("{}: {}", i+1, line)
            }
        }
    }

    Ok(())
}

fn main() {
     let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument error: {}", err);
        eprintln!("Usage: mygrep <pattern> <file_path> [-i|--ignore-case]");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
