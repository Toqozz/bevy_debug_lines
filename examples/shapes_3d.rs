use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugShapes};

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
}

fn demo(time: Res<Time>, mut shapes: ResMut<DebugShapes>) {
    use std::f32::consts::FRAC_PI_4;

    let seconds = time.elapsed_seconds();

    shapes
        .cuboid()
        .position(Vec3::new(2.0, 0.0, 0.0))
        .size(Vec3::ONE)
        .rotation(Quat::from_rotation_x(seconds * FRAC_PI_4))
        .color(Color::RED);

    shapes
        .cuboid()
        .min_max(Vec3::NEG_ONE, Vec3::ONE)
        .rotation(Quat::from_rotation_y(seconds * FRAC_PI_4))
        .color(Color::PURPLE);

    shapes
        .sphere()
        .position(Vec3::new(-2.0, 0.0, 0.0))
        .radius(0.5)
        .rotation(Quat::from_euler(
            EulerRot::XYZ,
            seconds * FRAC_PI_4,
            seconds * FRAC_PI_4,
            0.0,
        ))
        .color(Color::RED);
}
