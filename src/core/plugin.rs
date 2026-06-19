use bevy::app::PluginGroup;
use bevy::prelude::*;

// ── Base Plugin Trait ──
// Every module exports a Plugin struct implementing this trait.
// Modules register their own systems, resources, events, and state transitions.

pub trait GamePlugin: Plugin {
    fn name() -> &'static str;
    fn dependencies() -> Vec<&'static str>;
}

// ── Plugin Group ──
// Registers all game modules in dependency order.

pub struct ReapTidePlugins;

impl PluginGroup for ReapTidePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = bevy::app::PluginGroupBuilder::start::<Self>();
        // Core
        group = group.add(crate::core::CorePlugin);
        // Infrastructure (no gameplay dependencies)
        group = group.add(crate::save::SavePlugin);
        group = group.add(crate::audio::AudioPlugin);
        group = group.add(crate::assets::AssetPlugin);
        // Data registries
        group = group.add(crate::characters::CharacterPlugin);
        group = group.add(crate::achievements::AchievementPlugin);
        group = group.add(crate::shop::ShopPlugin);
        // Gameplay
        group = group.add(crate::gameplay::GameplayPlugin);
        // UI (depends on gameplay state)
        group = group.add(crate::ui::UIPlugin);
        group
    }
}
