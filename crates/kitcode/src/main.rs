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
fn run(args: &[String]) -> u8 {
    match args {
        [] => {
            println!("KitCode Agent is an AI agent built in Rust.");
            0
        }
        [arg] if matches!(arg.as_str(), "-V" | "--version") => {
            println!("kitcode {VERSION}");
            0
        }
        [arg] if matches!(arg.as_str(), "-h" | "--help") => {
            println!(
                "KitCode Agent {VERSION}\n\nUsage: kitcode [OPTIONS]\n\nOptions:\n  -h, --help     Print help\n  -V, --version  Print version"
            );
            0
        }
        [arg, ..] => {
            eprintln!("Unknown argument: {arg}");
            2
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    ExitCode::from(run(&args))
}
