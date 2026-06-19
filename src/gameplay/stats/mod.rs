pub mod modifier;
pub mod definitions;

pub use modifier::*;
pub use definitions::*;

use bevy::prelude::*;

pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(definitions::load_stat_definitions());

        // System to clean up expired modifiers each frame
        app.add_systems(Update, cleanup_expired_modifiers);
    }
}

/// Remove expired modifiers from all entities with StatBundle each frame.
fn cleanup_expired_modifiers(
    time: Res<Time>,
    mut q: Query<&mut StatBundle>,
) {
    let elapsed = time.elapsed_secs();
    for mut bundle in &mut q {
        for instance in bundle.stats.values_mut() {
            instance.remove_expired(elapsed);
        }
    }
}
