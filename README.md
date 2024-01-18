Learning Rust throught the Rust book

1.2 Hello World
- Get familiarized with the compiler and formatter
- `rustc main.rs` to compile
- `rustfmt main.rs` to format according to the rust language standards

1.3 Hello Cargo
- Get familiarized with cargo, which is the Rust package manager + build system
- `cargo new` to generate template of a new cargo project
- `cargo build` to build once you've created the Cargo.toml file
- `cargo run` to build and run the program
- `cargo check` to see if your code compiles; much faster than building it completely

2. Guessing Game
- Get familiarized with basic Rust concepts:
    + Using external packages by adding them to the Cargo.toml file
    + Loops using the `loop` keyword
    + Match various values using `match` keyword (similar to switch case)
    + and more...

3.1 Variables and Mutability
- `let` and `const` can be used to declare variables and constants respectively
- `const` needs to be annotated with a type
- `let` variables can be "shadowed" with different types: `let x = "string"; let x = x.len();` is valid
- cannot change the type of a mutable variable by assignment but can shadow it, but at that point the mut in the first variable is useless

3.5 Fibonacci
- Practice fibonacci program with recursion
- Ask for input without newline by using `print` and flushing stdout
