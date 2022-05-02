# Rust Learning

This document is a collection of thoughts while I attempt to learn rust with the [Rust book][1]

## What is rust?

- low level programming language
- expressive enough for higher level programming

## Who is it for?

- basically all people who want to learn to write low level code
- people who want to write expressive low level code 
- people who want to learn a modern low level language

## How is the book used?

- people who have a basic understanding of programming
- read front to back
- there are concept chatpers which explain features of rust and project chapters that which build sample projects with the knowledge gained so far

## Creating a new project with cargo

```shell
cargo new hello_world
```

### Build

Debug:
```shell
cargo build
```

Release:
```shell
cargo build --release
```

### Run

```shell
cargo run
```

## Concepts

### String

There are two types of Strings

- String
  - variable string type
- str
  - primitive sting literal and slices
  - has no ownership so the `&` reference must always be used
  - immutable

### Macros

- macros are written like functions with an `!` at the end

```rust
println!("Hello World");
``` 

[1]: https://doc.rust-lang.org/book/
