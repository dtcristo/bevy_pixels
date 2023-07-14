use crate::{prelude::*, system};

use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow};

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
        app.configure_set(Update, PixelsSet::Update)
            .configure_set(PostUpdate, PixelsSet::Draw)
            .configure_set(Last, PixelsSet::Render)
            .add_systems(Startup, system::setup)
            .add_systems(First, system::create_pixels)
            .add_systems(
                PreUpdate,
                (
                    system::window_change,
                    system::window_resize,
                    system::resize_buffer.after(system::window_resize),
                ),
            );

        #[cfg(feature = "render")]
        {
            app.add_systems(Last, system::render.in_set(PixelsSet::Render));
        }

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
