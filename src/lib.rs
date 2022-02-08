pub mod prelude {
    pub use crate::{PixelsOptions, PixelsPlugin, PixelsResource, PixelsStage};
}

pub use pixels;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::*,
    window::{WindowBackendScaleFactorChanged, WindowId, WindowResized},
    winit::WinitWindows,
};
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PixelsStage {
    Draw,
    Render,
    PostRender,
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
            width: 180,
            height: 120,
        }
    }
}

pub struct PixelsResource {
    pub pixels: Pixels,
    pub window_id: WindowId,
}

pub struct PixelsPlugin;

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut App) {
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
        .add_startup_system(Self::setup_system)
        .add_system(Self::window_resize_system)
        .add_system(Self::window_change_system)
        .add_system_to_stage(PixelsStage::Render, Self::render_system);
    }
}

impl PixelsPlugin {
    pub const RENDER_TIME: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339577959028643519383692);

    pub fn setup_system(
        mut commands: Commands,
        mut diagnostics: ResMut<Diagnostics>,
        options: Res<PixelsOptions>,
        windows: Res<Windows>,
        winit_windows: Res<WinitWindows>,
    ) {
        diagnostics.add(Diagnostic::new(Self::RENDER_TIME, "render_time", 20).with_suffix("s"));

        let window_id = windows
            .get_primary()
            .expect("primary window not found")
            .id();

        let winit_window = winit_windows
            .get_window(window_id)
            .expect("failed to get primary winit window");

        let window_size = winit_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, winit_window);
        let pixels = Pixels::new(options.width, options.height, surface_texture)
            .expect("failed to create pixels");

        commands.insert_resource(PixelsResource { pixels, window_id });
    }

    pub fn window_resize_system(
        mut window_resized_events: EventReader<WindowResized>,
        mut resource: ResMut<PixelsResource>,
    ) {
        for event in window_resized_events.iter() {
            if event.id == resource.window_id {
                resource
                    .pixels
                    .resize_surface(event.width as u32, event.height as u32);
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
                    .resize_surface(window.physical_width(), window.physical_height());
            }
        }
    }

    pub fn render_system(resource: Res<PixelsResource>, mut diagnostics: ResMut<Diagnostics>) {
        let start = Instant::now();

        resource.pixels.render().expect("failed to render pixels");

        let end = Instant::now();
        let render_time = end.duration_since(start);
        diagnostics.add_measurement(Self::RENDER_TIME, render_time.as_secs_f64());
    }
}
