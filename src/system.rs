#[cfg(feature = "render")]
#[cfg(not(target_arch = "wasm32"))]
use crate::diagnostic;
use crate::prelude::*;

#[cfg(feature = "render")]
#[cfg(not(target_arch = "wasm32"))]
use bevy::diagnostic::Diagnostics;
#[cfg(target_arch = "wasm32")]
use bevy::tasks::{AsyncComputeTaskPool, Task, futures::check_ready};
use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    window::{PresentMode, RawHandleWrapper},
};
use pixels::{PixelsBuilder, SurfaceTexture};
#[cfg(feature = "render")]
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Component)]
pub(crate) struct PendingPixels(Task<Result<PixelsWrapper, pixels::Error>>);

fn pixels_present_mode(present_mode: PresentMode) -> pixels::wgpu::PresentMode {
    match present_mode {
        PresentMode::Fifo => pixels::wgpu::PresentMode::Fifo,
        PresentMode::FifoRelaxed => pixels::wgpu::PresentMode::FifoRelaxed,
        PresentMode::Mailbox => pixels::wgpu::PresentMode::Mailbox,
        PresentMode::Immediate => pixels::wgpu::PresentMode::Immediate,
        PresentMode::AutoVsync => pixels::wgpu::PresentMode::AutoVsync,
        PresentMode::AutoNoVsync => pixels::wgpu::PresentMode::AutoNoVsync,
    }
}

/// Create [`PixelsWrapper`] (and underlying [`Pixels`] buffer) for all suitable [`Window`] with
/// a [`PixelsOptions`] component.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::type_complexity)]
pub fn create_pixels(
    mut commands: Commands,
    query: Query<(Entity, &PixelsOptions, &Window, &RawHandleWrapper), Without<PixelsWrapper>>,
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

        let pixels = PixelsBuilder::new(options.width, options.height, surface_texture)
            .present_mode(pixels_present_mode(window.present_mode))
            .build()
            .expect("failed to create pixels");

        commands.entity(entity).insert(PixelsWrapper { pixels });
    }
}

/// Begin creating [`PixelsWrapper`] asynchronously for suitable web windows.
#[cfg(target_arch = "wasm32")]
#[allow(clippy::type_complexity)]
pub fn create_pixels(
    mut commands: Commands,
    query: Query<
        (Entity, &PixelsOptions, &Window, &RawHandleWrapper),
        (Without<PixelsWrapper>, Without<PendingPixels>),
    >,
    _main_thread: NonSendMarker,
) {
    for (entity, options, window, raw_handle_wrapper) in &query {
        // SAFETY: `NonSendMarker` forces this system onto Bevy's main thread. The spawned local
        // task remains on the browser thread required by the thread-affine window handle.
        let thread_locked_handle = unsafe { raw_handle_wrapper.get_handle() };
        let surface_texture = SurfaceTexture::new(
            window.physical_width(),
            window.physical_height(),
            thread_locked_handle,
        );
        let builder = PixelsBuilder::new(options.width, options.height, surface_texture)
            .present_mode(pixels_present_mode(window.present_mode))
            .texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm)
            .surface_texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm);
        let task = AsyncComputeTaskPool::get().spawn_local(async move {
            builder
                .build_async()
                .await
                .map(|pixels| PixelsWrapper { pixels })
        });

        commands.entity(entity).insert(PendingPixels(task));
    }
}

/// Finish creating [`PixelsWrapper`] for web windows whose asynchronous initialization completed.
#[cfg(target_arch = "wasm32")]
pub fn finish_pixels_initialization(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PendingPixels)>,
) {
    for (entity, mut pending) in &mut query {
        if let Some(result) = check_ready(&mut pending.0) {
            let wrapper = result.expect("failed to create pixels");
            commands
                .entity(entity)
                .remove::<PendingPixels>()
                .insert(wrapper);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_mode_mapping_matches_pixels() {
        assert_eq!(
            pixels_present_mode(PresentMode::Fifo),
            pixels::wgpu::PresentMode::Fifo
        );
        assert_eq!(
            pixels_present_mode(PresentMode::FifoRelaxed),
            pixels::wgpu::PresentMode::FifoRelaxed
        );
        assert_eq!(
            pixels_present_mode(PresentMode::Mailbox),
            pixels::wgpu::PresentMode::Mailbox
        );
        assert_eq!(
            pixels_present_mode(PresentMode::Immediate),
            pixels::wgpu::PresentMode::Immediate
        );
        assert_eq!(
            pixels_present_mode(PresentMode::AutoVsync),
            pixels::wgpu::PresentMode::AutoVsync
        );
        assert_eq!(
            pixels_present_mode(PresentMode::AutoNoVsync),
            pixels::wgpu::PresentMode::AutoNoVsync
        );
    }
}
