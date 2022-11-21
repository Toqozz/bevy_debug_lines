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
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });
}

fn demo(mut shapes: ResMut<DebugShapes>) {
    shapes
        .rect(Vec3::new(100.0, 0.0, 0.0), Vec2::new(100.0, 100.0))
        .angle(std::f32::consts::FRAC_PI_4)
        .color(Color::RED);
}
