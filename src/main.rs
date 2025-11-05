use chess_fen_parser::*;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }
    match args[1].as_str() {
        "help" => {
            print_help();
            return;
        },
        "parse-string" => {
            if args.len() < 3 {
                eprintln!("There is no FEN string for parse-string");
                print_help();
                return;
            } else {
                let fen = &args[2];
                cmd_parse_string(fen);
                return;
            }
        },
        "parse-file" => {
            if args.len() < 3 {
                eprintln!("There is no file path for parse-file");
                print_help();
                return;
            } else {
                let path = &args[2];
                cmd_parse_file(path);
                return;
            }
        },
        "credits" => {
            print_credits();
            return;
        },
        other => {
            eprintln!("Unknown command: {}\n", other);
            print_help();
            return;
        }
    }
}

fn print_help() {
    println!("Commands:");
    println!("    parse-file <path>   Parse FEN from a text file");
    println!("    parse-string <fen>  Parse FEN from the provided string (in quotes)");
    println!("    help                Show this help message");
    println!("    credits             Show credits information");
}

fn cmd_parse_string(fen_str: &str) {
    match parse_fen(fen_str) {
        Ok(fen_data) => {
            print_parsing_results(&fen_data);
        },
        Err(e) => {
            eprintln!("Error parsing FEN: {}", e);
        }
    }
}

fn cmd_parse_file(path: &str) {
    let pb = std::path::PathBuf::from(path);
    match fs::read_to_string(pb.clone()) {
        Ok(contents) => {
            if let Some(line) = contents.lines().find(|l| !l.trim().is_empty()) {
                cmd_parse_string(line);
                return;
            } else {
                eprintln!("File '{}' contains no valid FEN string", pb.display());
                return;
            }
        },
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", pb.display(), e);
            return;
        }
    }
}

fn print_credits() {
    println!("Author: Dmytro Shvets");
}