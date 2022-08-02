use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup)
        .add_system(demo)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));

    commands.spawn_bundle(camera);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.3)),
        ..default()
    });
}

fn demo(mut lines: ResMut<DebugLines>) {
    lines.line_colored(
        Vec3::new(-400.0, 0.0, 0.5),
        Vec3::new(400.0, 0.0, 0.5),
        0.9,
        Color::GREEN,
    );
}
