use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugShapes};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                present_mode: PresentMode::Immediate,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin {
            wait_duration: bevy::utils::Duration::new(5, 0),
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(demo_circle)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
}

fn demo_circle(time: Res<Time>, mut shapes: ResMut<DebugShapes>) {
    use bevy_prototype_debug_lines::MAX_LINES;
    use std::f32::consts::PI;

    const RADIUS: f32 = 1.5;
    const DURATION: f32 = 0.0;

    let seconds = 0.5 * time.elapsed_seconds();

    for i in 0..MAX_LINES {
        let angle = (i as f32 / MAX_LINES as f32) * 180.0;
        let (x, y, z) = (
            RADIUS * f32::cos(angle * PI / 180.0 * seconds),
            RADIUS * f32::sin(angle * PI / 180.0 * seconds),
            seconds.sin(),
        );

        /* Super trippy.
        let (initial_x, initial_y) = (
            RADIUS * f32::cos(angle * PI / 180.0 * i as f32),
            RADIUS * f32::sin(angle * PI / 180.0 * i as f32),
        );

        let start_color = Color::rgba(initial_x, initial_y, 0.5, start.z.max(0.5));
        let end_color = Color::rgba(-initial_x, -initial_y, 0.5, end.z.max(0.5));
        */

        let start = Vec3::new(x, y, z);
        let end = -start;

        let start_color = Color::rgba(start.x, start.y, 0.5, start.z.max(0.5));
        let end_color = Color::rgba(end.x, end.y, 0.5, end.z.max(0.5));

        shapes
            .line()
            .start_end(start, end)
            .duration(DURATION)
            .gradient(start_color, end_color);
    }
}
