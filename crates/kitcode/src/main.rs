use std::env;
use std::process::ExitCode;

#[cfg(test)]
mod tests;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "CLI entrypoint writes user-facing output directly to stdout/stderr"
)]
fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("-V" | "--version") => {
            println!("kitcode {VERSION}");
            0
        }
        Some("-h" | "--help") => {
            println!(
                "KitCode Agent {VERSION}\n\nUsage: kitcode [OPTIONS]\n\nOptions:\n  -h, --help     Print help\n  -V, --version  Print version"
            );
            0
        }
        None => {
            println!("KitCode Agent is an AI agent built in Rust.");
            0
        }
        Some(other) => {
            eprintln!("Unknown argument: {other}");
            2
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = run(&args);
    ExitCode::from(code as u8)
}
