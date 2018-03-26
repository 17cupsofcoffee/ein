# 🐕 Ein

Ein is a simple, dynamically-typed programming language, inspired by Rust (in which the interpreter is written), Ruby and Wren.

## Status
Extremely, extremely work-in-progress -
I'm developing this project following the exercises in Bob Nystrom's wonderful [Crafting Interpreters](http://craftinginterpreters.com/) book. Hopefully something interesting and/or vaguely usable will come out of it!

## Example
```rust
fn sayHello(name) {
    "Hello, " + name + "!"
}

let greeting = sayHello("Ein")

print greeting
```

The above syntax is subject to change - I'll try to keep it in sync with the latest version of the code!