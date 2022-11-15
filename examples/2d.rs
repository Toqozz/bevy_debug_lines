use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup)
        .add_system(demo)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });
}

fn demo(mut lines: ResMut<DebugLines>) {
    lines.line(Vec3::new(-400.0, 200.0, 0.0), Vec3::new(400.0, 200.0, 0.0));
    lines
        .line(Vec3::new(-400.0, 0.0, 0.0), Vec3::new(400.0, 0.0, 0.0))
        .duration(0.9)
        .color(Color::GREEN);
    lines
        .line(
            Vec3::new(-400.0, -200.0, 0.0),
            Vec3::new(400.0, -200.0, 0.0),
        )
        .gradient(Color::WHITE, Color::PINK);
    lines
        .line(Vec3::new(-100.0, 100.0, 0.0), Vec3::new(100.0, -100.0, 0.0))
        .duration(0.8)
        .gradient(Color::WHITE, Color::PINK);
    lines
        .line(Vec3::new(-100.0, -100.0, 0.0), Vec3::new(100.0, 100.0, 0.0))
        .duration(0.3)
        .gradient(Color::MIDNIGHT_BLUE, Color::YELLOW_GREEN);
}
