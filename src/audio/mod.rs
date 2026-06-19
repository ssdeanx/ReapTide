use bevy::prelude::*;

pub mod sfx;
pub use sfx::AudioEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<sfx::AudioEvent>()
            .add_systems(Startup, sfx::setup_audio)
            .add_systems(Update, sfx::play_audio_events);
    }
}
