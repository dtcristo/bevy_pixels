pub mod prelude {
    pub use crate::{PixelsOptions, PixelsPlugin, PixelsSet, PixelsWrapper};
}

pub use pixels;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    ecs::system::SystemState,
    prelude::*,
    window::{PrimaryWindow, RawHandleWrapper, WindowBackendScaleFactorChanged, WindowResized},
    winit::WinitWindows,
};
use pixels::{Pixels, SurfaceTexture};
#[cfg(target_arch = "wasm32")]
use pollster::FutureExt as _;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PixelsSet {
    /// Runs in [`CoreSet::Update`] base set. Use this set for simulation logic before draw.
    Update,
    /// Runs in [`CoreSet::PostUpdate`] base set. Use this set for logic that draws to the buffer.
    Draw,
    /// Runs in [`CoreSet::Last`] base set. This set is used internally for rendering the buffer to
    /// the surface.
    Render,
}

/// Defines the sizing and behavior of the pixel buffer and surface texture.
#[derive(Component, Debug, Copy, Clone)]
pub struct PixelsOptions {
    /// Width of the pixel buffer. Changing this after initialization will resize the buffer.
    pub width: u32,
    /// Height of the pixel buffer. Changing this after initialization will resize the buffer.
    pub height: u32,
    /// Scale factor between logical window size and buffer size. Only used when
    /// `auto_resize_buffer` is enabled.
    pub scale_factor: f32,
    /// Should the buffer automatically be resized when the window changes?
    pub auto_resize_buffer: bool,
    /// Should the surface texture automatically be resized when the window changes?
    pub auto_resize_surface: bool,
}

impl Default for PixelsOptions {
    fn default() -> Self {
        PixelsOptions {
            width: 640,
            height: 360,
            scale_factor: 2.0,
            auto_resize_buffer: true,
            auto_resize_surface: true,
        }
    }
}

/// Wrapper component for underlying [`Pixels`] struct.
#[derive(Component, Debug)]
pub struct PixelsWrapper {
    pub pixels: Pixels,
}

/// A [`Plugin`] that defines an integration between Bevy and the [`pixels`](https://github.com/parasyte/pixels)
/// crate. Should be added to app after [`DefaultPlugins`].
pub struct PixelsPlugin {
    /// Configuration for the primary window pixel buffer. This will automatically create a
    /// [`PixelsWrapper`] component (using the provided options) for the primary window entity.
    pub primary_window: Option<PixelsOptions>,
}

impl Default for PixelsPlugin {
    fn default() -> Self {
        PixelsPlugin {
            primary_window: Some(PixelsOptions::default()),
        }
    }
}

impl Plugin for PixelsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets((
            PixelsSet::Update.in_base_set(CoreSet::Update),
            PixelsSet::Draw.in_base_set(CoreSet::PostUpdate),
            PixelsSet::Render.in_base_set(CoreSet::Last),
        ))
        .add_startup_system(Self::setup)
        .add_system(Self::create_pixels.in_base_set(CoreSet::First))
        .add_systems(
            (
                Self::window_change,
                Self::window_resize,
                Self::resize_buffer.after(Self::window_resize),
            )
                .in_base_set(CoreSet::PreUpdate),
        )
        .add_system(Self::render.in_set(PixelsSet::Render));

        // If supplied, attach the primary window [`PixelsOptions`] component to the [`Window`]
        // entity with the [`PrimaryWindow`] marker component (if it exists). This will trigger
        // [`create_pixels`] system for this entity which will initialize the [`Pixels`] buffer.
        if let Some(options) = &self.primary_window {
            let mut system_state: SystemState<Query<Entity, With<PrimaryWindow>>> =
                SystemState::new(&mut app.world);
            let query = system_state.get(&app.world);

            if let Ok(entity) = query.get_single() {
                app.world.entity_mut(entity).insert(*options);
            };
        }
    }
}

impl PixelsPlugin {
    pub const RENDER_TIME: DiagnosticId =
        DiagnosticId::from_u128(1187582084072339577959028643519383692);

    /// Setup diagnostics.
    fn setup(mut diagnostics: ResMut<Diagnostics>) {
        diagnostics.add(Diagnostic::new(Self::RENDER_TIME, "render_time", 20).with_suffix("ms"));
    }

    /// Create [`PixelsWrapper`] (and underlying [`Pixels`] buffer) for all suitable [`Window`] with
    /// a [`PixelsOptions`] component.
    fn create_pixels(
        mut commands: Commands,
        query: Query<(Entity, &PixelsOptions), (With<RawHandleWrapper>, Without<PixelsWrapper>)>,
        winit_windows: NonSend<WinitWindows>,
    ) {
        for (entity, options) in &query {
            let winit_window = winit_windows
                .get_window(entity)
                .expect("failed to get winit window");

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
                    // TODO: Find a way to asynchronously load pixels on web.
                    Pixels::new_async(options.width, options.height, surface_texture).block_on()
                }
            }
            .expect("failed to create pixels");

            commands.entity(entity).insert(PixelsWrapper { pixels });
        }
    }

    /// Resize buffer and surface to window when it is resized.
    fn window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        mut query: Query<(&mut PixelsWrapper, &mut PixelsOptions, &Window)>,
    ) {
        for event in window_resized_events.iter() {
            if let Ok((mut wrapper, mut options, window)) = query.get_mut(event.window) {
                if options.auto_resize_buffer {
                    options.width = (window.width() / options.scale_factor).floor() as u32;
                    options.height = (window.height() / options.scale_factor).floor() as u32;
                }

                if options.auto_resize_surface {
                    Self::resize_surface_to_window(&mut wrapper, window);
                }
            }
        }
    }

    /// Resize surface to window when scale factor changes.
    fn window_change(
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut query: Query<(&mut PixelsWrapper, &PixelsOptions, &Window)>,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            if let Ok((mut wrapper, options, window)) = query.get_mut(event.window) {
                if options.auto_resize_surface {
                    Self::resize_surface_to_window(&mut wrapper, window);
                }
            }
        }
    }

    fn resize_surface_to_window(wrapper: &mut PixelsWrapper, window: &Window) {
        let _ = wrapper
            .pixels
            .resize_surface(window.physical_width(), window.physical_height());
    }

    /// Resize buffer when width and height change.
    fn resize_buffer(
        mut query: Query<(&mut PixelsWrapper, &PixelsOptions), Changed<PixelsOptions>>,
    ) {
        for (mut wrapper, options) in &mut query {
            if options.auto_resize_buffer {
                let _ = wrapper.pixels.resize_buffer(options.width, options.height);
            }
        }
    }

    /// Render buffer to surface.
    #[cfg(not(target_arch = "wasm32"))]
    fn render(mut diagnostics: ResMut<Diagnostics>, query: Query<&PixelsWrapper>) {
        let start = Instant::now();

        for wrapper in &query {
            wrapper.pixels.render().expect("failed to render pixels");
        }

        let end = Instant::now();
        let render_time_seconds = end.duration_since(start).as_secs_f64();
        diagnostics.add_measurement(Self::RENDER_TIME, || render_time_seconds * 1000.0);
    }

    #[cfg(target_arch = "wasm32")]
    fn render(query: Query<&PixelsWrapper>) {
        for wrapper in &query {
            wrapper.pixels.render().expect("failed to render pixels");
        }
    }
}
