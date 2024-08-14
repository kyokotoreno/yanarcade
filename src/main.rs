mod npc;

use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/floor0.png"),
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 0.0),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 100.0
        }
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player0.png"),
            ..default()
        },
        Player
    ));
}

fn update(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = query.single_mut();

    let speed = 64.0 * time.delta_seconds(); // 64 pixels * seconds since last frame = 10 pixels per second

    for keycode in keyboard_input.get_pressed() {
        match keycode {
            KeyCode::KeyW => {
                transform.translation.y += speed;
            },
            KeyCode::KeyA => {
                transform.translation.x -= speed;
            },
            KeyCode::KeyS => {
                transform.translation.y -= speed;
            },
            KeyCode::KeyD => {
                transform.translation.x += speed;
            },
            _ => {}
        }
    }
}

fn spawn_npc_test (mut event: EventWriter<npc::SpawnNpcEvent>) {
    event.send(npc::SpawnNpcEvent {
        transform: Transform {
            translation: Vec3::new(0.0, 100.0, 1.0),
            ..default()
        }
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
        )
        .add_event::<npc::SpawnNpcEvent>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, spawn_npc_test)
        .add_systems(Update, update)
        .add_systems(PostUpdate, npc::spawn_npc_event_system)
        .run();
}
