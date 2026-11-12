use std::env;
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn prints_version_and_help() {
        assert_eq!(run(&["--version".to_string()]), 0);
        assert_eq!(run(&["--help".to_string()]), 0);
        assert_eq!(run(&[]), 0);
        assert_eq!(run(&["--unknown".to_string()]), 2);
    }
}
