use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use bevy::{prelude::*, log, log::LogPlugin};
use lightyear::{
    prelude::*,
    server::NetcodeConfig,
};
use crate::common::{CommonPlugin, SERVER_REPLICATION_INTERVAL, build_shared_config};

pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            LogPlugin {
                level: log::Level::INFO,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".into(),
                ..default()
            }
        ))
        .add_plugins(((
            CommonPlugin,
            ServerPlugins::new(ServerConfig {
                shared: build_shared_config(),
                net: vec![NetConfig::Netcode {
                    io: IoConfig {
                        transport: ServerTransport::UdpSocket(SERVER_ADDR),
                        ..default()
                    },
                    config: NetcodeConfig::default(),
                }],
                replication: ReplicationConfig {
                    send_interval: SERVER_REPLICATION_INTERVAL,
                    ..default()
                },
                ..default()
            }),
        )))
        .add_systems(Startup, startup_system);
    }
}

/// Startup system.
fn startup_system(mut commands: Commands) {
    commands.start_server();
}
