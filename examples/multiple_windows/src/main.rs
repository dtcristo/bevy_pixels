use bevy::prelude::*;
use bevy_pixels::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PixelsPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Draw, draw)
        .run();
}

/// Spawn two more windows in addition to the primary window that comes by default.
fn setup(mut commands: Commands) {
    commands.spawn((Window::default(), PixelsOptions::default()));
    commands.spawn((Window::default(), PixelsOptions::default()));
}

/// Draw solid background to each window's buffer.
fn draw(mut wrapper_query: Query<&mut PixelsWrapper>) {
    for mut wrapper in &mut wrapper_query {
        let frame = wrapper.pixels.frame_mut();

        frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
    }
}
