_**Disclaimer:** This language is a toy project, and is not currently under active development - you may be looking for the [ein-lang/ein](https://github.com/ein-lang/ein) repo instead, which is a seperate project by a different developer! If this project starts up again, I will change the name to avoid confusion :)_

# 🐕 Ein

[![Build Status](https://img.shields.io/github/workflow/status/17cupsofcoffee/ein/CI%20Build/main)](https://github.com/17cupsofcoffee/ein/actions?query=branch%3Amain)

Ein is a simple, dynamically-typed programming language, inspired by Rust (in which the interpreter is written), Lua and Wren.

## Status
Extremely, extremely work-in-progress -
I'm developing this project following the exercises in Bob Nystrom's wonderful [Crafting Interpreters](http://craftinginterpreters.com/) book. Hopefully something interesting and/or vaguely usable will come out of it!

## Example
```rust
fn sayHello(name) {
    return "Hello, " + name + "!";
}

let greeting = sayHello("Ein");

print(greeting);
```

The above syntax is subject to change - I'll try to keep it in sync with the latest version of the code!

## Project Structure

This project is made up of several Rust crates:

| Crate | Description |
| --- | --- |
| `ein` | The top level crate, containing a command line interface and REPL. |
| `ein_syntax` | Contains a hand-written lexer and [LALRPOP](https://github.com/lalrpop/lalrpop)-generated parser for the language's syntax. |
| `ein_vm` | A stack-based virtual machine. Incomplete, but can run some basic expressions. |
