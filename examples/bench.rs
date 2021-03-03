use bevy::prelude::*;
use bevy::diagnostic::PrintDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin;

use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines };

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WgpuResourceDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(demo_circle.system())
        //.add_system(demo2.system())
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

fn demo_circle(time: Res<Time>, mut lines: ResMut<DebugLines>) {
    use bevy_prototype_debug_lines::MAX_LINES;
    use std::f32::consts::PI;

    const RADIUS: f32 = 1.5;
    const THICKNESS: f32 = 0.01;

    let seconds = 0.5 * time.seconds_since_startup() as f32;

    for i in 0..MAX_LINES {
        let angle = (i as f32 / MAX_LINES as f32) * 180.0;
        let (x, y, z) = (
            RADIUS * f32::cos(angle * PI / 180.0 * seconds),
            RADIUS * f32::sin(angle * PI / 180.0 * seconds),
            seconds.sin(),
        );

        let start = Vec3::new(x, y, z);
        let end = -start;

        let start_color = Color::rgba(x, y, 0.5, start.z.max(0.0));
        let end_color = Color::rgba(x, y, 0.5, end.z.max(0.0));

        lines.line_gradient(i as u32, start, end, THICKNESS, start_color, end_color);
    }
}

fn demo_block(time: Res<Time>, mut lines: ResMut<DebugLines>) {
    use bevy_prototype_debug_lines::MAX_LINES;

    const THICKNESS: f32 = 0.01;
    const X: f32 = 2.0;
    const Y: f32 = 1.0;

    for i in 0..MAX_LINES {
        let percent = i as f32 / MAX_LINES as f32;

        let start = Vec3::lerp(Vec3::new(-X, Y, 0.0), Vec3::new(-X, -Y, 0.0), percent);
        let end = Vec3::lerp(Vec3::new(X, Y, 0.0), Vec3::new(X, -Y, 0.0), percent);

        let start_color = Color::rgba(start.x, start.y, 0.5, 1.0);
        let end_color = Color::rgba(end.x, end.y, 0.5, 1.0);

        lines.line_gradient(i as u32, start, end, THICKNESS, start_color, end_color);
    }
}
