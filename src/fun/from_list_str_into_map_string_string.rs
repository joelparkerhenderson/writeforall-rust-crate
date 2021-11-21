use crate::types::*;

/// Convert from &Vec<&str> into Map<String, String>.
/// 
/// Example:
//
/// ```rust
/// let from: Vec<&str> = vec!["alpha", "bravo", "charlie", "delta"];
/// let int: Map<String, String> = from_list_str_into_map_string_string(&from);
/// //=> ["alpha" => "bravo", "charlie" => "delta"]
/// ```
///
pub fn from_list_str_into_map_string_string(vec_str: &Vec<&str>) -> Map<String, String> {
    let mut map: Map<String, String> = Map::new();
    for i in (0..vec_str.len()-1).step_by(2) {
        let k = String::from(vec_str[i]);
        let v = String::from(vec_str[i+1]);
        map.insert(k, v);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_list_str_into_map_string_string() {
        let from: Vec<&str> = vec!["alpha", "bravo", "charlie", "delta"];
        let actual: Map<String, String> = from_list_str_into_map_string_string(&from);
        let mut expect: Map<String, String> = Map::new();
        expect.insert("alpha".into(), "bravo".into());
        expect.insert("charlie".into(), "delta".into());
        assert_eq!(actual, expect);
    }

}
