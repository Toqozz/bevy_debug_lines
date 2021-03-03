# bevy_debug_lines
A prototype plugin providing a simple line drawing api for bevy.

Expect breakage on master.

![demo](https://github.com/Toqozz/bevy_debug_lines/blob/master/demo.gif)

## About

This plugin uses a shader and sends individual points to the GPU, which then moves geometry to make a line.  This is quite fast with a significant number of lines, and there is no added cost to moving lines around.

## Usage
Add `bevy_prototype_debug_lines` to your `Cargo.toml`:
```toml
[dependencies]
bevy_prototype_debug_lines = "0.1.1-alpha"
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
    lines.add_or_update_line(id, start, end);
}
```

See [the examples](https://github.com/Toqozz/bevy_debug_lines/tree/master/examples) for a more complete usage example.

## Notes and Missing Stuff
This plugin is in alpha, so there are things missing and general annoyances:
- [ ] Each separate line needs a separate ID... to separate it from the others.  This can probably be handled automatically with a hash or something.
- [x] There is a defined maximum number of lines that is kinda low (`MAX_LINES = 255`).  I have some ideas to resolve this. -> 128k limit on master now.
- [x] Thickness is hardcoded (see `line.vert`), this is also easy to resolve -> fixed on master.
- [x] Color is hardcoded (see `line.frag`) -> fixed on master.
- [x] Missing documentation, but there's only really one function you should use; `add_or_update_line(id, start, end)`.
