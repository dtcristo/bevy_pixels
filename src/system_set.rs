use bevy::prelude::*;

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
