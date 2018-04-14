extern crate chrono;
extern crate rusty_ulid;

/// Contains functions for encoding and decoding of
/// [crockford Base32][crockford] strings.
///
/// [crockford]: https://crockford.com/wrmg/base32.html
pub mod crockford;

pub use rusty_ulid::Ulid;
pub use rusty_ulid::new_ulid_string;
use std::str::FromStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");
static HELP: &str = "rust_ulid

Usage:
    rusty_ulid [options]
        Generate a ULID.

    rusty_ulid [options] <args>...
        Check ULIDs given as args.

Options:
    -h, --help          Display this message and exit
    -V, --version       Print version info and exit
    -v, --verbose       Use verbose output
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let exit_code = main_with_args_and_return_value(args);
    std::process::exit(exit_code);
}

fn main_with_args_and_return_value(args: Vec<String>) -> i32 {
    let mut verbose: bool = false;
    let mut help: bool = false;
    let mut version: bool = false;
    let mut ulid_candidates = Vec::<String>::new();

    for arg in args {
        let argument: &str = &arg;
        match argument {
            "-v" => verbose = true,
            "--verbose" => verbose = true,
            "-h" => help = true,
            "--help" => help = true,
            "-V" => version = true,
            "--version" => version = true,
            _ => ulid_candidates.push(argument.to_string()),
        }
    }

    if version {
        println!("rusty_ulid {}", VERSION);
        return 0;
    }

    if help {
        println!("{}", HELP);
        return 0;
    }

    if ulid_candidates.is_empty() {
        // not checking, producing
        let ulid = Ulid::new();
        println!("{}", ulid);
        if verbose {
            println!("{}", ulid.datetime());
        }
        return 0;
    }

    let mut broken = Vec::<String>::new();
    for candidate in ulid_candidates {
        let result = Ulid::from_str(&candidate);
        if let Ok(ulid) = result {
            if verbose {
                println!("{}\n{}\n", ulid, ulid.datetime());
            }
        } else {
            broken.push(candidate);
        }
    }

    if !broken.is_empty() {
        eprintln!("Invalid ULID strings: {:?}", broken);
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_value_returns_error() {
        let args = vec!["foo".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 1);
    }

    #[test]
    fn valid_values_return_no_error() {
        let args = vec![
            "01CB265DSMTDS096TBTZRNTBPC".to_string(),
            "01CB265J6CRQA44WH98DP3YA07".to_string(),
        ];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_verbose_values_return_no_error() {
        let args = vec![
            "01CB265DSMTDS096TBTZRNTBPC".to_string(),
            "--verbose".to_string(),
            "01CB265J6CRQA44WH98DP3YA07".to_string(),
        ];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn no_args_return_no_error() {
        let args = vec![];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn verbose_short_returns_no_error() {
        let args = vec!["-v".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn verbose_long_returns_no_error() {
        let args = vec!["--verbose".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn version_short_returns_no_error() {
        let args = vec!["-V".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn version_long_returns_no_error() {
        let args = vec!["--version".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn help_short_returns_no_error() {
        let args = vec!["-h".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }

    #[test]
    fn help_long_returns_no_error() {
        let args = vec!["--help".to_string()];

        let result = main_with_args_and_return_value(args);
        assert_eq!(result, 0);
    }
}
