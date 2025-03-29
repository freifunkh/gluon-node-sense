use crate::nodes_json::Node;
use crate::nodes_json::NodesJSONUpdate;
use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::RwLock;

use tera::{Context, Tera};

#[get("/")]
pub async fn index(
    _req: HttpRequest,
    tera: Data<Tera>,
    nodes_json_arc_rw: Data<Arc<RwLock<NodesJSONUpdate>>>,
) -> impl Responder {
    let mut ctx = Context::new();
    let nodes: Vec<Node> = nodes_json_arc_rw.read().await.get_nodes();

    ctx.insert("nodes", &nodes);
    HttpResponse::Ok().body(tera.render("index.html", &ctx).unwrap())
}
