mod common;
mod client;
mod server;

use client::ClientPlugin;
use server::ServerPlugin;
use bevy::prelude::*;
use clap::Parser;

#[derive(Parser, PartialEq, Debug)]
enum Cli {
    /// Run the game as a server
    Server,
    /// Run the game as a client
    Client,
}

fn main() {
    let mut app = App::new();

    let cli = Cli::parse();

    match cli {
        Cli::Server => app.add_plugins(ServerPlugin),
        Cli::Client => app.add_plugins(ClientPlugin),
    };

    app.run();
}
