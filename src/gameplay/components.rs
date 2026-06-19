use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub level: u32,
    pub xp: u32,
    pub xp_to_next: u32,
}

#[derive(Resource, Default)]
pub struct PlayerState {
    pub character_id: String,
}

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct HudText;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub xp_value: u32,
}

#[derive(Component)]
pub struct XpGem(pub u32);

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub life: Timer,
    pub target: Vec2,
}

#[derive(Component)]
pub struct WaveSpawner {
    pub timer: Timer,
    pub wave: u32,
    pub count: u32,
}

#[derive(Resource, Default)]
pub struct WaveState {
    pub current: u32,
    pub timer: f32,
}
