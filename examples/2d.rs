use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup.system())
        .add_system(demo.system())
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));

    commands.spawn_bundle(camera);
}

fn demo(mut lines: ResMut<DebugLines>) {
    lines.line(
        Vec3::new(-400.0, 200.0, 0.0),
        Vec3::new(400.0, 200.0, 0.0),
        0.0,
    ); // Units are generally "smaller" for 2d, so thickness should be higher.
    lines.line_colored(
        Vec3::new(-400.0, 0.0, 0.0),
        Vec3::new(400.0, 0.0, 0.0),
        0.0,
        Color::GREEN,
    );
    lines.line_gradient(
        Vec3::new(-400.0, -200.0, 0.0),
        Vec3::new(400.0, -200.0, 0.0),
        0.0,
        Color::WHITE,
        Color::PINK,
    );
}
