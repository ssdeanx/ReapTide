use bevy::prelude::*;

pub mod screen;
pub mod components;
pub mod theme;
pub mod menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(crate::core::AppState::MainMenu), menu::spawn_main_menu)
            .add_systems(Update, menu::handle_menu_interactions.run_if(in_state(crate::core::AppState::MainMenu)))
            .add_systems(OnExit(crate::core::AppState::MainMenu), screen::despawn_screen::<menu::MenuRoot>);
    }
}
