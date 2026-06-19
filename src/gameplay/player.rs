use crate::constants::{CHARACTERS, ENTITY_HEIGHT};
use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use crate::gameplay::stats::modifier::{ModifierType, StatBundle, StatModifier};
use crate::gameplay::stats::StatDefinitions;
use bevy::prelude::*;

/// Spawn the player entity with a visual mesh, Player component, and StatBundle.
///
/// StatBundle holds all computed stats (max_health, move_speed, attack_damage,
/// attack_range, attack_interval) — base values come from the character definition,
/// and shop purchases are applied as named modifiers on top.
///
/// Player holds only runtime-mutable state (health, level, xp, attack_timer).
/// Use `Query<&StatBundle>` (or `&mut StatBundle`) to read/modify stats.
///
/// In 3D isometric: entity Y = ENTITY_HEIGHT, movement on XZ plane.
pub fn spawn_player(
    commands: &mut Commands,
    meshes: &GameMeshes,
    materials: &GameMaterials,
    profile: &crate::save::PlayerProfile,
    stat_defs: &StatDefinitions,
) {
    let char_def = CHARACTERS
        .iter()
        .find(|c| c.id == profile.current_character)
        .unwrap_or(&CHARACTERS[0]);

    // ── Initialise StatBundle with all stat defaults ──
    let mut bundle = stat_defs.create_bundle();

    // Override base values from the character definition
    if let Some(s) = bundle.stats.get_mut("max_health") {
        s.base = char_def.max_hp;
    }
    if let Some(s) = bundle.stats.get_mut("move_speed") {
        s.base = char_def.speed;
    }
    if let Some(s) = bundle.stats.get_mut("attack_damage") {
        s.base = char_def.attack_damage;
    }
    if let Some(s) = bundle.stats.get_mut("attack_range") {
        s.base = char_def.attack_range;
    }
    if let Some(s) = bundle.stats.get_mut("attack_interval") {
        s.base = char_def.attack_interval;
    }

    // Apply shop purchases as persistent modifiers
    let hp_bonus = profile.item_purchased("hp_up") * 20;
    if hp_bonus > 0 {
        bundle.add_modifier(
            "max_health",
            StatModifier::new("shop_hp_up", hp_bonus as f32, ModifierType::Flat),
        );
    }
    let dmg_pct = profile.item_purchased("dmg_up") as f32 * 0.15;
    if dmg_pct > 0.0 {
        bundle.add_modifier(
            "attack_damage",
            StatModifier::new("shop_dmg_up", dmg_pct, ModifierType::PercentAdd),
        );
    }
    let speed_pct = profile.item_purchased("speed_up") as f32 * 0.10;
    if speed_pct > 0.0 {
        bundle.add_modifier(
            "move_speed",
            StatModifier::new("shop_speed_up", speed_pct, ModifierType::PercentAdd),
        );
    }

    // Read the computed max_health (base + shop) for initial health
    let initial_health = bundle.get("max_health");

    commands
        .spawn((
            Mesh3d(meshes.player.clone()),
            MeshMaterial3d(materials.player.clone()),
            Transform::from_xyz(0.0, ENTITY_HEIGHT, 0.0),
            Player {
                health: initial_health,
                level: 1,
                xp: 0,
                xp_to_next: 10,
                attack_timer: Timer::from_seconds(
                    bundle.get("attack_interval"),
                    TimerMode::Repeating,
                ),
                upgrade_chosen: false,
                chosen_upgrade: None,
            },
            bundle,
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.gem.clone()),
                MeshMaterial3d(materials.player.clone()),
                Transform::from_xyz(20.0, 0.0, 0.0),
            ));
        });
}

/// Move the player based on WASD / arrow keys.
/// Movement is on the XZ plane (Y = height, stays at ENTITY_HEIGHT).
/// Player mesh faces the direction of movement via look_to().
pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    upgrade_state: Res<UpgradeState>,
    mut q: Query<(&mut Transform, &StatBundle), With<Player>>,
) {
    if upgrade_state.active {
        return;
    }
    let Ok((mut tf, stats)) = q.single_mut() else {
        return;
    };

    let current_speed = stats.get("move_speed");

    let mut dir = Vec2::ZERO;
    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        dir.y += 1.0; // Forward -> -Z in world space
    }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        dir.y -= 1.0; // Backward -> +Z in world space
    }
    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        dir.x -= 1.0; // Left -> -X in world space
    }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        dir.x += 1.0; // Right -> +X in world space
    }

    if dir != Vec2::ZERO {
        dir = dir.normalize();
        // XZ plane movement: screen-X = world-X, screen-Y = world-Z
        tf.translation.x += dir.x * current_speed * time.delta().as_secs_f32();
        tf.translation.z -= dir.y * current_speed * time.delta().as_secs_f32();
        // Face movement direction on the XZ plane
        tf.look_to(Vec3::new(dir.x, 0.0, -dir.y), Vec3::Y);
    }
}
