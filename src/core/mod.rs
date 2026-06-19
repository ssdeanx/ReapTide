pub mod state;
pub mod plugin;
pub mod event;

pub use state::*;
pub use plugin::*;
pub use event::*;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .init_state::<GameOverlayState>()
            .insert_resource(StateTransitionRequest::default());
    }
}
