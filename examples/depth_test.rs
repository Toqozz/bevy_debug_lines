use bevy::prelude::*;

use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines };

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .insert_resource(DebugLines { depth_test: true, ..Default::default() })
        .add_startup_system(setup.system())
        .add_system(demo.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial { base_color: Color::RED, ..Default::default() }),
        transform: Transform::from_xyz(0.0, 0.0, -0.5),
        ..Default::default()
    });
}

fn demo(mut lines: ResMut<DebugLines>) {
    lines.line_gradient(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0),  0.0, Color::BLUE, Color::RED);
}
