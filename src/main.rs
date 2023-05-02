/* Search
 * A simple grep like tool for searching text in files
 * Github: https://www.github.com/awesomelewis2007/search
 * Licence: GPL-3.0
 * By: Lewis Evans
*/

use colored::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: search <filename> <pattern> [args]");
        return;
    }

    let filename = &args[1];
    let pattern = &args[2];

    let mut case_sensitive = false;
    let mut line_number = false;
    let mut show_summary = false;
    let mut silent = false;

    for arg in args.iter().skip(3) {
        if arg == "-nc" || arg == "--no-color" {
            colored::control::set_override(false);
        } else if arg == "-c" || arg == "--case-sensitive" {
            case_sensitive = true;
        } else if arg == "-l" || arg == "--line-number" {
            line_number = true;
        } else if arg == "-s" || arg == "--summary" {
            show_summary = true;
        } else if arg == "-S" || arg == "--silent" {
            silent = true;
        } else if arg == "-h" || arg == "--help" {
            println!("Usage: search <filename> <pattern> [args]");
            println!("A simple grep like tool for searching text in files");
            println!("\nArguments:");
            println!("\t-c, --case-sensitive\t\tCase sensitive search");
            println!("\t-l, --line-number\t\tShow line number");
            println!("\t-s, --summary\t\t\tShow summary");
            println!("\t-h, --help\t\t\tShow this help message");
            println!("\t-nc, --no-color\t\t\tDisable colored output");
            return;
        }
    }

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("{} Could not open file {}", "Error:".red(), filename);
            return;
        }
    };
    let reader = BufReader::new(file);
    let mut total_matches = 0;
    if !silent {
        println!(
            "{} {} {} {} {}",
            "Searching for".cyan(),
            pattern.green(),
            "in".cyan(),
            filename.green(),
            "...".cyan()
        );
    }
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if case_sensitive {
            if line.contains(pattern) {
                if line_number {
                    println!(
                        "{}\t| {}",
                        (index + 1).to_string().cyan(),
                        line.replace(pattern, &pattern.green().to_string())
                    );
                    total_matches += 1;
                } else {
                    println!("{}", line.replace(pattern, &pattern.green().to_string()));
                    total_matches += 1;
                }
            }
        } else {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                if line_number {
                    println!(
                        "{}\t| {}",
                        (index + 1).to_string().cyan(),
                        line.replace(pattern, &pattern.green().to_string())
                    );
                    total_matches += 1;
                } else {
                    println!("{}", line.replace(pattern, &pattern.green().to_string()));
                    total_matches += 1;
                }
            }
        }
    }
    if total_matches == 0 {
        if !silent {
            println!("{}", "No matches found".red());
        }
    }
    if show_summary {
        if total_matches != 0 {
            println!(
                "{} {} matches found",
                "Summary:".cyan(),
                total_matches.to_string().green()
            );
        }
    }
}
