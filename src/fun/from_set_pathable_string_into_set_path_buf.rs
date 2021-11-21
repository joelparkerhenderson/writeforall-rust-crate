use std::path::PathBuf;
use crate::types::*;

/// Convert from &Set<GlobString> into Set<PathBuf>.
/// 
/// Example:
//
/// ```rust
/// let globs = vec!["a/*", "b/*"];
/// let paths = globs_to_paths(globs);
/// //=> ["a", "a/a1.txt", "a/a2.txt", "b", "b/b1.txt", "b/b2.txt"]
/// ```
///
/// This function deliberately ignores errors.
///
#[allow(dead_code)]
pub fn from_set_pathable_string_into_set_path_buf(glob_string_set: &Set<GlobString>) -> Set<PathBuf> {
    let x: Set<PathBuf> = glob_string_set.iter().flat_map(|glob_string| 
        ::glob::glob(&glob_string).unwrap().filter_map(|x| x.ok())
    ).collect::<_>();
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::*;

    lazy_static! {
        pub static ref TESTS_DIR: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>();
    }    

    #[test]
    fn test_from_set_pathable_string_into_set_path_buf() {
        let dir_as_buf = TESTS_DIR.join("function").join("from_set_pathable_string_into_set_path_buf");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let from: Set<PathableString> = set![
            format!("{}{}", dir_as_string, "/a/**/*"),
            format!("{}{}", dir_as_string, "/b/**/*")
        ];
        let actual: Set<PathBuf> = from_set_pathable_string_into_set_path_buf(&from);
        let expect: Set<PathBuf> = set![
            dir_as_buf.join("a/aa"),
            dir_as_buf.join("a/aa/aaa"),
            dir_as_buf.join("a/aa/aab"),
            dir_as_buf.join("a/ab"),
            dir_as_buf.join("a/ab/aba"),
            dir_as_buf.join("a/ab/abb"),
            dir_as_buf.join("b/ba"),
            dir_as_buf.join("b/ba/baa"),
            dir_as_buf.join("b/ba/bab"),
            dir_as_buf.join("b/bb"),
            dir_as_buf.join("b/bb/bba"),
            dir_as_buf.join("b/bb/bbb"),
            dir_as_buf.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

}
