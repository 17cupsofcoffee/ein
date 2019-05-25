use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::StructOpt;

use ein_syntax::parser::{self, ParseError};
use ein_vm::{Chunk, Instruction, RuntimeError, Value, VirtualMachine};

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

fn run<'a>(input: &'a str, vm: &mut VirtualMachine) -> Result<'a, Option<Value>> {
    let mut chunk = Chunk::new();

    match parser::parse_expr(input) {
        Ok(expr) => chunk.emit(&expr),
        Err(_) => {
            let ast = parser::parse_program(input)?;
            chunk.emit(&ast);
        }
    }

    chunk.add_instruction(Instruction::Return);

    let return_value = vm.run(&chunk)?;

    Ok(return_value)
}

fn run_file(path: &PathBuf) {
    match fs::read_to_string(path) {
        Ok(program) => {
            if let Err(e) = run(&program, &mut VirtualMachine::new()) {
                eprintln!("Error: {}\n", e);
            }
        }
        Err(e) => eprintln!("Error: {}\n", e),
    }
}

fn repl() {
    println!("| Ein {}", env!("CARGO_PKG_VERSION"));
    println!("| Copyright © 2018-2019 Joe Clay");
    println!("| Released under the MIT License\n");

    let mut editor = Editor::<()>::new();

    if let Err(e) = editor.load_history("history.txt") {
        if !is_not_found_error(&e) {
            eprintln!("Failed to load REPL history: {}\n", e);
        }
    }

    let mut ctx = VirtualMachine::new();

    loop {
        let line = match editor.readline(">> ") {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error: {}\n", e);
                break;
            }
        };

        editor.add_history_entry(line.as_str());

        match run(&line, &mut ctx) {
            Ok(Some(value)) => println!("{}\n", value),
            Ok(None) => {}
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }

    if let Err(e) = editor.save_history("history.txt") {
        eprintln!("Failed to save REPL history: {}\n", e);
    }
}

fn is_not_found_error(error: &ReadlineError) -> bool {
    match error {
        ReadlineError::Io(inner_error) => match inner_error.kind() {
            io::ErrorKind::NotFound => true,
            _ => false,
        },
        _ => false,
    }
}

type Result<'a, T = ()> = std::result::Result<T, EinError<'a>>;

enum EinError<'a> {
    Parse(ParseError<'a>),
    Runtime(RuntimeError),
    Io(io::Error),
    Readline(ReadlineError),
}

impl<'a> From<ParseError<'a>> for EinError<'a> {
    fn from(err: ParseError<'a>) -> EinError<'a> {
        EinError::Parse(err)
    }
}

impl<'a> From<RuntimeError> for EinError<'a> {
    fn from(err: RuntimeError) -> EinError<'a> {
        EinError::Runtime(err)
    }
}

impl<'a> From<io::Error> for EinError<'a> {
    fn from(err: io::Error) -> EinError<'a> {
        EinError::Io(err)
    }
}

impl<'a> From<ReadlineError> for EinError<'a> {
    fn from(err: ReadlineError) -> EinError<'a> {
        EinError::Readline(err)
    }
}

impl<'a> Display for EinError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            EinError::Parse(e) => e.fmt(f),
            EinError::Runtime(e) => e.fmt(f),
            EinError::Io(e) => e.fmt(f),
            EinError::Readline(e) => e.fmt(f),
        }
    }
}
