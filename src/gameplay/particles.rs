use crate::gameplay::components::*;
use bevy::prelude::*;

pub fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut Particle)>,
) {
    let dt = time.delta().as_secs_f32();
    for (e, mut tf, mut p) in &mut q {
        p.life.tick(time.delta());
        if p.life.just_finished() { commands.entity(e).despawn(); continue; }
        tf.translation.x += p.velocity.x * dt;
        tf.translation.y += p.velocity.y * dt;
        let t = p.life.fraction_remaining();
        tf.scale = Vec3::splat(t);
    }
}
