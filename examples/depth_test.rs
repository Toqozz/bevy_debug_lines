use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugLinesPlugin::with_depth_test(true))
        .add_systems(Startup, setup)
        .add_systems(Update, demo)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::new(0.5, 0.5, 0.5),
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, -0.5),
        ..Default::default()
    });
}

fn demo(mut lines: ResMut<DebugLines>) {
    lines.line_gradient(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
        0.0,
        Color::BLUE,
        Color::RED,
    );
}
