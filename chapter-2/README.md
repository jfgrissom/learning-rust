# Things to remember from the "Programming is a Guessing Game" chapter.

## Managing Rust Compiler Versions

- `rustup` is the command line tool for installing Rust and managing Rust
  versions. Figured this out just using the tool during this chapter's
  exercises.

## Handling Strings.

- When using `println!` macro the string `"You guessed: {guess}"` works like a
  JavaScript/Typescript template literal (example:
  `` `You guessed: ${guess}` ``).

## Handling Variables.

- Variables are immutable by default. To make a variable mutable, use the `mut`
  keyword.

## Handling Comments.

- Two forward slashes `//` are used to start a comment that lasts until the end
  of the line.

## Analyzer in your editor.

- The editor may throw errors requesting that you add a Cargo.toml file when one
  exists. You may need to tell your editor were the TOML files are for your
  projects.
