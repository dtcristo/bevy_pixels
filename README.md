# bevy_pixels

A [Bevy](https://github.com/bevyengine/bevy) plugin that connects the [pixels](https://github.com/parasyte/pixels) 2D pixel buffer renderer.

## Usage

Add `bevy_pixels` to `Cargo.toml`.

```
[dependencies]
bevy_pixels = { git = "https://github.com/dtcristo/bevy_pixels" }
```

Add `PixelsPlugin` to your Bevy project.

```rust
use bevy_pixels::bevy::prelude::*;
use bevy_pixels::{PixelsPlugin, PixelsResource};

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
    // Get a mutable slice into the pixel buffer
    let pixel_buffer: &mut [u8] = pixels_resource.pixels.get_frame();

    // Fill pixel buffer with pixel data
    // ...

    // Request a redraw of primary window
    windows.get_primary_mut().unwrap().request_redraw();
}
```

## Examples

### Hello Bevy Pixels

This example is based off [`minimal-winit`](https://github.com/parasyte/pixels/tree/master/examples/minimal-winit) example from the pixels project.

```sh
cargo run --release --example minimal
```
