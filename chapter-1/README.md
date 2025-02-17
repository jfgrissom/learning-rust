# Things to remember from the Getting Started Chapter

## Installation:

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## The compiler:

```bash
$ rustc --version
```

- When you run the compiler the output executable is dropped into the current
  directory regardless of the location of the source file.
- The compiled executable is named after the source file.
- The compiled executable is automatically made executable on MacOS.

## The package manager:

```bash
$ cargo --version
```

- `cargo new` is used to create a new project.
- `cargo new` automatically creates a new directory with the name of the
  project.
- `cargo new` automatically creates an src directory inside the project
  directory.
- `cargo new` automatically creates a Cargo.toml file to manage dependencies for
  the project.
- `cargo build` should be run where the Cargo.toml file is located.
- `cargo run` builds and runs the program and at the same time provides a target
  directory that has an executable that is suitable for a specific compile
  target (at a glance it looks like the default compile target is the current
  platform).
- `cargo run` also produces a Cargo.lock file that retains the exact versions of
  the dependencies that were used to compile the program.
- `cargo check` is used to check the program without producing an executable.
- Remember `cargo new`, `cargo build`, `cargo run`, and `cargo check`.
- `cargo build --release` is used to build the program for production.

## Open the documentation in the browser:

```bash
$ rustup doc
```

## Conventions:

- Snake case is use for directory names (example_directory_name).
