use crate::nodes_json::Node;
use crate::nodes_json::NodesJSONUpdate;
use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

use tera::{Context, Tera};

#[derive(Debug, Default, Deserialize)]
pub struct SortQuery {
    asc: Option<bool>,
    cmp: Option<String>,
    q: Option<String>,
}

#[get("/")]
pub async fn index(
    query: Query<SortQuery>,
    tera: Data<Tera>,
    nodes_json_arc_rw: Data<Arc<RwLock<NodesJSONUpdate>>>,
) -> impl Responder {
    let mut ctx = Context::new();
    let mut nodes: Vec<Node> = nodes_json_arc_rw.read().await.get_nodes();

    let ascending = query.asc.unwrap_or(true);
    let comparator: String = query.cmp.clone().unwrap_or("hostname".to_string());
    let query_string: Option<String> = query.q.clone();

    if let Some(string) = query_string {
        nodes.retain(|node| {
            node.hostname.contains(&string)
                || node.version.contains(&string)
                || node.status.contains(&string)
                || node.node_id.contains(&string)
        });
    };

    nodes.sort_by(|a, b| {
        let cmp = match comparator.as_str() {
            "Router" => a.hostname.to_lowercase().cmp(&b.hostname.to_lowercase()),
            "Version" => a.version.cmp(&b.version),
            "Status" => a.status.cmp(&b.status),
            "Meshviewer" => a.node_id.cmp(&b.node_id),
            _ => a.hostname.cmp(&b.hostname),
        };
        if ascending {
            return cmp;
        }
        cmp.reverse()
    });

    let table_header = gen_table_header(&query, nodes.len());

    ctx.insert("nodes", &nodes);
    ctx.insert("table_header", &table_header);
    HttpResponse::Ok().body(tera.render("index.html", &ctx).unwrap())
}

fn gen_table_header(query: &SortQuery, routers: usize) -> String {
    let mut res = "<thead>
        <tr>"
        .to_string();
    let headers = vec!["Router", "Version", "Status", "Meshviewer"];

    let asc = query.asc.unwrap_or(true);
    let symbol = if asc { "▲" } else { "▼" };
    let inverse_string = if asc { "false" } else { "true" };
    let cmp = query.cmp.clone().unwrap_or("Router".to_string());
    let q = query.q.clone();

    for header in headers {
        res.push_str(&format!(
            "<th hx-trigger=\"click\" hx-get=\"/deprecated_list?asc={inverse_string}&cmp={header}"
        ));
        if let Some(ref query_string) = q {
            res.push_str(&format!("&q={query_string}"));
        }
        res.push_str(&format!("\" hx-target=\"#frame-wide\" hx-swap=\"innerHTML\" hx-push-url=\"?asc={inverse_string}&cmp={header}"));
        if let Some(ref query_string) = q {
            res.push_str(&format!("&q={query_string}"));
        }
        res.push_str(&format!("\" >{header}"));
        if header == "Router" {
            res.push_str(&format!(" ({routers})"));
        };
        if header == cmp {
            res.push_str(&format!(" {symbol}"));
        };
        res.push_str("</th>");
    }
    res.push_str(
        "
        </tr>
        </thead>
    ",
    );
    res
}

#[get("/deprecated_list")]
pub async fn deprecated_list(
    query: Query<SortQuery>,
    tera: Data<Tera>,
    nodes_json_arc_rw: Data<Arc<RwLock<NodesJSONUpdate>>>,
) -> impl Responder {
    let mut ctx = Context::new();
    let r_nodes: Vec<Node> = nodes_json_arc_rw.read().await.get_nodes();
    let mut nodes = r_nodes.clone();

    let ascending = query.asc.unwrap_or(true);
    let comparator: String = query.cmp.clone().unwrap_or("hostname".to_string());
    let query_string: Option<String> = query.q.clone();

    if let Some(string) = query_string {
        let lower = string.to_lowercase();
        nodes.retain(|node| {
            node.hostname.to_lowercase().contains(&lower)
                || node.version.to_lowercase().contains(&lower)
                || node.status.to_lowercase().contains(&lower)
                || node.node_id.to_lowercase().contains(&lower)
        });
    };

    nodes.sort_by(|a, b| {
        let cmp = match comparator.as_str() {
            "Router" => a.hostname.to_lowercase().cmp(&b.hostname.to_lowercase()),
            "Version" => a.version.cmp(&b.version),
            "Status" => a.status.cmp(&b.status),
            "Meshviewer" => a.node_id.cmp(&b.node_id),
            _ => a.hostname.cmp(&b.hostname),
        };
        if ascending {
            return cmp;
        }
        cmp.reverse()
    });

    let table_header = gen_table_header(&query, nodes.len());

    ctx.insert("nodes", &nodes);
    ctx.insert("table_header", &table_header);
    HttpResponse::Ok().body(
        tera.render("components/deprecated_list.html", &ctx)
            .unwrap(),
    )
}
