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

fn setup(mut commands: Commands, mut lines: ResMut<DebugLines>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
    // A line that stays on screen 9 seconds
    lines
        .line(Vec3::new(1.0, -1.0, -1.0), Vec3::new(-1.0, 1.0, 1.0))
        .duration(9.0)
        .gradient(Color::CYAN, Color::MIDNIGHT_BLUE);
}

fn demo(time: Res<Time>, mut lines: ResMut<DebugLines>) {
    let seconds = time.elapsed_seconds();

    lines.line(
        Vec3::new(-1.0, f32::sin(seconds), 0.0),
        Vec3::new(1.0, f32::sin(seconds + 3.14), 0.0),
    );
    lines
        .line(
            Vec3::new(f32::sin(seconds), -1.0, 0.0),
            Vec3::new(f32::sin(seconds + 3.14), 1.0, 0.0),
        )
        .color(Color::WHITE);
    lines
        .line(
            Vec3::new(-1.0, -1.0, f32::sin(seconds)),
            Vec3::new(1.0, 1.0, f32::sin(seconds + 3.14)),
        )
        .gradient(Color::GOLD, Color::PINK);
}
