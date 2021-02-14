# bevy_pixels

[Bevy](https://github.com/bevyengine/bevy) plugin that uses [Pixels](https://github.com/parasyte/pixels) (a tiny pixel buffer) for rendering.

## Usage

Add `bevy_pixels` to `Cargo.toml`. If depending on `bevy` directly, be sure to disable `render` and `bevy_wgpu` features (with `default-features = false`) as they will conflict with rendering provided by `bevy_pixels`.

```toml
[dependencies]
bevy_pixels = { git = "https://github.com/dtcristo/bevy_pixels" }
```

Add `PixelsPlugin` to your Bevy project.

```rust
use bevy_pixels::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_system(main_system.system())
        .run();
}
```

Use `PixelsResource` in your systems.

```rust
fn main_system(mut pixels_resource: ResMut<PixelsResource>, mut windows: ResMut<Windows>) {
    // Get a mutable slice for the pixel buffer
    let pixel_buffer: &mut [u8] = pixels_resource.pixels.get_frame();

    // Fill pixel buffer with pixel data
    // ...
}
```

## Examples

### [Hello Bevy Pixels](https://github.com/dtcristo/bevy_pixels/blob/main/examples/minimal.rs)

This example is based off [`minimal-winit`](https://github.com/parasyte/pixels/tree/master/examples/minimal-winit) example from the pixels project.

```sh
cargo run --release --example minimal
```

## TODO

- Add more configuration around how rendering is performed.
- Add support for multiple windows.
- Improve minimal example.
- Publish to crates.io.
