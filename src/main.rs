mod deprecated;

use deprecated::emit_json;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "freifunk-node-sense", version = "0.0.1", about = "Sensing problems and opportunities in Freifunk nodes.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show deprecated models as json
    ShowDeprecated,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ShowDeprecated => {
            println!("{}", emit_json());
        }
    }
}
