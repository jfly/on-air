mod commands;
mod webcam;

use clap::{Parser, Subcommand};

/// Detect if an attached webcam is in use and optionally publish its status to a
/// MQTT broker in a way Home Assistant understands.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get(commands::get::Get),
    Mqtt(commands::mqtt::Mqtt),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Get(c) => c.execute(),
        Commands::Mqtt(c) => c.execute(),
    }
}
