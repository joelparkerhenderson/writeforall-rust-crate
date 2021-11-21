//! Args for the application.
//!
//! These args correspond to the matches in the file `clap.rs`.
//! We have these args in their own file in order to be flexible,
//! such as being able to start our app with other arg parsers.

use std::path::PathBuf;
use crate::types::*;

#[derive(Debug)]
pub struct Args {

    /// Test flag that sets whether the app prints diagnostics.
    /// Example: true means print diagnostics.
    pub(crate) test: bool,

    /// Log level: 0=none, 1=error, 2=warn, 3=info, 4=trace, 5=debug.
    /// Example: 5 means print debug diagnostics.
    pub(crate) log_level: Option<::log::Level>,

    /// Input pathable string list.
    /// Example glob: "inputs/**/*"
    /// Example file: "input.md"
    pub(crate) input_list_pathable_string: Option<List<PathableString>>,

    /// Input file path buf list.    
    /// This is typically calculated from `input_list_pathable_string`.
    /// Each item points to a file, not a directory, glob, etc.
    pub(crate) input_list_path_buf: Option<List<PathBuf>>,

    /// Settings for the program.
    /// Example: {"alpha" => "bravo", "charlie" => "delta"}
    pub(crate) settings: Option<Map<String, String>>,

    /// tip pathable string list.
    /// Example glob: "tips/**/*"
    /// Example file: "tip.html"
    pub(crate) tip_list_pathable_string: Option<List<PathableString>>,

    /// Tip file path buf list.
    /// This is typically calculated from `tip_list_pathable_string`.
    /// Each item points to a file, not a directory, glob, etc.
    pub(crate) tip_list_path_buf: Option<List<PathBuf>>,
}

impl ::std::default::Default for Args {
    fn default() -> Self { Self {
        test: false,
        log_level: None,
        input_list_pathable_string: None,
        input_list_path_buf: None,
        settings: None,
        tip_list_pathable_string: None,
        tip_list_path_buf: None,
    } }
}
