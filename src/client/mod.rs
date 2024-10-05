use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use bevy::prelude::*;
use lightyear::prelude::*;
pub use lightyear::prelude::client::*;
use bevy_egui::EguiPlugin;
use bevy_editor_pls::EditorPlugin;

use crate::{
    common::{CommonPlugin, SERVER_REPLICATION_INTERVAL, build_shared_config},
    server::SERVER_ADDR,
};

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),
            EguiPlugin,
            #[cfg(debug_assertions)]
            EditorPlugin::default(),
        ))
        .add_plugins((
            CommonPlugin,
            // build_client_plugins()
            ClientPlugins::new(ClientConfig {
                shared: build_shared_config(),
                net: NetConfig::Netcode { 
                    auth: Authentication::Manual {
                        server_addr: SERVER_ADDR,
                        client_id: 0,
                        private_key: Key::default(),
                        protocol_id: 0
                    },
                    io: IoConfig {
                        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
                        ..default()
                    },
                    config: NetcodeConfig::default()
                },
                ..default()
            })
        ));
    }
}

fn build_client_plugins() -> ClientPlugins {
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };

    let io = IoConfig {
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };

    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };

    let config = ClientConfig {
        shared: build_shared_config(),
        net: net_config,
        ..default()
    };

    ClientPlugins::new(config)
}
