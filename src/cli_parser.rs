use std::{env, process};

const CRATE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const CRATE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
/// Struct to hold command-line arguments.
pub struct Args {
    pub verbose: i32,
}

impl Args {
    /// Parse the command-line arguments
    pub fn parse() -> Result<Self, String> {
        let mut verbose: i32 = 0;
        let mut help: bool = false;
        let mut version: bool = false;

        let args: Vec<String> = env::args().skip(1).collect(); // Skip program name

        for arg in &args {
            match arg.as_str() {
                "-v"  | "--verbose" => verbose = 1,
                "-v2" | "--verbose2" => verbose = 2,
                "-h"  | "--help" => help = true,
                "-V"  | "--version" => version = true,
                _ => return Err(format!("Invalid argument: {}", arg)),
            }
        }

        if help {
            Self::print_help();
            process::exit(0);
        }

        if version {
            Self::print_version();
            process::exit(0);
        }

        Ok(Self {
            verbose,
        })
    }

    /// Display version information.
    pub fn print_version() {
        println!("{CRATE_NAME}    v{CRATE_VERSION}");
        println!("{CRATE_DESCRIPTION}");
        println!("{CRATE_AUTHORS}");
        println!("{CRATE_REPO}");
    }

    /// Display help message.
    pub fn print_help() {
        Self::print_version();
        println!();
        println!("Usage: {CRATE_NAME}[.exe] [OPTIONS] {{INPUT_PACK_DIR}} {{OUTPUT_PACK_DIR}}");
        println!();
        println!("Options:");
        println!("    -v,  --verbose     Enable verbose mode");
        println!("    -v2, --verbose2    Enable SUPER verbose mode (dev purposes only)");
        println!("    -h,  --help        Print help information");
        println!("    -V,  --version     Print version information");
    }
}
