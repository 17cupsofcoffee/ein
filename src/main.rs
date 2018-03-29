extern crate itertools;
extern crate rustyline;
#[macro_use]
extern crate structopt;
extern crate lalrpop_util;

mod ast;
mod grammar;
mod lexer;
mod tokens;

use ast::Expr;
use lexer::Lexer;
use tokens::Token;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;
use rustyline::Editor;

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

fn run(source: &str) -> Result<Expr, lalrpop_util::ParseError<usize, Token, String>> {
    grammar::ExprParser::new().parse(Lexer::new(source))
}

fn run_file(path: &PathBuf) {
    // TODO: Better error handling
    let mut file = File::open(path).expect("not found");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("couldn't read file");
    println!("{:#?}", run(&buffer));
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
                println!("{:#?}", run(&line))
            }
            Err(err) => {
                eprintln!("Error: {}\n", err);
                break;
            }
        }
    }

    editor.save_history("history.txt").unwrap();
}
