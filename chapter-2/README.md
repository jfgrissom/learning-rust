# Things to remember from the "Programming is a Guessing Game" chapter.

## Managing Rust Compiler Versions

- `rustup` is the command line tool for installing Rust and managing Rust
  versions. Figured this out just using the tool during this chapter's
  exercises.

## Handling Strings

- When using `println!` macro the string `"You guessed: {guess}"` has
  placeholders ({} - the book actually says "like crab claws holding the value
  in place"😊) that work like a JavaScript/Typescript template literal (example:
  `` `You guessed: ${guess}` ``).

## Handling Variables

- Variables are immutable by default. To make a variable mutable, use the `mut`
  keyword.

## Handling Comments

- Two forward slashes `//` are used to start a comment that lasts until the end
  of the line.

## Analyzer in Your Editor

- The editor may throw errors requesting that you add a Cargo.toml file when one
  exists. You may need to tell your editor were the TOML files are for your
  projects.

## The Prelude

- The prelude (a module) is a list of items that are imported into your
  application's scope by default (it's available to all of your code).
- Items in the prelude are available without being prefixed with the module
  name.
- The standard library is available to review at
  https://doc.rust-lang.org/std/prelude/index.html

- The `Result` type is an enum that is part of the prelude. It has the
  capability to allow your application to crash by calling the `expect` method.
  If you've reached a logical place to stop the application (such as an level OS
  failure) then you want the application to crash from a result (like a system
  level IO failure).

## Regarding Types (Structures/Structs)

- The book uses calls out an "instance of a type". This is interesting because
  it implies that types are not static. So, for example, a `String` is not the
  same as a `&String`. String is a type (the blueprint) that represents a
  dynamic sequence of characters. `&String` is a reference to an instance of a
  `String` (the literal thing constructed from the blueprint).

- Enums are actually useful in Rust to enforce compile-time checks.

- The `Result` type is an enum that is part of the prelude. It is used to handle
  a result that has the potential to return an error.

## Regarding Packages

- Packages are compiled into Crates.
- Crates are the compiled binary or library.
- Cargo does a lot of work to protect your project from accidental upgrades that
  could break your code.
