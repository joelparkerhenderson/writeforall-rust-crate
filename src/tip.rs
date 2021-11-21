use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tip {
    pub search: Vec<String>,
    pub score: f32,
    pub suggest: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_str() {
        let data = r#"
        {
                "search": [ "alpha", "bravo" ],
                "score": 0.5,
                "suggest": [ "charlie", "delta" ]
        }"#;
        let x: Tip = serde_json::from_str(data).unwrap();
        assert_eq!(x.search, vec!["alpha", "bravo"]);
        assert_eq!(x.score, 0.5);
        assert_eq!(x.suggest, vec!["charlie", "delta"]);
    }
}