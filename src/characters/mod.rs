use bevy::prelude::*;

pub mod registry;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(registry::load_character_registry());
    }
}
