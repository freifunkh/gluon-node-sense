use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
struct DeprecatedList {
    deprecated: Vec<String>,
}

lazy_static! {
    static ref DEPRECATED_SET: HashSet<String> = {
        let json_data = r#"
{
  "deprecated": [
  ]
}"#;

let parsed: DeprecatedList = serde_json::from_str(json_data).expect("Failed to parse JSON");
        parsed.deprecated.into_iter().collect()
    };
}

pub fn is_deprecated(model: &str) -> bool {
    DEPRECATED_SET.contains(model)
}

pub fn emit_json() -> String {
    let deprecated_list: Vec<String> = DEPRECATED_SET.iter().cloned().collect();
    let data = DeprecatedList { deprecated: deprecated_list };
    serde_json::to_string_pretty(&data).expect("Failed to serialize JSON")
}

