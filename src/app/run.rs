//! Run the app

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use crate::app::args::Args;
use crate::app::config::Config;
use crate::errors::*;
use crate::tip::*;

/// Run everything.
///
/// Steps:
///
///   * Initialize configuration.
///
///   * Initialize arguments.
///
///   * Initialize templating.
///
//    * Process each page.
///
/// Example:
///
/// ```
/// run();
/// //-> Initialize everything then process each page
/// ```
///
pub(crate) fn run() -> Result<()> {
    trace!("run()");

    // Initialize configuration
    let _config: Config = ::confy::load("writeforall")
    .chain_err(|| "error: confy load")?;

    // Initialize arguments
    let args: Args = crate::app::clap::args();
    if args.test { println!("{:?}", args); }

    // Process each tip file. 
    // TODO convert to map.
    let mut tips : Vec<Tip> = Vec::new();
    if let Some(paths) = &args.tip_list_path_buf {
        for path in paths {
            let mut x : Vec<Tip> = ::serde_json::from_reader(File::open(&path).unwrap()).unwrap();
            tips.append(&mut x)
        }
    }

    // Process each document file. 
    // TODO convert to map.
    if let Some(paths) = &args.input_list_path_buf {
        for path in paths {
            do_path(
                &args,
                &tips,
                &path,
            )?;
        }
    };
    Ok(())
}

fn do_path(
    _args: &Args, 
    tips: &Vec<Tip>,
    path: &PathBuf, 
) -> Result<()> {
    trace!("do_path → start → path: {:?}", path);

    // Read content as text
    let text: String = fs::read_to_string(&path).unwrap().parse().unwrap();
    debug!("text: {:?}", text);

    let mut score = 0.0;
    for tip in tips {
        for term in &tip.search {
            for (i, _s) in text.match_indices(term) {
                score += &tip.score;
                println!("{:?}\t{}\t{}\t{}\t{:?}", 
                    &path, 
                    i, 
                    term, 
                    &tip.score, 
                    &tip.suggest
                );
            }
        }
    }

    trace!("do_path → finish → input: {:?} → score: {}", path, score);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;
    use ::lazy_static::lazy_static;
    use crate::app::args::Args;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }

    #[test]
    fn test_run() {
        //TODO
    }

    #[test]
    fn test_do_path() {
        //TODO
    }

}
