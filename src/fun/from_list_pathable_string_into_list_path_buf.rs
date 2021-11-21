use std::path::PathBuf;
use crate::types::*;

/// Convert from &List<PathableString> into List<PathBuf>.
/// 
/// Example:
//
/// ```rust
/// let from: List<PathableString> = vec!["a/*", "b/*"];
/// let into: List<PathBuf> = from_list_pathable_string_into_list_path_buf(list_pathable_string);
/// //=> ["a", "a/a1.txt", "a/a2.txt", "b", "b/b1.txt", "b/b2.txt"]
/// ```
///
/// This function deliberately ignores errors.
///
#[allow(dead_code)]
pub fn from_list_pathable_string_into_list_path_buf(from: &List<PathableString>) -> List<PathBuf> {
    let x: List<PathBuf> = from.iter().flat_map(|pathable_string| 
        ::glob::glob(&pathable_string).unwrap().filter_map(|x| x.ok())
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
    fn test_from_list_pathable_string_into_list_path_buf() {
        let dir_as_buf = TESTS_DIR.join("function").join("from_list_pathable_string_into_list_path_buf");
        let dir_as_string = dir_as_buf.to_string_lossy();
        let from: List<PathableString> = list![
            format!("{}{}", dir_as_string, "/a/**/*"),
            format!("{}{}", dir_as_string, "/b/**/*")
        ];
        let actual: List<PathBuf> = from_list_pathable_string_into_list_path_buf(&from);
        let expect: List<PathBuf> = list![
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
            dir_as_buf.join("b/bb/bbb")
        ];
        assert_eq!(actual, expect);
    }

}
