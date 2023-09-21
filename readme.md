# Password Generator

A command-line tool that generates random passwords and copies them to the clipboard.

## Installation

1. Install Rust and Cargo from [rustup.rs](https://rustup.rs/).
2. Clone this repository: `git clone https://github.com/your-username/password-generator.git`
3. Navigate to the project directory: `cd password-generator`
4. Build the project: `cargo build --release`
5. Run the project: `cargo run --release`

## Usage

```
password-generator [OPTIONS] [LENGTH]
```

Generates a random password of the specified length (default: 12) and copies it to the clipboard.
Defaults to lowercase letters unless specified.

### Options

* `-h, --help`: Prints help information.
* `-v, --version`: Prints version information.
* `-u, --uppercase`: Include uppercase letters in the password 
* `-n, --number`: Include numbers letters in the password 
* `-s, --special`: Include special letters in the password 
* `-c, --no-clipboard`: Does not copy the generated password to the clipboard.

## Acknowledgments

This project uses the following third-party libraries:

* [clap](https://crates.io/crates/clap) for command-line argument parsing.
* [rand](https://crates.io/crates/rand) for random number generation.
* [clipboard](https://crates.io/crates/clipboard) for clipboard access.

Let me know if you have any other questions!
