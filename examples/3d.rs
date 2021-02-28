use bevy::prelude::*;

use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines };

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup.system())
        .add_system(demo.system())
        .run();
}

fn setup(
    commands: &mut Commands,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        });
}

fn demo(time: Res<Time>, mut lines: ResMut<DebugLines>) {
    let seconds = time.seconds_since_startup() as f32;

    lines.line_colored(0, Vec3::new(-1.0, f32::sin(seconds), 0.0), Vec3::new(1.0, f32::sin(seconds + 3.14), 0.0),  0.01, Color::WHITE);
    lines.line_colored(1, Vec3::new(f32::sin(seconds), -1.0, 0.0), Vec3::new(f32::sin(seconds + 3.14), 1.0, 0.0),  0.01, Color::GREEN);
    lines.line_colored(2, Vec3::new(-1.0, -1.0, f32::sin(seconds)), Vec3::new(1.0, 1.0, f32::sin(seconds + 3.14)), 0.01, Color::WHITE);
}
