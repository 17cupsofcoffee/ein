extern crate rustyline;
#[macro_use]
extern crate structopt;
extern crate fnv;
extern crate lalrpop_util;

mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Value;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;
use structopt::StructOpt;
use rustyline::Editor;
use lexer::Lexer;
use parser::ProgramParser;
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

fn run(source: &str, ctx: &mut Context) -> Result<Value, String> {
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();

    parser
        .parse(lexer)
        .map_err(|e| format!("{:?}", e))?
        .eval(ctx)
}

fn run_file(path: &PathBuf) {
    // TODO: Better error handling
    let mut file = File::open(path).expect("not found");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("couldn't read file");

    let mut context = Context::new();

    if let Err(e) = run(&buffer, &mut context) {
        eprintln!("Error: {}", e);
    }
}

fn repl() {
    println!("| Ein 0.1.0");
    println!("| Copyright © 2018 Joe Clay");
    println!("| Released under the MIT License\n");

    let mut editor = Editor::<()>::new();
    let _ = editor.load_history("history.txt");

    let mut context = Context::new();

    loop {
        match editor.readline(">> ") {
            Ok(line) => {
                editor.add_history_entry(&line);
                match run(&line, &mut context) {
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
