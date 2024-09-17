#![allow(unused_assignments)]

use crate::minecraft::MCEdition;
use clap::{arg, crate_authors, value_parser, ArgMatches, Command};
use once_cell::sync::Lazy;
use std::{env, error::Error, path::PathBuf, process};

static LONG_VERSION: Lazy<String> = Lazy::new(|| {
    format!(
        " v{}\nAuthor(s): {}\nDescription: {}\nRepository: {}",
        env!("CARGO_PKG_VERSION"),
        crate_authors!(", "),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_REPOSITORY")
    )
});

static SHORT_VERSION: Lazy<String> = Lazy::new(|| format!(" v{}", env!("CARGO_PKG_VERSION"),));

/// Struct to describe passed command-line arguments
#[derive(Debug)]
#[allow(dead_code)]
pub struct Args {
    pub input: PathBuf,
    pub output: PathBuf,
    pub verbose: u8,
    pub threads: u8,
    pub dry_run: bool,
    pub force_convert: MCEdition,
    pub config: PathBuf,
}

impl Args {
    pub fn parse() -> Result<Args, Box<dyn Error>> {
        // Possible arguments
        // INPUT: `get_one::<PathBuf>("input")`
        // OUTPUT: `get_one::<PathBuf>("output")`
        // verbose: `get_count("verbose")`
        // threads: `get_one::<u8>("threads")`
        // dry-run: `get_flag("dry-run")`
        // force-convert: `get_one::<String>("force-convert")`
        // config: `get_one::<PathBuf>("config")`
        let matches: ArgMatches = Self::command()
            .ignore_errors(true)
            .arg(arg!(<input> "Path to input pack or directory").value_parser(value_parser!(PathBuf)))
            .arg(arg!(<output> "Path to output pack or directory").value_parser(value_parser!(PathBuf)))
            .arg(
                arg!(
                    -v --verbose ... "Turns on verbose logging (Max level of 2)"
                )
                .value_parser(value_parser!(u8).range(0..=2))
            )
            .arg(
                arg!(
                    -t --threads <NUM> "Specifies how many threads to run the converter with"
                )
                .value_parser(value_parser!(u8).range(1..))
                .default_value("1")
            )
            .arg(
                arg!(
                    -d --"dry-run" "Runs through the pack(s) but does not convert resources"
                )
            )
            .arg(
                arg!(
                    -f --"force-convert" <EDITION> "Forces the convertor to convert to specified edition"
                )
                .value_parser(["java", "bedrock"])
            )
            .arg(
                arg!(
                    -c --config <PATH> "Use your custom resource conversion tables"
                )
                .value_parser(value_parser!(PathBuf))
            )
            .arg(
                arg!(
                    --"debug-info" "Prints out debug info about host & binary"
                )
            )
            .get_matches();

        if matches.get_flag("debug-info") {
            println!("{}", Self::debug_info());
            process::exit(0);
        }

        Ok(Args {
            input: matches
                .get_one::<PathBuf>("input")
                .expect("Input path is required")
                .to_path_buf(),
            output: matches
                .get_one::<PathBuf>("output")
                .expect("Output path is required")
                .to_path_buf(),
            verbose: matches.get_count("verbose"),
            threads: *matches.get_one::<u8>("threads").unwrap_or(&1),
            dry_run: matches.get_flag("dry-run"),
            force_convert: match matches
                .get_one::<String>("force-convert")
                .map(String::as_str)
            {
                Some("java") => MCEdition::Java,
                Some("bedrock") => MCEdition::Bedrock,
                _ => MCEdition::None,
            },
            config: matches
                .get_one::<PathBuf>("config")
                .cloned()
                .unwrap_or_else(|| PathBuf::from("")),
        })
    }

    pub fn command() -> Command {
        // crate_name!() = env!("CARGO_PKG_NAME");
        // crate_version!() = env!("CARGO_PKG_VERSION");
        // crate_authors!() = env!("CARGO_PKG_AUTHORS") + custom_separator;
        // crate_description!() = env!("CARGO_PKG_DESCRIPTION");

        let cmd: Command = Command::new(env!("CARGO_PKG_NAME"))
            .author(crate_authors!(", "))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .long_about(env!("CARGO_PKG_DESCRIPTION_LONG"))
            .version(SHORT_VERSION.as_str())
            .long_version(LONG_VERSION.as_str());
        cmd
    }

    /// Display version message.
    pub fn print_version() {
        println!("{}", Self::command().render_version());
    }

    /// Display help message.
    pub fn print_help() {
        println!("{}", Self::command().render_help())
    }

    /// Prints debug info about host and binary
    fn debug_info() -> String {
        let env_list: Vec<&str> = vec![
            "CUSTOM_BIN_NAME",
            "CARGO_PKG_VERSION",
            "VERGEN_BUILD_TIMESTAMP",
            "VERGEN_GIT_SHA",
            "VERGEN_GIT_COMMIT_TIMESTAMP",
            "VERGEN_GIT_BRANCH",
            "VERGEN_CARGO_TARGET_TRIPLE",
            "VERGEN_RUSTC_CHANNEL",
            "VERGEN_RUSTC_COMMIT_DATE",
            "VERGEN_RUSTC_COMMIT_HASH",
            "VERGEN_RUSTC_SEMVER",
            "VERGEN_SYSINFO_OS_VERSION",
            "VERGEN_SYSINFO_TOTAL_MEMORY",
            "VERGEN_SYSINFO_CPU_CORE_COUNT",
            "VERGEN_SYSINFO_CPU_BRAND",
            "VERGEN_SYSINFO_CPU_FREQUENCY",
        ];
        let mut info_text: String = String::from("");

        env::set_var(
            "CUSTOM_BIN_NAME",
            env::current_exe()
                .unwrap_or("UNKNOWN".into())
                .display()
                .to_string(),
        );

        for environ in env_list {
            let mut trunc_environ: String = environ.to_string();
            trunc_environ.replace_range(..environ.find("_").unwrap() + 1, "");

            let concat_string: String = match env::var(environ) {
                Ok(value) => format!("{} = {}\n", trunc_environ, value),
                Err(_) => format!("{} = NOT_SET\n", trunc_environ),
            };

            info_text.push_str(&concat_string);
        }

        info_text
    }
}
