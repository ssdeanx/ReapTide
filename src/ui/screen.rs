use bevy::prelude::*;

pub fn despawn_screen<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in &q { commands.entity(e).despawn(); }
}
