# 🐕 Ein

[![Build Status](https://travis-ci.org/17cupsofcoffee/ein.svg?branch=master)](https://travis-ci.org/17cupsofcoffee/ein)

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
| `ein_treewalk` | A simple tree-walking interpreter. Probably not very performant, but easy to hack on! |
