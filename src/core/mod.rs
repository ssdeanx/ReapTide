pub mod event;
pub mod plugin;
pub mod state;

pub use event::*;
pub use plugin::*;
pub use state::*;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GameOverlayState>()
            .insert_resource(StateTransitionRequest::default())
            // Register core events
            .add_message::<event::DamageEvent>()
            .add_message::<event::KillEvent>()
            .add_message::<event::XpPickupEvent>()
            .add_message::<event::LevelUpEvent>()
            .add_message::<event::GameOverEvent>()
            .add_message::<event::WaveStartEvent>()
            .add_message::<event::PlayerDeathEvent>();
    }
}
