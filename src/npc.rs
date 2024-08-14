use bevy::prelude::*;

#[derive(Component)]
pub struct Npc;

#[derive(Event)]
pub struct SpawnNpcEvent {
    pub transform: Transform
}

pub fn spawn_npc_event_system(
    mut events: EventReader<SpawnNpcEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for evt in events.read() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/npc0.png"),
                transform: evt.transform,
                ..default()
            },
            Npc
        ));
    }
}
