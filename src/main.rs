//! Main

//// error-chain

// Simple and robust error handling with error-chain!

// `error_chain!` can recurse deeply, so limit it.
#![recursion_limit = "1024"]

// Import the macro. Be sure to add `error-chain` in your `Cargo.toml`.
#[macro_use]
extern crate error_chain;

// We put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// that `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

#[allow(unused_imports)]
pub use errors::*;

//// Logging

#[macro_use]
extern crate log;
extern crate env_logger;

//// Modules

#[macro_use] pub(crate) mod types; // Type aliases

pub(crate) mod app { // Application
    pub(crate) mod args; // Arguments struct, such as set via `clap`.
    pub(crate) mod clap; // Command line argument parser
    pub(crate) mod config; // Configuration struct, such as set via `confy`
    pub(crate) mod confy; // Configuration tests for loading and parsing
    pub(crate) mod run; // Run function that handles everything
}

pub(crate) mod fun { // Functions 
    pub(crate) mod from_list_pathable_string_into_list_path_buf; // from List<PathableString> into List<PathBuf>
    pub(crate) mod from_list_str_into_map_string_string; // from List<&str> into Map<String, String>
    pub(crate) mod from_set_pathable_string_into_set_path_buf; // from Set<PathableString> into Set<PathBuf>
}

pub(crate) mod tip; // Type aliases

//// Main error-chain

fn main() {
    env_logger::init();
    if let Err(ref e) = crate::app::run::run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
