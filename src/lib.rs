pub use bevy;
pub use pixels;

use bevy::prelude::*;
use bevy::window::{WindowBackendScaleFactorChanged, WindowId, WindowResized};
use bevy::winit::WinitWindows;
use pixels::{Pixels, SurfaceTexture};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PixelsStage {
    Draw,
    Render,
    PostRender,
}

pub mod prelude {
    pub use crate::{PixelsOptions, PixelsPlugin, PixelsResource, PixelsStage};
    pub use bevy;
    pub use bevy::prelude::*;
    pub use pixels;
}

#[derive(Debug, Clone)]
pub struct PixelsOptions {
    /// Width of the pixel buffer
    pub width: u32,
    /// Height of the pixel buffer
    pub height: u32,
}

impl Default for PixelsOptions {
    fn default() -> Self {
        PixelsOptions {
            width: 1280,
            height: 720,
        }
    }
}

pub struct PixelsResource {
    pub pixels: Pixels,
    pub window_id: WindowId,
}

pub struct PixelsPlugin;

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(
            CoreStage::PostUpdate,
            PixelsStage::Draw,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PixelsStage::Draw,
            PixelsStage::Render,
            SystemStage::parallel(),
        )
        .add_stage_after(
            PixelsStage::Render,
            PixelsStage::PostRender,
            SystemStage::parallel(),
        )
        .init_resource::<PixelsOptions>()
        .add_startup_system(Self::setup_system.system())
        .add_system(Self::window_resize_system.system())
        .add_system(Self::window_change_system.system())
        .add_system_to_stage(PixelsStage::Render, Self::render_system.system());
    }
}

impl PixelsPlugin {
    pub fn setup_system(
        mut commands: Commands,
        options: Res<PixelsOptions>,
        windows: Res<Windows>,
        winit_windows: Res<WinitWindows>,
    ) {
        let primary_window_id = windows
            .get_primary()
            .expect("primary window not found")
            .id();

        let winit_window = winit_windows
            .get_window(primary_window_id)
            .expect("failed to get primary winit window");

        let window_size = winit_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, winit_window);
        let pixels = Pixels::new(options.width, options.height, surface_texture)
            .expect("failed to create pixels");

        commands.insert_resource(PixelsResource {
            pixels: pixels,
            window_id: primary_window_id,
        });
    }

    pub fn window_resize_system(
        mut window_resized_events: EventReader<WindowResized>,
        mut resource: ResMut<PixelsResource>,
    ) {
        for event in window_resized_events.iter() {
            if event.id == resource.window_id {
                resource
                    .pixels
                    .resize(event.width as u32, event.height as u32);
            }
        }
    }

    pub fn window_change_system(
        windows: Res<Windows>,
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut resource: ResMut<PixelsResource>,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            if event.id == resource.window_id {
                let window = windows.get(resource.window_id).unwrap();

                resource
                    .pixels
                    .resize(window.physical_width(), window.physical_height());
            }
        }
    }

    pub fn render_system(mut resource: ResMut<PixelsResource>) {
        resource.pixels.render().expect("failed to render pixels");
    }
}
