use crate::constants::CHARACTERS;
use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &GameMeshes,
    materials: &GameMaterials,
    profile: &crate::save::PlayerProfile,
) {
    let char_def = CHARACTERS
        .iter()
        .find(|c| c.id == profile.current_character)
        .unwrap_or(&CHARACTERS[0]);

    let hp_bonus = profile.item_purchased("hp_up") * 20;
    let dmg_mult = 1.0 + profile.item_purchased("dmg_up") as f32 * 0.15;

    commands
        .spawn((
            Mesh2d(meshes.player.clone()),
            MeshMaterial2d(materials.player.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player {
                health: char_def.max_hp + hp_bonus as f32,
                max_health: char_def.max_hp + hp_bonus as f32,
                level: 1,
                xp: 0,
                xp_to_next: 10,
                attack_timer: Timer::from_seconds(char_def.attack_interval, TimerMode::Repeating),
                attack_damage: char_def.attack_damage * dmg_mult,
                attack_range: char_def.attack_range,
                upgrade_chosen: false,
                chosen_upgrade: None,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.gem.clone()),
                MeshMaterial2d(materials.player.clone()),
                Transform::from_xyz(20.0, 0.0, 0.0),
            ));
        });
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    upgrade_state: Res<UpgradeState>,
    profile: Res<crate::save::PlayerProfile>,
    mut q: Query<(&mut Transform, &Player)>,
) {
    if upgrade_state.active { return; }
    let Ok((mut tf, _)) = q.single_mut() else { return; };

    let speed_mult = 1.0 + profile.item_purchased("speed_up") as f32 * 0.10;
    let char_def = CHARACTERS
        .iter()
        .find(|c| c.id == profile.current_character)
        .unwrap_or(&CHARACTERS[0]);
    let current_speed = char_def.speed * speed_mult;

    let mut dir = Vec2::ZERO;
    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) { dir.y += 1.0; }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) { dir.y -= 1.0; }
    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) { dir.x -= 1.0; }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) { dir.x += 1.0; }

    if dir != Vec2::ZERO {
        dir = dir.normalize();
        tf.translation.x += dir.x * current_speed * time.delta().as_secs_f32();
        tf.translation.y += dir.y * current_speed * time.delta().as_secs_f32();
        tf.rotation = Quat::from_rotation_z(f32::atan2(dir.y, dir.x));
    }
}
