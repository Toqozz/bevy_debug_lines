# bevy_debug_lines
A prototype plugin providing a simple line drawing api for bevy.

See [docs.rs](https://docs.rs/bevy_prototype_debug_lines/) for documentation.

Expect breakage on master.

![demo](https://github.com/Toqozz/bevy_debug_lines/blob/master/demo.gif)

## About
This plugin uses a shader and sends individual points to the GPU, which then moves geometry to make a line.  This is quite fast with a significant number of lines, and there is no added cost to moving lines around.

## Usage
Add `bevy_prototype_debug_lines` to your `Cargo.toml`:
```toml
[dependencies]
bevy_prototype_debug_lines = "0.1.2"
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
    let id = 0;
    let start = Vec3::splat(-1.0);
    let end = Vec3::splat(1.0);
    let thickness = 0.01;
    lines.line(id, start, end, thickness);
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
- [ ] Each separate line needs a separate ID... to separate it from the others.  This can probably be handled automatically with a hash or something in the future.
- [ ] Gracefully handle exceeding max lines (`128000`).
- [ ] Performance with over 50,000 lines.

Let me know if you have any requests, improvements, or suggestions on how to make this crate more ergonomic.
