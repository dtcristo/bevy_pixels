use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PixelsPlugin::default()))
        .add_systems(Draw, draw)
        // Custom render system. Default `render` cargo feature must be disabled before
        // defining a custom render system. Use `default-features = "false"` in Cargo.toml.
        .add_systems(Render, render)
        .run();
}

/// Draw solid background to window buffer.
fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else {
        return;
    };
    let frame = wrapper.pixels.frame_mut();

    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

/// Custom render system.
pub fn render(wrapper_query: Query<&PixelsWrapper>) {
    let Ok(wrapper) = wrapper_query.get_single() else {
        return;
    };

    // Custom render logic here. Should support usage of shaders.
    wrapper
        .pixels
        .render_with(|encoder, render_target, context| {
            context.scaling_renderer.render(encoder, render_target);
            // etc...
            Ok(())
        })
        .expect("failed to render pixels");
}
