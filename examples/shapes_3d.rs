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
}

fn demo(time: Res<Time>, mut lines: ResMut<DebugLines>) {
    let seconds = time.elapsed_seconds();

    lines
        .cuboid(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
        .rotation(Quat::from_axis_angle(
            Vec3::X,
            seconds * std::f32::consts::FRAC_PI_4,
        ))
        .color(Color::RED);
}