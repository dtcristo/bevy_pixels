pub mod prelude {
    pub use crate::{PixelsPlugin, PixelsResource, PixelsSet};
}

pub use pixels;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::*,
    window::{WindowBackendScaleFactorChanged, WindowResized},
    winit::WinitWindows,
};
use pixels::{Pixels, SurfaceTexture};
#[cfg(target_arch = "wasm32")]
use pollster::FutureExt as _;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PixelsSet {
    Draw,
    Render,
    PostRender,
}

#[derive(Resource)]
pub struct PixelsResource {
    pub pixels: Pixels,
    pub window: Entity,
}

#[derive(Resource)]
struct PixelsOptions {
    width: u32,
    height: u32,
}

pub struct PixelsPlugin {
    /// Width of the pixel buffer
    pub width: u32,
    /// Height of the pixel buffer
    pub height: u32,
}

impl Default for PixelsPlugin {
    fn default() -> Self {
        PixelsPlugin {
            width: 180,
            height: 120,
        }
    }
}

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PixelsOptions {
            width: self.width,
            height: self.height,
        })
        .configure_set(
            PixelsSet::Draw
                .before(PixelsSet::Render)
                .before(PixelsSet::PostRender), // (PixelsSet::Draw, PixelsSet::Render, PixelsSet::PostRender).chain()
        )
        .add_startup_system(Self::setup)
        .add_system(Self::window_resize)
        .add_system(Self::window_change)
        .add_system(Self::render.in_set(PixelsSet::Render));
    }
}

impl PixelsPlugin {
    pub const RENDER_TIME: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339577959028643519383692);

    fn setup(
        mut commands: Commands,
        mut diagnostics: ResMut<Diagnostics>,
        options: Res<PixelsOptions>,
        windows: Query<(Entity, &Window)>,
        winit_windows: NonSend<WinitWindows>,
    ) {
        diagnostics.add(Diagnostic::new(Self::RENDER_TIME, "render_time", 20).with_suffix("s"));

        let (window, _) = windows.get_single().expect("primary window not found");

        let winit_window = winit_windows
            .get_window(window)
            .expect("failed to get primary winit window");

        let window_size = winit_window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, winit_window);

        let pixels = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                Pixels::new(options.width, options.height, surface_texture)
            }
            #[cfg(target_arch = "wasm32")]
            {
                // TODO: Find a way to asynchronously load pixels on web
                Pixels::new_async(options.width, options.height, surface_texture).block_on()
            }
        }
        .expect("failed to create pixels");

        commands.insert_resource(PixelsResource { pixels, window });
    }

    fn window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        mut resource: ResMut<PixelsResource>,
        windows: Query<&Window>,
    ) {
        for event in window_resized_events.iter() {
            if event.window == resource.window {
                if let Ok(window) = windows.get(event.window) {
                    Self::resize_surface_to_window(&mut resource, window);
                }
            }
        }
    }

    fn window_change(
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut resource: ResMut<PixelsResource>,
        windows: Query<&Window>,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            if event.window == resource.window {
                if let Ok(window) = windows.get(event.window) {
                    Self::resize_surface_to_window(&mut resource, window);
                }
            }
        }
    }

    fn resize_surface_to_window(resource: &mut ResMut<PixelsResource>, window: &Window) {
        let _ = resource
            .pixels
            .resize_surface(window.physical_width(), window.physical_height());
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render(resource: Res<PixelsResource>, mut diagnostics: ResMut<Diagnostics>) {
        let start = Instant::now();

        resource.pixels.render().expect("failed to render pixels");

        let end = Instant::now();
        let render_time = end.duration_since(start);
        diagnostics.add_measurement(Self::RENDER_TIME, || render_time.as_secs_f64());
    }

    #[cfg(target_arch = "wasm32")]
    fn render(resource: Res<PixelsResource>) {
        resource.pixels.render().expect("failed to render pixels");
    }
}
