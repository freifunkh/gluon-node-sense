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
    "A5-V11",
    "AP121",
    "AP121U",
    "D-Link DIR-615",
    "D-Link DIR-615 D",
    "AVM FRITZ!Box 7320",
    "AVM FRITZ!Box 7330",
    "AVM FRITZ!Box 7330 SL",
    "TP-Link TL-MR13U v1",
    "TP-Link TL-MR3020 v1",
    "TP-Link TL-MR3040 v1",
    "TP-Link TL-MR3040 v2",
    "TP-Link TL-MR3220 v1",
    "TP-Link TL-MR3220 v2",
    "TP-Link TL-MR3420 v1",
    "TP-Link TL-MR3420 v2",
    "TP-Link TL-WA701N/ND v1",
    "TP-Link TL-WA701N/ND v2",
    "TP-Link TL-WA730RE v1",
    "TP-Link TL-WA750RE v1",
    "TP-Link TL-WA801N/ND v1",
    "TP-Link TL-WA801N/ND v2",
    "TP-Link TL-WA801N/ND v3",
    "TP-Link TL-WA830RE v1",
    "TP-Link TL-WA830RE v2",
    "TP-Link TL-WA850RE v1",
    "TP-Link TL-WA860RE v1",
    "TP-Link TL-WA901N/ND v1",
    "TP-Link TL-WA901N/ND v2",
    "TP-Link TL-WA901N/ND v3",
    "TP-Link TL-WA901N/ND v4",
    "TP-Link TL-WA901N/ND v5",
    "TP-Link TL-WA7210N v2",
    "TP-Link TL-WA7510N v1",
    "TP-Link TL-WR703N v1",
    "TP-Link TL-WR710N v1",
    "TP-Link TL-WR710N v2",
    "TP-Link TL-WR710N v2.1",
    "TP-Link TL-WR740N/ND v1",
    "TP-Link TL-WR740N/ND v3",
    "TP-Link TL-WR740N/ND v4",
    "TP-Link TL-WR740N/ND v5",
    "TP-Link TL-WR741N/ND v1",
    "TP-Link TL-WR741N/ND v3",
    "TP-Link TL-WR741N/ND v4",
    "TP-Link TL-WR741N/ND v5",
    "TP-Link TL-WR743N/ND v1",
    "TP-Link TL-WR743N/ND v2",
    "TP-Link TL-WR840N v2",
    "TP-Link TL-WR841N/ND v3",
    "TP-Link TL-WR841N/ND v5",
    "TP-Link TL-WR841N/ND v7",
    "TP-Link TL-WR841N/ND v8",
    "TP-Link TL-WR841N/ND v9",
    "TP-Link TL-WR841N/ND v10",
    "TP-Link TL-WR841N/ND v11",
    "TP-Link TL-WR841N/ND v12",
    "TP-Link TL-WR841N/ND Mod (16M) v11",
    "TP-Link TL-WR841N/ND Mod (16M) v10",
    "TP-Link TL-WR841N/ND Mod (16M) v8",
    "TP-Link TL-WR841N/ND Mod (16M) v9",
    "TP-Link TL-WR841N/ND Mod (8M) v10",
    "TP-Link TL-WR842N/ND v1",
    "TP-Link TL-WR842N/ND v2",
    "TP-Link TL-WR843N/ND v1",
    "TP-Link TL-WR940N v1",
    "TP-Link TL-WR940N v2",
    "TP-Link TL-WR940N v3",
    "TP-Link TL-WR940N v4",
    "TP-Link TL-WR940N v5",
    "TP-Link TL-WR940N v6",
    "TP-Link TL-WR941N/ND v2",
    "TP-Link TL-WR941N/ND v3",
    "TP-Link TL-WR941N/ND v4",
    "TP-Link TL-WR941N/ND v5",
    "TP-Link TL-WR941N/ND v6",
    "TP-Link TL-WR1043N/ND v1",
    "D-Link DIR-615 D1",
    "D-Link DIR-615 D2",
    "D-Link DIR-615 D3",
    "D-Link DIR-615 D4",
    "D-Link DIR-615 H1",
    "Ubiquiti NanoStation loco M2",
    "Ubiquiti NanoStation M2",
    "Ubiquiti PicoStation M2",
    "Ubiquiti Bullet M",
    "Ubiquiti Bullet M2",
    "Ubiquiti AirRouter",
    "VoCore 8M",
    "VoCore 16M",
    "WD My Net N600",
    "WD My Net N750"
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

