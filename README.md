<div align="center">
  <h1>
    bevy_pixels
  </h1>
  <p>
    <strong>
      <a href="https://github.com/bevyengine/bevy">Bevy</a> plugin that uses
      <a href="https://github.com/parasyte/pixels">Pixels</a> (a tiny pixel buffer) for rendering
    </strong>
  </p>
  <p>
    <a href="https://crates.io/crates/bevy_pixels">
      <img src="https://img.shields.io/crates/v/bevy_pixels.svg" alt="crates.io" />
    </a>
    <a
      href="https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking"
    >
      <img
        src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue"
        alt="Bevy tracking"
      />
    </a>
  </p>
</div>

## Usage

Add `bevy` and `bevy_pixels` to `Cargo.toml`. Be sure to disable `bevy`'s `render` and `bevy_wgpu` features (with `default-features = false`) as they will conflict with rendering provided by `bevy_pixels`.

```toml
[dependencies]
bevy = { version = "0.10", default_features = false }
bevy_pixels = "0.9"
```

Add `PixelsPlugin` to your Bevy project.

```rust
use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin::default())
        // Add systems that draw to the buffer in `PixelsSet::Draw` set (or before)
        // to ensure they are rendered in the current frame.
        .add_system(draw.in_set(PixelsSet::Draw))
        .run();
}
```

Use `PixelsWrapper` in your systems.

```rust
fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };

    // Get a mutable slice for the pixel buffer.
    let frame: &mut [u8] = wrapper.pixels.frame_mut();

    // Fill frame with pixel data.
    // ...
}
```

## Bevy and Pixels version mapping

| bevy_pixels | bevy  | pixels |
| ----------- | ----- | ------ |
| 0.1         | 0.5   | 0.3    |
| 0.2         | 0.5   | 0.8    |
| 0.3-0.4     | 0.6   | 0.9    |
| 0.5         | 0.7   | 0.9    |
| 0.6         | 0.8   | 0.10   |
| 0.7         | 0.9   | 0.10   |
| 0.8         | 0.9   | 0.11   |
| 0.9         | 0.10  | 0.12   |

## Examples

### [minimal](https://github.com/dtcristo/bevy_pixels/blob/main/examples/minimal/src/main.rs)

This example demonstrates rendering a solid color to the pixel buffer.

### [multiple_windows](https://github.com/dtcristo/bevy_pixels/blob/main/examples/multiple_windows/src/main.rs)

This example demonstrate usage of multiple windows each with their own pixel buffer.

### [custom_render](https://github.com/dtcristo/bevy_pixels/blob/main/examples/custom_render/src/main.rs)

This example demonstrate usage of a custom render system. Default `render` cargo feature must be disabled before defining a custom render system. Use `default_features = "false"` in Cargo.toml.

### [bounce](https://github.com/dtcristo/bevy_pixels/blob/main/examples/bounce/src/main.rs)

This example is based off [`minimal-winit`](https://github.com/parasyte/pixels/tree/master/examples/minimal-winit) example from the pixels project. It demonstrates rendering dynamic content to the pixel buffer.

![bounce example](images/bounce.png)

### Running examples natively

Build and run example with [just](https://github.com/casey/just). See [`Justfile`](Justfile) for more details. Install `just` with `cargo install just`.

```sh
just run example_name
```

### Running examples in web browser

Install dependencies.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli miniserve
```

Build and serve example for web.

```sh
just serve-web example_name
```

Open [localhost:8080](http://localhost:8080/) in your web browser to run the example.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
