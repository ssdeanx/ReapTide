use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::prelude::*;
use rand::Rng;

// ── Enemy Spawning ──

pub fn spawn_enemies(
    time: Res<Time>,
    upgrade_state: Res<UpgradeState>,
    mut q: Query<&mut WaveSpawner>,
    mut commands: Commands,
    meshes: Res<GameMeshes>,
    materials: Res<GameMaterials>,
) {
    if upgrade_state.active { return; }
    let Ok(mut s) = q.single_mut() else { return; };
    s.timer.tick(time.delta());
    if !s.timer.just_finished() { return; }

    s.wave += 1;
    let n = (s.count + s.wave / 3).min(50);
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let dist = rng.gen_range(crate::constants::SPAWN_MIN_DIST..crate::constants::SPAWN_MAX_DIST);
        let x = angle.cos() * dist;
        let y = angle.sin() * dist;

        let kind_roll: f32 = rng.gen();
        let (kind, mesh, mat, hp_base, hp_per, spd_base, spd_per, xp_base) = if kind_roll < 0.40 {
            (EnemyKind::Small, meshes.small_enemy.clone(), materials.small_enemy.clone(),
             crate::constants::SMALL_HP_BASE, crate::constants::SMALL_HP_PER_WAVE,
             crate::constants::SMALL_SPEED_BASE, crate::constants::SMALL_SPEED_PER_WAVE,
             crate::constants::SMALL_XP_BASE)
        } else if kind_roll < 0.75 {
            (EnemyKind::Medium, meshes.medium_enemy.clone(), materials.medium_enemy.clone(),
             crate::constants::MEDIUM_HP_BASE, crate::constants::MEDIUM_HP_PER_WAVE,
             crate::constants::MEDIUM_SPEED_BASE, crate::constants::MEDIUM_SPEED_PER_WAVE,
             crate::constants::MEDIUM_XP_BASE)
        } else {
            (EnemyKind::Big, meshes.big_enemy.clone(), materials.big_enemy.clone(),
             crate::constants::BIG_HP_BASE, crate::constants::BIG_HP_PER_WAVE,
             crate::constants::BIG_SPEED_BASE, crate::constants::BIG_SPEED_PER_WAVE,
             crate::constants::BIG_XP_BASE)
        };

        let hp = hp_base + s.wave as f32 * hp_per;
        let spd = spd_base + s.wave as f32 * spd_per;
        let xp = xp_base + s.wave / 3;

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(mat),
            Transform::from_xyz(x, y, 0.0),
            Enemy { health: hp + rng.gen_range(-3.0..3.0), speed: spd + rng.gen_range(-10.0..10.0), xp_value: xp, kind },
        ));
    }

    s.count = n;
    let interval = (crate::constants::WAVE_START_INTERVAL - s.wave as f32 * 0.08).max(0.7);
    s.timer = Timer::from_seconds(interval, TimerMode::Repeating);
}

// ── Enemy Chase AI ──

pub fn enemies_chase(
    time: Res<Time>,
    upgrade_state: Res<UpgradeState>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    if upgrade_state.active { return; }
    let Ok(p_tf) = player_q.single() else { return; };
    let pp = p_tf.translation.truncate();
    for (mut tf, e) in &mut enemy_q {
        let dir = (pp - tf.translation.truncate()).normalize();
        tf.translation.x += dir.x * e.speed * time.delta().as_secs_f32();
        tf.translation.y += dir.y * e.speed * time.delta().as_secs_f32();
    }
}
