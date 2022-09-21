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
bevy = { version = "0.8", default_features = false }
bevy_pixels = { git = "https://github.com/dtcristo/bevy_pixels" }
```

Add `PixelsPlugin` to your Bevy project.

```rust
use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_system(main_system)
        .run();
}
```

Use `PixelsResource` in your systems.

```rust
fn main_system(mut pixels_resource: ResMut<PixelsResource>) {
    // Get a mutable slice for the pixel buffer
    let frame: &mut [u8] = pixels_resource.pixels.get_frame_mut();

    // Fill frame with pixel data
    // ...
}
```

## Bevy and Pixels version mapping

| bevy_pixels | bevy | pixels |
| ----------- | ---- | ------ |
| 0.1         | 0.5  | 0.3    |
| 0.2         | 0.5  | 0.8    |
| 0.3-0.4     | 0.6  | 0.9    |
| 0.5         | 0.7  | 0.9    |

## Examples

### [minimal](https://github.com/dtcristo/bevy_pixels/blob/main/examples/minimal.rs)

This example is based off [`minimal-winit`](https://github.com/parasyte/pixels/tree/master/examples/minimal-winit) example from the pixels project.

![minimal example](images/minimal.png)

### Running examples natively

```sh
cargo run --release --example example_name
```

### Running examples in browser

Install dependencies.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli just miniserve
```

Build and serve example with [just](https://github.com/casey/just). See [`Justfile`](Justfile) for more details.

```sh
just serve example_name
```

Open http://localhost:8080/ in your browser to run the example.

## License

Licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

## Todo

- Add more configuration around how rendering is performed.
- Add support for multiple windows.
