use bevy::prelude::*;

pub mod registry;
pub mod conditions;
pub mod checker;

pub use registry::*;

pub struct AchievementPlugin;

impl Plugin for AchievementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(registry::load_achievement_registry());
        app.insert_resource(checker::AchievementChecker::default());
        app.add_systems(Update, (
            checker::check_achievements,
            checker::cleanup_achievement_notifications,
        ));
    }
}
