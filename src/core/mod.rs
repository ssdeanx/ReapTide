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
            .add_event::<event::DamageEvent>()
            .add_event::<event::KillEvent>()
            .add_event::<event::XpPickupEvent>()
            .add_event::<event::LevelUpEvent>()
            .add_event::<event::GameOverEvent>()
            .add_event::<event::WaveStartEvent>()
            .add_event::<event::PlayerDeathEvent>();
    }
}
