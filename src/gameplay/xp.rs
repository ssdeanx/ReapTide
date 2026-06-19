use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::prelude::*;

// ── XP Magnet ──

pub fn magnet_xp(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Player>, Without<XpGem>)>,
    mut gem_q: Query<&mut Transform, (With<XpGem>, Without<Player>)>,
) {
    let Ok(p_tf) = player_q.single() else {
        return;
    };
    let pp = p_tf.translation.truncate();
    let dt = time.delta().as_secs_f32();
    for mut tf in &mut gem_q {
        let gp = tf.translation.truncate();
        let dist = pp.distance(gp);
        if dist < crate::constants::MAGNET_RANGE && dist > 0.0 {
            let strength = 1.0 - (dist / crate::constants::MAGNET_RANGE);
            let speed = crate::constants::MAGNET_BASE_SPEED
                + strength
                    * (crate::constants::MAGNET_MAX_SPEED - crate::constants::MAGNET_BASE_SPEED);
            let dir = (pp - gp).normalize();
            tf.translation.x += dir.x * speed * dt;
            tf.translation.y += dir.y * speed * dt;
        }
    }
}

// ── XP Collection & Leveling ──

pub fn collect_xp(
    mut player_q: Query<(&Transform, &mut Player)>,
    gem_q: Query<(Entity, &Transform, &XpGem)>,
    mut commands: Commands,
    mut upgrade_state: ResMut<UpgradeState>,
    mut stats: ResMut<GameStats>,
    mut audio_events: EventWriter<crate::audio::AudioEvent>,
) {
    let Ok((p_tf, mut p)) = player_q.single_mut() else {
        return;
    };
    let pp = p_tf.translation.truncate();

    for (e, g_tf, gem) in &gem_q {
        if pp.distance(g_tf.translation.truncate()) < 36.0 {
            p.xp += gem.0;
            stats.xp_collected += gem.0 as u64;
            commands.entity(e).despawn();
            audio_events.write(crate::audio::AudioEvent::Pickup);

            while p.xp >= p.xp_to_next {
                p.xp -= p.xp_to_next;
                p.level += 1;
                p.xp_to_next = (p.xp_to_next as f32 * 1.5) as u32;
                p.max_health += 10.0;
                p.health = (p.health + 20.0).min(p.max_health);
                p.attack_damage *= 1.15;
                audio_events.write(crate::audio::AudioEvent::LevelUp);

                if p.level == 2 && !p.upgrade_chosen && !upgrade_state.active {
                    upgrade_state.active = true;
                }
            }
        }
    }
}
