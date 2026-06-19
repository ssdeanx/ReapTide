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
}

#[derive(Resource, Clone)]
pub struct GameMaterials {
    pub player: Handle<ColorMaterial>,
    pub small_enemy: Handle<ColorMaterial>,
    pub medium_enemy: Handle<ColorMaterial>,
    pub big_enemy: Handle<ColorMaterial>,
    pub projectile_base: Handle<ColorMaterial>,
    pub projectile_damage: Handle<ColorMaterial>,
    pub projectile_speed: Handle<ColorMaterial>,
    pub projectile_range: Handle<ColorMaterial>,
    pub gem: Handle<ColorMaterial>,
    pub particle: Handle<ColorMaterial>,
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

#[derive(Resource)]
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
