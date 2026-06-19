use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod stats;
pub use components::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(stats::StatPlugin)
            .add_systems(OnEnter(crate::core::AppState::Playing), setup_gameplay);
    }
}

fn setup_gameplay(mut commands: Commands) {
    commands.insert_resource(PlayerState::default());
    commands.insert_resource(WaveState::default());
}
