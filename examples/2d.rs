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
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        });
}

fn demo(mut lines: ResMut<DebugLines>) {
    let start = Vec3::splat(-200.0);
    let end = Vec3::splat(200.0);
    lines.line(Vec3::new(-400.0, 200.0, 0.0), Vec3::new(400.0, 200.0, 0.0), 10.0);  // Units are generally "smaller" for 2d, so thickness should be higher.
    lines.line_colored(
        Vec3::new(-400.0, 0.0, 0.0), Vec3::new(400.0, 0.0, 0.0),
        10.0,
        Color::GREEN
    );
    lines.line_gradient(
        Vec3::new(-400.0, -200.0, 0.0), Vec3::new(400.0, -200.0, 0.0),
        10.0,
        Color::WHITE,
        Color::PINK
    );
}
