//! Command line argument parsing (CLAP) for the application.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! Clap has a variety of setup approachs:
//!
//!   * via typical functions, which favors advanced uses yet is verbose.
//!   * via usage strings, which looks more like writing documentation.
//!   * via macros, which is fast and less verbose, yet atypical to read.
//!   * via YAML file, which favors localization and text file readability.
//!
//! We prefer the typical functions, because they provide maximum capability,
//! and in our experience are the easiest for Rust IDEs to read and debug.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we like the separation of concerns.


use clap::{Arg, App};
use std::path::PathBuf;
use crate::app::args::Args;
use crate::types::*;
use crate::fun::from_list_pathable_string_into_list_path_buf::*;
use crate::fun::from_list_str_into_map_string_string::*;

/// Create a clap app.
pub fn app() -> App<'static> {
    App::new("Write For All")
    .version("1.0.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Write For All text style checker")
    .arg(Arg::new("input")
        .short('i')
        .long("input")
        .alias("inputs")
        .value_name("FILE | DIRECTORY | GLOB …")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .about("An input path string. Example file: --input \"input.text\" … Example directory: --input \"inputs/\" … Example glob: --input \"inputs/**/*\" …"))
    .arg(Arg::new("tip")
        .short('t')
        .long("tip")
        .alias("tips")
        .value_name("FILE | DIRECTORY | GLOB …")
        .takes_value(true)
        .multiple_occurrences(true)
        .multiple_values(true)
        .about("A tip path string. Example file: --tip \"tip.json\" … Example directory: --tip \"tips/\" … Example glob: --tip \"tips/**/*\" …"))
    .arg(Arg::new("test")
        .long("test")
        .takes_value(false)
        .about("Print test output for debugging, verifying, tracing, and the like. Example: --test …"))
    .arg(Arg::new("set")
        .short('s')
        .long("set")
        .value_names(&["NAME", "VALUE"])
        .multiple_occurrences(true)
        .multiple_values(true)
        .about("Set a variable name to a value. Example: --set pi \"3.1415\" …"))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .multiple_occurrences(true)
        .multiple_values(true)
        .takes_value(false)
        .about("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …"))
}

/// Create an Args struct initiatied with the clap App settings.
pub fn args() -> Args {
    let matches = app().get_matches();
    let mut args = Args {
        test: matches.is_present("test"),
        log_level: match matches.occurrences_of("verbose") {
            0 => None,
            1 => Some(::log::Level::Error),
            2 => Some(::log::Level::Warn),
            3 => Some(::log::Level::Info),
            4 => Some(::log::Level::Debug),
            5 => Some(::log::Level::Trace),
            _ => Some(::log::Level::Trace),
        },
        input_list_pathable_string: match matches.values_of("input") {
            Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
            _ => None,
        },
        input_list_path_buf: None, // Set below
        settings: match matches.values_of("set") {
            Some(x) => {
                let vec: Vec<&str> = x.collect();
                Some(from_list_str_into_map_string_string(&vec))
            },
            _ => None,
        },
        tip_list_pathable_string: match matches.values_of("tip") {
            Some(x) => Some(x.map(|x| PathableString::from(x)).collect::<List<PathableString>>()),
            _ => None,
        },
        tip_list_path_buf: None, // Set below
    };

    if let Some(ref x) = args.input_list_pathable_string {
        args.input_list_path_buf = Some(from_list_pathable_string_into_list_path_buf(x));
    }

    if let Some(ref x) = args.tip_list_pathable_string {
        args.tip_list_path_buf = Some(from_list_pathable_string_into_list_path_buf(x));
    }

    args    
}
