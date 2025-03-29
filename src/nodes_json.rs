use crate::deprecated::is_deprecated;
use nodes_parse::NodesJSON;
use reqwest::Error;
use serde::Serialize;

pub struct NodesJSONUpdate(pub NodesJSON);

#[derive(Serialize)]
pub struct Node {
    pub hostname: String,
    pub version: String,
    pub status: String,
    pub node_id: String,
}

impl NodesJSONUpdate {
    pub async fn update_from_json(&mut self, url: &str) -> Result<(), Error> {
        let response = reqwest::get(url).await?;

        if response.status().is_success() {
            let json: NodesJSON = response.json().await?;
            let wrapped = Self(json);
            *self = wrapped;
        } else {
            eprintln!("Failed to download JSON.");
        }

        Ok(())
    }

    pub fn filter_non_deprecated(&mut self) {
        self.0.nodes.retain(|node| {
            if let Some(model) = &node.nodeinfo.hardware.model {
                is_deprecated(model)
            } else {
                //TODO have enviroment variable to silence warning
                eprintln!(
                    "Warning: Node ID {} (Hostname: {}) removed due to missing model.",
                    node.nodeinfo.node_id, node.nodeinfo.hostname
                );
                false
            }
        });
    }

    pub fn get_deprecated_nodes(&mut self) -> Vec<Node> {
        self.filter_non_deprecated();
        self.0
            .nodes
            .iter()
            .map(|node| Node {
                hostname: node.nodeinfo.hostname.clone(),
                version: "versio pending".to_string(),
                status: "deprecated".to_string(),
                node_id: node.nodeinfo.node_id.clone(),
            })
            .collect()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.0
            .nodes
            .iter()
            .map(|node| Node {
                hostname: node.nodeinfo.hostname.clone(),
                version: node
                    .nodeinfo
                    .software
                    .firmware
                    .release
                    .clone()
                    .unwrap_or("not available".to_string()),
                status: "deprecated".to_string(),
                node_id: node.nodeinfo.node_id.clone(),
            })
            .collect()
    }
}
