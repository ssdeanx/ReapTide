use bevy::prelude::*;

pub mod profile;

pub use profile::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_profile());
    }
}
