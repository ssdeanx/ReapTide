use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::prelude::*;

// ── Auto-Attack ──

pub fn auto_attack(
    time: Res<Time>,
    upgrade_state: Res<UpgradeState>,
    mut player_q: Query<(&Transform, &mut Player)>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    meshes: Res<crate::gameplay::resources::GameMeshes>,
    materials: Res<crate::gameplay::resources::GameMaterials>,
    mut audio_events: MessageWriter<crate::audio::AudioEvent>,
) {
    if upgrade_state.active { return; }
    let Ok((p_tf, mut p)) = player_q.single_mut() else { return; };
    p.attack_timer.tick(time.delta());
    if !p.attack_timer.just_finished() { return; }

    let pp = p_tf.translation.truncate();
    let mut best: Option<(Vec2, f32)> = None;

    for e_tf in &enemy_q {
        let ep = e_tf.translation.truncate();
        let d = pp.distance(ep);
        if d <= p.attack_range {
            match best {
                Some((_, cd)) if d < cd => best = Some((ep, d)),
                None => best = Some((ep, d)),
                _ => {}
            }
        }
    }

    if let Some((target, _)) = best {
        let (proj_mat, scale) = match p.chosen_upgrade {
            Some(crate::constants::WeaponUpgrade::Damage) => (materials.projectile_damage.clone(), 1.25),
            Some(crate::constants::WeaponUpgrade::AttackSpeed) => (materials.projectile_speed.clone(), 0.75),
            Some(crate::constants::WeaponUpgrade::Range) => (materials.projectile_range.clone(), 1.0),
            None => (materials.projectile_base.clone(), 1.0),
        };

        commands.spawn((
            Mesh2d(meshes.projectile.clone()),
            MeshMaterial2d(proj_mat),
            Transform::from_xyz(pp.x, pp.y, 0.0).with_scale(Vec3::splat(scale)),
            Projectile { damage: p.attack_damage, life: Timer::from_seconds(0.5, TimerMode::Once), target },
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
        if p.life.just_finished() { kill.push(e); continue; }

        let pos = tf.translation.truncate();
        let dir = (p.target - pos).normalize();
        tf.translation.x += dir.x * 400.0 * time.delta().as_secs_f32();
        tf.translation.y += dir.y * 400.0 * time.delta().as_secs_f32();

        let new_pos = tf.translation.truncate();
        for (mut enemy, e_tf) in &mut enemy_q {
            if new_pos.distance(e_tf.translation.truncate()) < 14.0 {
                enemy.health -= p.damage;
                kill.push(e);
                break;
            }
        }
    }
    for e in kill { commands.entity(e).despawn(); }
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
    let mut rng = rand::thread_rng();
    for (e, tf, enemy) in &q {
        if enemy.health <= 0.0 {
            // Drop XP gem
            commands.spawn((
                Mesh2d(meshes.gem.clone()),
                MeshMaterial2d(materials.gem.clone()),
                Transform::from_xyz(tf.translation.x, tf.translation.y, 0.0),
                XpGem(enemy.xp_value),
            ));

            // Particle burst
            let pos = tf.translation.truncate();
            for _ in 0..crate::constants::PARTICLE_COUNT {
                let a = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(60.0..crate::constants::PARTICLE_SPEED);
                let vel = Vec2::new(a.cos() * speed, a.sin() * speed);
                commands.spawn((
                    Mesh2d(meshes.gem.clone()),
                    MeshMaterial2d(materials.particle.clone()),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                    Particle { velocity: vel, life: Timer::from_seconds(rng.gen_range(0.25..crate::constants::PARTICLE_LIFETIME), TimerMode::Once) },
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
    if upgrade_state.active { return; }
    let Ok((p_tf, mut p)) = player_q.single_mut() else { return; };
    let pp = p_tf.translation.truncate();
    let mut count = 0u32;
    for e_tf in &enemy_q {
        if pp.distance(e_tf.translation.truncate()) < crate::constants::CONTACT_RANGE { count += 1; }
    }
    if count > 0 {
        p.health -= crate::constants::ENEMY_CONTACT_DPS * count as f32 * time.delta().as_secs_f32();
        if let Some(mut s) = shake { s.timer.reset(); }
        else {
            commands.insert_resource(ScreenShake {
                timer: Timer::from_seconds(crate::constants::SHAKE_DURATION, TimerMode::Once),
                magnitude: crate::constants::SHAKE_MAGNITUDE,
                current_offset: Vec3::ZERO,
            });
        }
        audio_events.write(crate::audio::AudioEvent::Damage);
    }
}
