use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelsPlugin::default())
        .add_systems(Update, bevy::window::close_on_esc)
        // Add systems that draw to the buffer in `PixelsSet::Draw` set (or before)
        // to ensure they are rendered in the current frame.
        .add_systems(PostUpdate, draw.in_set(PixelsSet::Draw))
        .run();
}

/// Draw solid background to window buffer.
fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };
    let frame = wrapper.pixels.frame_mut();

    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}
