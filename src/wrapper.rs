use bevy::prelude::*;
use pixels::Pixels;

/// Wrapper component for underlying [`Pixels`] struct.
#[derive(Component, Debug)]
pub struct PixelsWrapper {
    pub pixels: Pixels<'static>,
}

#[cfg(target_arch = "wasm32")]
// Bevy components must be Send + Sync, but pixels/wgpu keep the web surface and device types
// thread-affine on wasm. This integration runs on the browser main thread, so marking the wrapper
// as Send + Sync keeps it storable in the ECS without changing its actual execution model.
unsafe impl Send for PixelsWrapper {}

#[cfg(target_arch = "wasm32")]
unsafe impl Sync for PixelsWrapper {}
