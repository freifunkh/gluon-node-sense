mod deprecated;
mod deprecated_devices;
mod handlers;
mod nodes_json;

use actix_web::{web::Data, App, HttpServer};
use clap::{Parser, Subcommand};
use deprecated::emit_json;
use deprecated_devices::emit_devices;
use std::io;
use tera::Tera;

#[derive(Parser)]
#[command(
    name = "freifunk-node-sense",
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
    let tera = Data::new(Tera::new("./web/templates/**/*.html").unwrap());
    HttpServer::new(move || App::new().app_data(tera.clone()).service(handlers::index))
        .bind("127.0.0.1:8080")?
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
