use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use rustyline::Editor;
use structopt::StructOpt;

use ein_syntax::parser::{self, ParseError};
use ein_treewalk::{Context, Evaluate};

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

fn run_expr<'a>(input: &'a str, ctx: &mut Context) -> Result<(), ParseError<'a>> {
    match parser::parse_expr(input)?.eval(ctx) {
        Ok(value) => println!("{}\n", value),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    Ok(())
}

fn run_program(input: &str, ctx: &mut Context) {
    match parser::parse_program(input) {
        Ok(ast) => {
            if let Err(e) = ast.eval(ctx) {
                eprintln!("Error: {}\n", e);
            }
        }
        Err(e) => eprintln!("Error: {}\n", e),
    }
}

fn run_file(path: &PathBuf) {
    // TODO: Better error handling
    let mut file = File::open(path).expect("not found");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("couldn't read file");

    run_program(&buffer, &mut Context::new())
}

fn repl() {
    println!("| Ein 0.1.0");
    println!("| Copyright © 2018 Joe Clay");
    println!("| Released under the MIT License\n");

    let mut editor = Editor::<()>::new();
    let _ = editor.load_history("history.txt");

    let mut ctx = Context::new();

    loop {
        match editor.readline(">> ") {
            Ok(line) => {
                editor.add_history_entry(&line);

                if run_expr(&line, &mut ctx).is_err() {
                    run_program(&line, &mut ctx);
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
