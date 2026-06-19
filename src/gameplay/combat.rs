use crate::constants::ENTITY_HEIGHT;
use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use crate::gameplay::stats::modifier::StatBundle;
use bevy::prelude::*;
use rand::Rng;

// ── Auto-Attack ──
//
// Reads computed stats (attack_range, attack_damage) from StatBundle so that
// modifiers from character base, shop purchases, level-ups, and weapon upgrades
// all apply transparently.

pub fn auto_attack(
    time: Res<Time>,
    upgrade_state: Res<UpgradeState>,
    mut player_q: Query<(&Transform, &mut Player, &StatBundle)>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    meshes: Res<crate::gameplay::resources::GameMeshes>,
    materials: Res<crate::gameplay::resources::GameMaterials>,
    mut audio_events: MessageWriter<crate::audio::AudioEvent>,
) {
    if upgrade_state.active {
        return;
    }
    let Ok((p_tf, mut p, stats)) = player_q.single_mut() else {
        return;
    };
    p.attack_timer.tick(time.delta());
    if !p.attack_timer.just_finished() {
        return;
    }

    let pp = Vec2::new(p_tf.translation.x, p_tf.translation.z);
    let attack_range = stats.get("attack_range");
    let attack_damage = stats.get("attack_damage");
    let mut best: Option<(Vec3, f32)> = None;

    for e_tf in &enemy_q {
        let ep = Vec3::new(e_tf.translation.x, 0.0, e_tf.translation.z);
        let d = pp.distance(Vec2::new(e_tf.translation.x, e_tf.translation.z));
        if d <= attack_range {
            match best {
                Some((_, cd)) if d < cd => best = Some((ep, d)),
                None => best = Some((ep, d)),
                _ => {}
            }
        }
    }

    if let Some((target, _)) = best {
        let (proj_mat, scale) = match p.chosen_upgrade {
            Some(crate::gameplay::components::WeaponUpgrade::Damage) => {
                (materials.projectile_damage.clone(), 1.25)
            }
            Some(crate::gameplay::components::WeaponUpgrade::AttackSpeed) => {
                (materials.projectile_speed.clone(), 0.75)
            }
            Some(crate::gameplay::components::WeaponUpgrade::Range) => {
                (materials.projectile_range.clone(), 1.0)
            }
            None => (materials.projectile_base.clone(), 1.0),
        };

        commands.spawn((
            Mesh3d(meshes.projectile.clone()),
            MeshMaterial3d(proj_mat),
            Transform::from_xyz(pp.x, ENTITY_HEIGHT, pp.y).with_scale(Vec3::splat(scale)),
            Projectile {
                damage: attack_damage,
                life: Timer::from_seconds(0.5, TimerMode::Once),
                target,
            },
        ));
        audio_events.write(crate::audio::AudioEvent::Shoot);
    }
}

// ── Projectile Update ──

pub fn update_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    mut proj_q: Query<(Entity, &mut Transform, &mut Projectile)>,
    mut enemy_q: Query<(&mut Enemy, &Transform), Without<Projectile>>,
) {
    let mut kill: Vec<Entity> = vec![];
    for (e, mut tf, mut p) in &mut proj_q {
        p.life.tick(time.delta());
        if p.life.just_finished() {
            kill.push(e);
            continue;
        }

        // Projectile moves toward target on XZ plane
        let pos = Vec2::new(tf.translation.x, tf.translation.z);
        let target_xz = Vec2::new(p.target.x, p.target.z);
        let dir = (target_xz - pos).normalize();
        let dt = time.delta().as_secs_f32();
        tf.translation.x += dir.x * 400.0 * dt;
        tf.translation.z += dir.y * 400.0 * dt;

        let new_pos = Vec2::new(tf.translation.x, tf.translation.z);
        for (mut enemy, e_tf) in &mut enemy_q {
            let epos = Vec2::new(e_tf.translation.x, e_tf.translation.z);
            if new_pos.distance(epos) < 14.0 {
                enemy.health -= p.damage;
                kill.push(e);
                break;
            }
        }
    }
    for e in kill {
        commands.entity(e).despawn();
    }
}

// ── Enemy Death (with XP drop and particles) ──

pub fn enemy_death(
    mut commands: Commands,
    q: Query<(Entity, &Transform, &Enemy)>,
    meshes: Res<crate::gameplay::resources::GameMeshes>,
    materials: Res<crate::gameplay::resources::GameMaterials>,
    mut stats: ResMut<GameStats>,
    mut audio_events: MessageWriter<crate::audio::AudioEvent>,
) {
    let mut rng = rand::rng();
    for (e, tf, enemy) in &q {
        if enemy.health <= 0.0 {
            // Drop XP gem (on XZ plane at ground level)
            commands.spawn((
                Mesh3d(meshes.gem.clone()),
                MeshMaterial3d(materials.gem.clone()),
                Transform::from_xyz(tf.translation.x, ENTITY_HEIGHT, tf.translation.z),
                XpGem(enemy.xp_value),
            ));

            // Particle burst in 3D (XZ plane with small Y variation)
            let pos = Vec3::new(tf.translation.x, ENTITY_HEIGHT, tf.translation.z);
            for _ in 0..crate::constants::PARTICLE_COUNT {
                let a = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(60.0..crate::constants::PARTICLE_SPEED);
                let vel = Vec3::new(a.cos() * speed, rng.gen_range(-20.0..20.0), a.sin() * speed);
                commands.spawn((
                    Mesh3d(meshes.gem.clone()),
                    MeshMaterial3d(materials.particle.clone()),
                    Transform::from_xyz(pos.x, pos.y, pos.z),
                    Particle {
                        velocity: vel,
                        life: Timer::from_seconds(
                            rng.gen_range(0.25..crate::constants::PARTICLE_LIFETIME),
                            TimerMode::Once,
                        ),
                    },
                ));
            }

            commands.entity(e).despawn();
            stats.kills += 1;
            stats.damage_dealt += enemy.xp_value as f64;
            audio_events.write(crate::audio::AudioEvent::Kill);
        }
    }
}

// ── Contact Damage ──

pub fn contact_damage(
    mut commands: Commands,
    time: Res<Time>,
    upgrade_state: Res<UpgradeState>,
    mut player_q: Query<(&Transform, &mut Player)>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    shake: Option<ResMut<ScreenShake>>,
    mut audio_events: MessageWriter<crate::audio::AudioEvent>,
) {
    if upgrade_state.active {
        return;
    }
    let Ok((p_tf, mut p)) = player_q.single_mut() else {
        return;
    };
    let pp = Vec2::new(p_tf.translation.x, p_tf.translation.z);
    let mut count = 0u32;
    for e_tf in &enemy_q {
        let ep = Vec2::new(e_tf.translation.x, e_tf.translation.z);
        if pp.distance(ep) < crate::constants::CONTACT_RANGE {
            count += 1;
        }
    }
    if count > 0 {
        p.health -= crate::constants::ENEMY_CONTACT_DPS * count as f32 * time.delta().as_secs_f32();
        if let Some(mut s) = shake {
            s.timer.reset();
        } else {
            commands.insert_resource(ScreenShake {
                timer: Timer::from_seconds(crate::constants::SHAKE_DURATION, TimerMode::Once),
                magnitude: crate::constants::SHAKE_MAGNITUDE,
                current_offset: Vec3::ZERO,
            });
        }
        audio_events.write(crate::audio::AudioEvent::Damage);
    }
}
