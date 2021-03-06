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

## Build

Debug:

```shell
cargo build
```

Release:

```shell
cargo build --release
```

## Run

```shell
cargo run
```

## Concepts

### Variables

- Immutable by default
- Can be made mutable
- Immutables are never reassignable
- They can however be redeclared which is called shadowing

### Ownership

- no garbage collector
- no manual memory allocation
- if a owner goes out of scope the value is discarded (happens by moving or at the end of blocks)

Ownership Rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### References

- To use a value without taking ownership we can use the `&` operator
- if a reference goes out of scope the value is NOT dropped that would only happen if the owner goes out of scope

Reference Rules

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

Dangling References

- if a value goes out of scope it is dropped
- this is why the following does not work because we return a reference
- the code that is calling that may have a reference to a value that was already dropped

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
```

### String

There are two types of Strings

- String
  - variable string type
- str
  - primitive sting literal and slices
  - has no ownership so the `&` reference must always be used
  - immutable

## Module System

- src/main.rs -> root of binary crate
- src/lib.rs -> root of library crate

A package can host 0 or 1 library crates and one or multiple bin crates.

inside src/bin we can define more binary packages that can be called via
`cargo run --bin {name_of_crate}`. We do not need to register them in
cargo.toml but I couldn't get intellisense to work without it.

Common `use` Conventions:
functions: the parent module of the function is brought into scope
structs/enums: the struct/enum is brought in with the full path except if two modules define the same name for a struct/enum
## Privacy

A struct inside a module can be made public, however the fields are
still private by default. If even one field is private we can't create
a new instance of it. To get around that we need a public constructor
method that returns a new instance of said struct. This method is 
usually called `::new`.

Enums work a bit different. All their variant are public if the enum
itself is public.

### Macros

- macros are written like functions with an `!` at the end

```rust
println!("Hello World");
```

### Keywords

<https://doc.rust-lang.org/book/appendix-01-keywords.html>

## Random Notes

- Rust lets you think more about how to use data and prevents you from doing sh*t. I think it's cool that you can look at a function signature and see if this function mutates something or just uses its values. Example for that is String::clear. It takes a mutable reference so the string itself must be mutable in order to be cleared. It doesn't work if the string was not declared mutable.
- The principle around ownership and borrowing also applies for structs and theirs data. If you give a reference to a struct then their fields are also references to that field in the struct. That means there is not inconsistency when using borrowed and owned data.

[1]: https://doc.rust-lang.org/book/
