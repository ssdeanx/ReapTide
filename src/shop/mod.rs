use bevy::prelude::*;

pub mod catalog;
pub mod purchases;
pub mod ui;

pub use catalog::*;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(catalog::load_shop_catalog());
        app.add_systems(OnEnter(crate::core::GameOverlayState::Shop), ui::spawn_shop);
        app.add_systems(Update, ui::handle_shop_interactions.run_if(in_state(crate::core::GameOverlayState::Shop)));
        app.add_systems(OnExit(crate::core::GameOverlayState::Shop), ui::despawn_shop);
    }
}
