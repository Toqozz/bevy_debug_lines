use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugShapes};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugLinesPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, demo)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });
}

fn demo(time: Res<Time>, mut shapes: ResMut<DebugShapes>) {
    use std::f32::consts::FRAC_PI_4;

    let seconds = time.elapsed_seconds();

    shapes
        .rect()
        .position(Vec3::new(200.0, 0.0, 0.0))
        .size(Vec2::new(100.0, 100.0))
        .angle(seconds * FRAC_PI_4)
        .color(Color::RED);

    shapes
        .rect()
        .min_max(Vec2::new(-100.0, -100.0), Vec2::new(100.0, 100.0))
        .angle(seconds * -FRAC_PI_4)
        .color(Color::PURPLE);

    shapes
        .circle()
        .position(Vec3::new(-200.0, 0.0, 0.0))
        .radius(50.0)
        .color(Color::RED);
}
