//! Configuration test of the `confy` crate for the application.
//!
//! Our source code convention is using this file `confy.rs`
//! in order to test configuration file loading and parsing.
//!
//! See also the project file `config.rs` for our `Config` struct.

#[cfg(test)]
mod tests {
    use crate::app::config::Config;

    #[test]
    fn test_confy() {
        let config: Config = ::confy::load("writeforall").unwrap();
        dbg!(config);
        // assert_eq!(i_string, o_string); //TODO
    }

}