use crate::deprecated::is_deprecated;
use nodes_parse::NodesJSON;
use reqwest::Error;

pub struct NodesJSONUpdate(pub NodesJSON);

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

    fn filter_non_deprecated(&mut self) {
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

    pub fn get_deprecated_nodes(&mut self) -> Vec<(String, String)> {
        self.filter_non_deprecated();
        self.0
            .nodes
            .iter()
            .map(|node| {
                (
                    node.nodeinfo.node_id.clone(),
                    node.nodeinfo.hostname.clone(),
                )
            })
            .collect()
    }
}
