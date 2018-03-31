extern crate itertools;
extern crate rustyline;
#[macro_use]
extern crate structopt;
extern crate lalrpop_util;

mod ast;
#[allow(unknown_lints)]
#[allow(clippy)]
mod grammar;
mod interpreter;
mod lexer;
mod tokens;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;
use rustyline::Editor;
use lexer::Lexer;
use grammar::ExprParser;
use interpreter::Evaluate;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let options = Options::from_args();

    match options.file {
        Some(path) => run_file(&path),
        None => repl(),
    }
}

fn run(source: &str) {
    let lexer = Lexer::new(source);
    let parser = ExprParser::new();
    let value = parser
        .parse(lexer)
        .map_err(|e| format!("{:?}", e))
        .and_then(|ast| ast.eval());

    match value {
        Ok(value) => println!("{}\n", value),
        Err(e) => eprintln!("Error: {}\n", e),
    }
}

fn run_file(path: &PathBuf) {
    // TODO: Better error handling
    let mut file = File::open(path).expect("not found");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("couldn't read file");
    run(&buffer);
}

fn repl() {
    let mut editor = Editor::<()>::new();

    let _ = editor.load_history("history.txt");

    println!("| Ein 0.1.0");
    println!("| Copyright © 2018 Joe Clay");
    println!("| Released under the MIT License\n");

    loop {
        match editor.readline(">> ") {
            Ok(line) => {
                editor.add_history_entry(&line);
                run(&line);
            }
            Err(err) => {
                eprintln!("Error: {}\n", err);
                break;
            }
        }
    }

    editor.save_history("history.txt").unwrap();
}
