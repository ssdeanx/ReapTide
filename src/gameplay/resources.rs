use bevy::prelude::*;

// ── Game meshes & materials ──
#[derive(Resource, Clone)]
pub struct GameMeshes {
    pub player: Handle<Mesh>,
    pub small_enemy: Handle<Mesh>,
    pub medium_enemy: Handle<Mesh>,
    pub big_enemy: Handle<Mesh>,
    pub projectile: Handle<Mesh>,
    pub gem: Handle<Mesh>,
    pub ground: Handle<Mesh>,
}

#[derive(Resource, Clone)]
pub struct GameMaterials {
    pub player: Handle<StandardMaterial>,
    pub small_enemy: Handle<StandardMaterial>,
    pub medium_enemy: Handle<StandardMaterial>,
    pub big_enemy: Handle<StandardMaterial>,
    pub projectile_base: Handle<StandardMaterial>,
    pub projectile_damage: Handle<StandardMaterial>,
    pub projectile_speed: Handle<StandardMaterial>,
    pub projectile_range: Handle<StandardMaterial>,
    pub gem: Handle<StandardMaterial>,
    pub particle: Handle<StandardMaterial>,
    pub ground: Handle<StandardMaterial>,
}

// ── Game state resources ──
#[derive(Resource, Default)]
pub struct GameOver {
    pub active: bool,
    pub level: u32,
    pub kills: u64,
    pub survival_time: f64,
    pub damage_dealt: f64,
    pub xp_collected: u64,
}

#[derive(Resource)]
pub struct ScreenShake {
    pub timer: Timer,
    pub magnitude: f32,
    pub current_offset: Vec3,
}

#[derive(Resource, Default)]
pub struct Paused(pub bool);

#[derive(Resource, Default)]
pub struct ZoomLevel(pub f32);

#[derive(Resource, Default)]
pub struct UpgradeState {
    pub active: bool,
}

// ── In-game stats tracking ──
#[derive(Resource, Default)]
pub struct GameStats {
    pub kills: u64,
    pub survival_time: f64,
    pub damage_dealt: f64,
    pub xp_collected: u64,
}

// ── Initialise game meshes & materials (runs once at startup) ──

/// Creates the shared GameMeshes / GameMaterials resources.
/// Registered as a Startup system so the handles exist before any Playing-state system runs.
pub fn init_game_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use crate::constants::*;

    commands.insert_resource(GameMeshes {
        player: meshes.add(Circle::new(12.0)),
        small_enemy: meshes.add(Circle::new(SMALL_ENEMY_RADIUS)),
        medium_enemy: meshes.add(Circle::new(MEDIUM_ENEMY_RADIUS)),
        big_enemy: meshes.add(Circle::new(BIG_ENEMY_RADIUS)),
        projectile: meshes.add(Circle::new(4.0)),
        gem: meshes.add(Circle::new(3.0)),
        // Ground plane — a large flat quad
        ground: meshes.add(Plane3d::default().mesh().size(GROUND_SIZE, GROUND_SIZE)),
    });

    commands.insert_resource(GameMaterials {
        player: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.5, 1.0),
            unlit: true,
            ..default()
        }),
        small_enemy: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            unlit: true,
            ..default()
        }),
        medium_enemy: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.3, 0.1),
            unlit: true,
            ..default()
        }),
        big_enemy: materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.1, 0.1),
            unlit: true,
            ..default()
        }),
        projectile_base: materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.8, 1.0),
            unlit: true,
            ..default()
        }),
        projectile_damage: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.3, 0.1),
            unlit: true,
            ..default()
        }),
        projectile_speed: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 1.0, 0.4),
            unlit: true,
            ..default()
        }),
        projectile_range: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 1.0),
            unlit: true,
            ..default()
        }),
        gem: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 1.0, 0.2),
            unlit: true,
            ..default()
        }),
        particle: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 0.5, 0.8),
            unlit: true,
            ..default()
        }),
        ground: materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.15, 0.2),
            perceptual_roughness: 0.9,
            metallic: 0.0,
            ..default()
        }),
    });
}
