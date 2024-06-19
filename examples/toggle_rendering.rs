use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugLinesPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, demo)
        .run();
}

fn setup(mut commands: Commands, mut lines: ResMut<DebugLines>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
    // A line that stays on screen 9 seconds
    lines.line_gradient(
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        9.0,
        Color::CYAN,
        Color::MIDNIGHT_BLUE,
    );
}

fn demo(time: Res<Time>, mut lines: ResMut<DebugLines>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Toggled Rendering.");
        lines.enabled = !lines.enabled;
    }

    let seconds = time.elapsed_seconds();

    lines.line(
        Vec3::new(-1.0, f32::sin(seconds), 0.0),
        Vec3::new(1.0, f32::sin(seconds + 3.14), 0.0),
        0.0,
    );
    lines.line_colored(
        Vec3::new(f32::sin(seconds), -1.0, 0.0),
        Vec3::new(f32::sin(seconds + 3.14), 1.0, 0.0),
        0.0,
        Color::WHITE,
    );
    lines.line_gradient(
        Vec3::new(-1.0, -1.0, f32::sin(seconds)),
        Vec3::new(1.0, 1.0, f32::sin(seconds + 3.14)),
        0.0,
        Color::GOLD,
        Color::PINK,
    );
}
