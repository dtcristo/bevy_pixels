use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResizeConstraints,
};
use bevy_pixels::prelude::*;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum AppStage {
    AppStartup,
}

#[derive(Component, Debug)]
struct Color(u8, u8, u8, u8);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Pixels Draw".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            resize_constraints: WindowResizeConstraints {
                min_width: WIDTH as f32,
                min_height: HEIGHT as f32,
                ..default()
            },
            ..default()
        })
        .insert_resource(PixelsOptions {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(exit_on_escape)
        .add_system_to_stage(PixelsStage::Draw, draw_mouse)
        .run();
}

fn setup(mut pixels_resource: ResMut<PixelsResource>) {
    let frame = pixels_resource.pixels.get_frame_mut();
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

fn exit_on_escape(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn draw_mouse(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut pixels_resource: ResMut<PixelsResource>,
) {
    for event in cursor_moved_events.iter() {
        if event.id == pixels_resource.window_id {
            let frame = pixels_resource.pixels.get_frame_mut();
            let frame_width_bytes = (WIDTH * 4) as usize;
            let x_offset = event.position.x as usize * 4;
            let y_offset = event.position.y as usize * frame_width_bytes;
            let i = x_offset + y_offset;
            frame[i..(i + 4)].copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }
    }
}
