use bevy::prelude::*;

#[derive(Component)]
pub struct Npc;

#[derive(Event)]
pub struct SpawnNpcEvent;

pub fn spawn_npc(
    evt: EventReader<SpawnNpcEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/npc0.png"),
            ..default()
        },
        Npc
    ));
}
