use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_pixels::{PixelsOptions, PixelsPlugin, PixelsResource};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Hello Bevy Pixels".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            ..Default::default()
        })
        .insert_resource(PixelsOptions {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(PixelsPlugin)
        .insert_resource(World::new())
        .add_system(main_system.system())
        .add_system(keyboard_input_system.system())
        .run();
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn main_system(mut world: ResMut<World>, mut pixels_resource: ResMut<PixelsResource>) {
    world.update();

    let pixels = &mut pixels_resource.pixels;
    world.draw(pixels.get_frame());
    pixels.render().expect("failed to render pixels");
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
