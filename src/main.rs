mod deprecated;
mod deprecated_devices;
mod handlers;
mod nodes_json;

use crate::nodes_json::NodesJSONUpdate;
use actix_web::{web::Data, App, HttpServer};
use clap::{Parser, Subcommand};
use deprecated::emit_json;
use deprecated_devices::emit_devices;
use nodes_parse::NodesJSON;
use std::env;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tera::Tera;
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Parser)]
#[command(
    name = "gluon-node-sense",
    version = "0.0.1",
    about = "Sensing problems and opportunities in Freifunk nodes."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server,
    /// Show deprecated models as json
    ShowDeprecated,
    ShowDeprecatedDevices,
}

async fn start_server() -> io::Result<()> {
    const DEFAULT_NODES_JSON_URL: &str = "https://harvester.ffh.zone/api/nodes.json";
    let nodes_json_url: String =
        env::var("NODES_JSON_URL").unwrap_or(DEFAULT_NODES_JSON_URL.to_string());
    const DEFAULT_BIND_HOST: &str = "0.0.0.0";
    const DEFAULT_BIND_PORT: &str = "8080";
    let bind_host: String = env::var("BIND_HOST").unwrap_or(DEFAULT_BIND_HOST.to_string());
    let bind_port: String = env::var("BIND_PORT").unwrap_or(DEFAULT_BIND_PORT.to_string());
    let bind_endpoint: String = format!("{}:{}", bind_host, bind_port);
    const DEFAULT_NODES_JSON_POLL_INTERVAL_S: u64 = 60;
    let nodes_json_poll_interval_s = env::var("NODES_JSON_POLL_INTERVAL_S")
        .ok()
        .and_then(|interval| interval.parse::<u64>().ok())
        .unwrap_or(DEFAULT_NODES_JSON_POLL_INTERVAL_S);

    let wrapped_nodes_json = NodesJSONUpdate(NodesJSON::default());
    let nodes_json = Arc::new(RwLock::new(wrapped_nodes_json));
    let mut interval = interval(Duration::from_secs(nodes_json_poll_interval_s));

    let nodes_json_clone = Arc::clone(&nodes_json);
    let _maintainance = tokio::task::spawn(async move {
        loop {
            interval.tick().await;
            {
                let mut nodes_json_writer = nodes_json_clone.write().await;
                if let Err(e) = nodes_json_writer.update_from_json(&nodes_json_url).await {
                    println!("Error updating struct: {:?}", e);
                } else {
                    nodes_json_writer.filter_non_deprecated();
                }
            }
        }
    });

    let tera = Data::new(Tera::new("./web/templates/**/*.html").unwrap());
    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .app_data(Data::new(nodes_json.clone()))
            .service(handlers::index)
            .service(handlers::deprecated_list_and_bar)
            .service(actix_files::Files::new("/static", "./web/static").show_files_listing())
    })
    .bind(bind_endpoint)?
    .run()
    .await
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server => {
            let _ = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(start_server());
        }
        Commands::ShowDeprecated => {
            println!("{}", emit_json());
        }
        Commands::ShowDeprecatedDevices => {
            println!("{}", emit_devices());
        }
    }
}
