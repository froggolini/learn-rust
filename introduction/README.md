## Installing Rust
- Install Rust from [this website](https://www.rust-lang.org/tools/install).
- Choose the default installation (option 1).

## Hello World in Rust
- Create the `hello.rs` file.
- Compile it using `rustc hello.rs`.
- Run it using `./hello`.

## The proper way to run Rust
- Initialize the project.

    ```bash
    froggo@froggo:~/learn-rust/introduction$ cargo init
        Created binary (application) package
    ```
- Compile it.
    ```bash
    froggo@froggo:~/learn-rust/introduction$ cargo run
    Compiling introduction v0.1.0 (/home/froggo/learn-rust/introduction)
        Finished dev [unoptimized + debuginfo] target(s) in 0.40s
        Running `target/debug/introduction`
    Hello, world!
    ```
    The command will compile as well as run the program. It compiled into `/target/debug/folder_name`.
- Run the executable.
    ```bash
    froggo@froggo:~/learn-rust/introduction$ ./target/debug/introduction 
    Hello, world!
    ```
- If we just want to build, not run it.
    ```bash
    froggo@froggo:~/learn-rust/introduction$ cargo build
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
    ```
- Build for Production.
    ```bash
    froggo@froggo:~/learn-rust/introduction$ cargo build --release
    Compiling introduction v0.1.0 (/home/froggo/learn-rust/introduction)
        Finished release [optimized] target(s) in 0.18s
    ```
- Run the Production release.
    ```bash
    froggo@froggo:~/learn-rust/introduction$ ./target/release/introduction 
    Hello, world!
    ```

## Key point
- `rustup` is version manager. 
- `rustc` is the compiler.
- `cargo` is the package manager.
- Install the Rust extension from Visual Studio Code.
- `cargo.toml` includes the application info and dependencies.
- When compiling, our file will be placed in the `/target` folder.
- `/src` folder contains our Rust source code.