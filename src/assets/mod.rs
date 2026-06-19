use bevy::prelude::*;

pub mod loading;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        // Asset loading is handled by Bevy's AssetServer by default
    }
}
