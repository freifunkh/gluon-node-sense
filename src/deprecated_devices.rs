use std::env;
use tokio::runtime::Runtime;

use crate::nodes_json::NodesJSONUpdate;
use nodes_parse::NodesJSON;

pub fn emit_devices() -> std::string::String {
    const DEFAULT_NODES_JSON_URL: &str = "https://harvester.ffh.zone/api/nodes.json";

    let nodes_json_url = env::var("nodes_json_url").unwrap_or(DEFAULT_NODES_JSON_URL.to_string());
    let mut wrapped_nodes_json = NodesJSONUpdate(NodesJSON::default());
    let rt = Runtime::new().unwrap();
    rt.block_on(wrapped_nodes_json.update_from_json(&nodes_json_url))
        .expect("Error: Could not update from json.");

    let tuple_list = wrapped_nodes_json.get_deprecated_nodes();

    tuple_list
        .iter()
        .map(|(nodeid, hostname)| format!("{}\t{}", nodeid, hostname))
        .collect::<Vec<String>>()
        .join("\n")
}
