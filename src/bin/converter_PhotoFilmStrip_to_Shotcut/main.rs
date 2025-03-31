//! src/bin/converter_PhotoFilmStrip_to_Shotcut/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

// Linux terminal colors
use converter_PhotoFilmStrip_to_Shotcut::{GREEN, RED, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                print_greet_name(greet_name);
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(greet_name) => {
                // this can return an error. Here is the last place I can deal with the error.
                match upper_greet_name(greet_name) {
                    // do nothing
                    Ok(()) => (),
                    // log error from anyhow
                    Err(err) => println!("{RED}Error: {err}{RESET}"),
                }
            }
            None => println!("{RED}Error: Missing arguments `greet_name`.{RESET}"),
        },
        _ => println!("{RED}Error: Unrecognized arguments. Try `converter_PhotoFilmStrip_to_Shotcut --help`{RESET}"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to converter_PhotoFilmStrip_to_Shotcut !
    This is a simple yet complete template for a CLI program written in Rust.{RESET}

{GREEN}converter_PhotoFilmStrip_to_Shotcut --help{RESET}
{GREEN}converter_PhotoFilmStrip_to_Shotcut print world{RESET}
{GREEN}converter_PhotoFilmStrip_to_Shotcut upper world{RESET}

    {YELLOW}This command should return an error:{RESET}
{GREEN}converter_PhotoFilmStrip_to_Shotcut upper WORLD{RESET}
  
    {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
}

/// print my name
fn print_greet_name(greet_name: &str) {
    // call the function from the `lib.rs`
    println!(
        "{}",
        converter_PhotoFilmStrip_to_Shotcut::format_hello_phrase(greet_name)
    );
}

/// print my name upper, can return error
fn upper_greet_name(greet_name: &str) -> anyhow::Result<()> {
    // the function from `lib.rs`, can return error
    // use the ? syntax to bubble the error up one level or continue (early return)
    let upper = converter_PhotoFilmStrip_to_Shotcut::format_upper_hello_phrase(greet_name)?;
    println!("{}", upper);
    // return
    Ok(())
}
