use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{} - failed to read from file '{}': {:?}", 
                      "Error: ".bold().red(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}",
                      "Error:".bold().red(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, replaced_data) {
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                      "Error: ".red().bold(), args.output, e);
            std::process::exit(1);
        },
        Ok(_) => {},
    };
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!("{} - change occurences of one string with another.",
              "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <filename> <output>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} - wrong number of arguments: expected 4, got {}",
                  "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments { target: args[0].clone(), replacement: args[1].clone(), filename: args[2].clone(), output: args[3].clone() }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}