extern crate rustyline;
#[macro_use]
extern crate structopt;
extern crate fnv;
extern crate lalrpop_util;
#[macro_use]
extern crate failure;

mod ast;
mod interpreter;
mod lexer;
mod parser;

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;
use rustyline::Editor;
use lexer::Lexer;
use parser::{ExprParser, ProgramParser};
use interpreter::{Context, Evaluate};

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

fn run_file(path: &PathBuf) {
    // TODO: Better error handling
    let mut file = File::open(path).expect("not found");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("couldn't read file");

    let lexer = Lexer::new(&buffer);
    let parser = ProgramParser::new();

    let ret = parser
        .parse(lexer)
        .map_err(|e| format!("{}", e))
        .map(|ast| ast.eval(&mut Context::new()));

    if let Err(e) = ret {
        eprintln!("Error: {}", e);
    }
}

fn repl() {
    println!("| Ein 0.1.0");
    println!("| Copyright © 2018 Joe Clay");
    println!("| Released under the MIT License\n");

    let mut editor = Editor::<()>::new();
    let _ = editor.load_history("history.txt");

    let mut ctx = Context::new();
    let program_parser = ProgramParser::new();
    let expr_parser = ExprParser::new();

    loop {
        match editor.readline(">> ") {
            Ok(line) => {
                editor.add_history_entry(&line);

                let ret = match expr_parser.parse(Lexer::new(&line)) {
                    Ok(expr) => expr.eval(&mut ctx),
                    Err(_) => match program_parser.parse(Lexer::new(&line)) {
                        Ok(ast) => ast.eval(&mut ctx),
                        Err(e) => Err(format!("{}", e)),
                    },
                };

                match ret {
                    Ok(value) => println!("{}\n", value),
                    Err(e) => eprintln!("Error: {}\n", e),
                }
            }
            Err(err) => {
                eprintln!("Error: {}\n", err);
                break;
            }
        }
    }

    editor.save_history("history.txt").unwrap();
}
