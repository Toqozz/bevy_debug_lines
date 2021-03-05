# bevy_debug_lines
A prototype plugin providing a simple line drawing api for bevy.

See [docs.rs](https://docs.rs/bevy_prototype_debug_lines/) for documentation.

Expect breakage on master.

![demo](https://github.com/Toqozz/bevy_debug_lines/blob/master/demo.gif)
[![demo_2](https://github.com/Toqozz/bevy_debug_lines/blob/master/demo_2.png)](https://i.imgur.com/ryu3SEe.gifv)
Click on the above demo to play it.

## About
This plugin uses a shader and sends individual points to the GPU, which then moves geometry to make a line.  This is quite fast with a significant number of lines, and there is no added cost to moving lines around.

## Usage
Add `bevy_prototype_debug_lines` to your `Cargo.toml`:
```toml
[dependencies]
bevy_prototype_debug_lines = "0.1.3"
```

Add the plugin in your `App::build()` phase:
```rust
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        ...
        .run();
}
```

Draw a line in whatever system you have using the `DebugLines` resource:
```rust
fn some_system(
    ...
    mut lines: ResMut<DebugLines>
) {
    let start = Vec3::splat(-1.0);
    let end = Vec3::splat(1.0);
    let thickness = 0.01;
    lines.line(start, end, thickness);
}
```

See [the examples](https://github.com/Toqozz/bevy_debug_lines/tree/master/examples) for more complete usage examples.

## Running Examples
You can run the examples like so:
```shell
cargo run --example 3d --features="example_deps"
```

Where `3d` is one of the files in [the examples](https://github.com/Toqozz/bevy_debug_lines/tree/master/examples)

## Notes and Missing Stuff
This plugin is incomplete, so there are some things missing and general annoyances:
- [ ] More gracefully handle exceeding max lines (`128000`).
- [ ] More, I'm sure...

Let me know if you have any requests, improvements, or suggestions on how to make this crate more ergonomic.
