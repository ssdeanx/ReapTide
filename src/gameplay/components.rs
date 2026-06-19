use bevy::prelude::*;

// ── Weapon Upgrade Enum ──

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WeaponUpgrade {
    AttackSpeed,
    Damage,
    Range,
}

// ── Enemy Kind Enum ──

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyKind {
    Small,
    Medium,
    Big,
}

// ── Player Component ──

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub level: u32,
    pub xp: u32,
    pub xp_to_next: u32,
    pub attack_timer: Timer,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub upgrade_chosen: bool,
    pub chosen_upgrade: Option<WeaponUpgrade>,
}

#[derive(Resource, Default)]
pub struct PlayerState {
    pub character_id: String,
}

// ── UI Marker Components ──

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct HudText;

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct ControlsOverlayRoot;

#[derive(Component)]
pub struct GameOverOverlay;

#[derive(Component)]
pub struct UpgradeMenuRoot;

#[derive(Component)]
pub struct UpgradeChoiceButton {
    pub upgrade: WeaponUpgrade,
}

#[derive(Component)]
pub struct DarkOverlay;

// ── Camera ──

#[derive(Component)]
pub struct GameCamera;

// ── Enemy Component ──

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub xp_value: u32,
    pub kind: EnemyKind,
}

// ── Particle Component ──

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub life: Timer,
}

// ── XP Gem ──

#[derive(Component)]
pub struct XpGem(pub u32);

// ── Projectile ──

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub life: Timer,
    pub target: Vec2,
}

// ── Wave Spawner ──

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
