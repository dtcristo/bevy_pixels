#[cfg(feature = "render")]
#[cfg(not(target_arch = "wasm32"))]
use crate::diagnostic;
use crate::prelude::*;

#[cfg(feature = "render")]
#[cfg(not(target_arch = "wasm32"))]
use bevy::diagnostic::Diagnostics;
use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    window::{PresentMode, RawHandleWrapper, WindowBackendScaleFactorChanged, WindowResized},
};
use pixels::{PixelsBuilder, SurfaceTexture};
#[cfg(target_arch = "wasm32")]
use pollster::FutureExt as _;
#[cfg(feature = "render")]
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

/// Create [`PixelsWrapper`] (and underlying [`Pixels`] buffer) for all suitable [`Window`] with
/// a [`PixelsOptions`] component.
#[allow(clippy::type_complexity)]
pub fn create_pixels(
    mut commands: Commands,
    query: Query<
        (Entity, &PixelsOptions, &Window, &RawHandleWrapper),
        (With<RawHandleWrapper>, Without<PixelsWrapper>),
    >,
    _main_thread: NonSendMarker,
) {
    for (entity, options, window, raw_handle_wrapper) in &query {
        // SAFETY: `NonSendMarker` forces this system onto Bevy's main thread, which is required by
        // `RawHandleWrapper::get_handle` on platforms whose window handles are thread-affine.
        let thread_locked_handle = unsafe { raw_handle_wrapper.get_handle() };

        let surface_texture = SurfaceTexture::new(
            window.physical_width(),
            window.physical_height(),
            thread_locked_handle,
        );

        let pixels = {
            let builder = PixelsBuilder::new(options.width, options.height, surface_texture)
                .present_mode(match window.present_mode {
                    PresentMode::Fifo => pixels::wgpu::PresentMode::Fifo,
                    PresentMode::FifoRelaxed => pixels::wgpu::PresentMode::FifoRelaxed,
                    PresentMode::Mailbox => pixels::wgpu::PresentMode::Mailbox,
                    PresentMode::Immediate => pixels::wgpu::PresentMode::Immediate,
                    PresentMode::AutoVsync => pixels::wgpu::PresentMode::AutoVsync,
                    PresentMode::AutoNoVsync => pixels::wgpu::PresentMode::AutoNoVsync,
                });

            #[cfg(not(target_arch = "wasm32"))]
            {
                builder.build()
            }
            #[cfg(target_arch = "wasm32")]
            {
                // TODO: Find a way to asynchronously load pixels on web.
                builder.build_async().block_on()
            }
        }
        .expect("failed to create pixels");

        commands.entity(entity).insert(PixelsWrapper { pixels });
    }
}

/// Resize buffer and surface to window when it is resized.
pub fn window_resize(
    mut window_resized_events: MessageReader<WindowResized>,
    mut query: Query<(&mut PixelsWrapper, &mut PixelsOptions, &Window)>,
) {
    for event in window_resized_events.read() {
        if let Ok((mut wrapper, mut options, window)) = query.get_mut(event.window) {
            if options.auto_resize_buffer {
                options.width = (window.width() / options.scale_factor).floor() as u32;
                options.height = (window.height() / options.scale_factor).floor() as u32;
            }

            if options.auto_resize_surface {
                resize_surface_to_window(&mut wrapper, window);
            }
        }
    }
}

/// Resize surface to window when scale factor changes.
pub fn window_change(
    mut window_backend_scale_factor_changed_events: MessageReader<WindowBackendScaleFactorChanged>,
    mut query: Query<(&mut PixelsWrapper, &PixelsOptions, &Window)>,
) {
    for event in window_backend_scale_factor_changed_events.read() {
        if let Ok((mut wrapper, options, window)) = query.get_mut(event.window) {
            if options.auto_resize_surface {
                resize_surface_to_window(&mut wrapper, window);
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
pub fn resize_buffer(
    mut query: Query<(&mut PixelsWrapper, &PixelsOptions), Changed<PixelsOptions>>,
) {
    for (mut wrapper, options) in &mut query {
        if options.auto_resize_buffer {
            let _ = wrapper.pixels.resize_buffer(options.width, options.height);
        }
    }
}

/// Render buffer to surface.
#[cfg(feature = "render")]
pub fn render(
    // TODO: Support `RENDER_TIME` diagnostics on web.
    #[cfg(not(target_arch = "wasm32"))] mut diagnostics: Diagnostics,
    query: Query<&PixelsWrapper>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    let start = Instant::now();

    for wrapper in &query {
        wrapper.pixels.render().expect("failed to render pixels");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let end = Instant::now();
        let render_time_seconds = end.duration_since(start).as_secs_f64();
        diagnostics.add_measurement(&diagnostic::RENDER_TIME, || render_time_seconds * 1000.0);
    }
}
