use bevy::{input::mouse::MouseMotion, prelude::*};

use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup.system())
        .add_system_to_stage(CoreStage::Last, move_with_mouse.before("draw_lines"))
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

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, -0.5),
            ..Default::default()
        })
        .insert(MoveWithMouse);
}

#[derive(Component)]
struct MoveWithMouse;
fn move_with_mouse(
    mut mouse_motion: EventReader<MouseMotion>,
    mut lines: ResMut<DebugLines>,
    mut query: Query<&mut Transform, With<MoveWithMouse>>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion.iter() {
        delta += event.delta;
    }

    for mut transform in query.iter_mut() {
        let movement = Vec3::new(delta.x, -delta.y, 0.0) * 0.01;
        transform.translation += movement;
        let forward = transform.local_z();
        lines.line_colored(
            transform.translation,
            transform.translation + forward,
            0.0,
            Color::GREEN,
        );
    }
}
