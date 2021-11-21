//! Configuration for the application.
//!
//! Our source code convention is using this file `config.rs` to
//! define `struct Config` and `impl ::std::default::Default`.
//!
//! We manage configuration file settings via the `confy` crate.
//! See the project file `confy.rs` for testing our `confy` loading.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    version: u8,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self {
        version: 1,
    } }
}
