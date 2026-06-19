// ── Game Constants ──
pub const PLAYER_SPEED: f32 = 280.0;
pub const ATTACK_INTERVAL: f32 = 0.35;
pub const ATTACK_DAMAGE: f32 = 18.0;
pub const ATTACK_RANGE: f32 = 180.0;
pub const PLAYER_MAX_HP: f32 = 100.0;
pub const WAVE_START_INTERVAL: f32 = 2.0;
pub const SPAWN_MIN_DIST: f32 = 400.0;
pub const SPAWN_MAX_DIST: f32 = 650.0;
pub const ENEMY_CONTACT_DPS: f32 = 10.0;
pub const CONTACT_RANGE: f32 = 18.0;
pub const MAGNET_RANGE: f32 = 150.0;
pub const MAGNET_BASE_SPEED: f32 = 80.0;
pub const MAGNET_MAX_SPEED: f32 = 300.0;
pub const GRID_SPACING: f32 = 64.0;
pub const GRID_HALF_EXTENT: f32 = 1200.0;
pub const PARTICLE_COUNT: u32 = 10;
pub const PARTICLE_SPEED: f32 = 150.0;
pub const PARTICLE_LIFETIME: f32 = 0.6;
pub const SMALL_ENEMY_RADIUS: f32 = 7.0;
pub const MEDIUM_ENEMY_RADIUS: f32 = 11.0;
pub const BIG_ENEMY_RADIUS: f32 = 17.0;
pub const SHAKE_DURATION: f32 = 0.2;
pub const SHAKE_MAGNITUDE: f32 = 6.0;
pub const CAMERA_FOLLOW_SPEED: f32 = 0.06;
pub const ZOOM_MIN: f32 = 0.3;
pub const ZOOM_MAX: f32 = 3.0;
pub const ZOOM_STEP: f32 = 0.1;
pub const ZOOM_DEFAULT: f32 = 1.0;

// ── Enemy Stat Constants ──
pub const SMALL_HP_BASE: f32 = 15.0;
pub const SMALL_HP_PER_WAVE: f32 = 5.0;
pub const SMALL_SPEED_BASE: f32 = 80.0;
pub const SMALL_SPEED_PER_WAVE: f32 = 3.0;
pub const SMALL_XP_BASE: u32 = 5;
pub const MEDIUM_HP_BASE: f32 = 40.0;
pub const MEDIUM_HP_PER_WAVE: f32 = 12.0;
pub const MEDIUM_SPEED_BASE: f32 = 55.0;
pub const MEDIUM_SPEED_PER_WAVE: f32 = 2.0;
pub const MEDIUM_XP_BASE: u32 = 15;
pub const BIG_HP_BASE: f32 = 100.0;
pub const BIG_HP_PER_WAVE: f32 = 25.0;
pub const BIG_SPEED_BASE: f32 = 35.0;
pub const BIG_SPEED_PER_WAVE: f32 = 1.0;
pub const BIG_XP_BASE: u32 = 40;

// ── Upgrade Multipliers ──
pub const UPGRADE_ATTACK_SPEED_MULT: f32 = 1.5;
pub const UPGRADE_DAMAGE_MULT: f32 = 1.5;
pub const UPGRADE_RANGE_MULT: f32 = 1.5;

// ── Character Definitions ──
#[derive(Clone)]
pub struct CharacterDef {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub speed: f32,
    pub max_hp: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_interval: f32,
    pub cost: u64,
    pub color: (f32, f32, f32),
}

pub const CHARACTERS: &[CharacterDef] = &[
    CharacterDef { id: "reaper", name: "Reaper", desc: "Balanced reaper of souls", speed: 280.0, max_hp: 100.0, attack_damage: 18.0, attack_range: 180.0, attack_interval: 0.35, cost: 0, color: (0.2, 0.5, 1.0) },
    CharacterDef { id: "harbinger", name: "Harbinger", desc: "Heavy damage, shorter reach", speed: 250.0, max_hp: 80.0, attack_damage: 28.0, attack_range: 140.0, attack_interval: 0.45, cost: 500, color: (1.0, 0.2, 0.2) },
    CharacterDef { id: "shade", name: "Shade", desc: "Swift assassin, fragile", speed: 340.0, max_hp: 65.0, attack_damage: 14.0, attack_range: 200.0, attack_interval: 0.25, cost: 800, color: (0.6, 0.2, 0.8) },
    CharacterDef { id: "wraith", name: "Wraith", desc: "Rapid fire, low damage", speed: 290.0, max_hp: 90.0, attack_damage: 10.0, attack_range: 170.0, attack_interval: 0.18, cost: 1200, color: (0.1, 0.9, 0.6) },
];

// ── Shop Definitions ──
#[derive(Clone)]
pub struct ShopItemDef {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u64,
    pub max_purchases: u32,
}

pub enum ShopCategory {
    Upgrade,
    Character,
    Cosmetic,
}

pub const SHOP_ITEMS: &[ShopItemData] = &[
    ShopItemData { id: "hp_up", name: "Vitality Boon", desc: "+20 max HP", cost: 200, max_purchases: 5, category: ShopCategory::Upgrade },
    ShopItemData { id: "dmg_up", name: "Soul Render", desc: "+15% damage", cost: 300, max_purchases: 3, category: ShopCategory::Upgrade },
    ShopItemData { id: "speed_up", name: "Wind Walker", desc: "+10% move speed", cost: 250, max_purchases: 3, category: ShopCategory::Upgrade },
    ShopItemData { id: "magnet_up", name: "Magnetism", desc: "+30% magnet range", cost: 150, max_purchases: 3, category: ShopCategory::Upgrade },
    ShopItemData { id: "xp_up", name: "Wisdom", desc: "+20% XP gain", cost: 400, max_purchases: 3, category: ShopCategory::Upgrade },
    ShopItemData { id: "extra_proj", name: "Duality", desc: "Fire an extra projectile", cost: 1000, max_purchases: 1, category: ShopCategory::Upgrade },
];

pub struct ShopItemData {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u64,
    pub max_purchases: u32,
    pub category: ShopCategory,
}

// ── Achievement Definitions ──
#[derive(Clone)]
pub struct AchievementDef {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub reward: u64,
}

pub const ACHIEVEMENTS: &[AchievementDef] = &[
    AchievementDef { id: "first_blood", name: "First Blood", desc: "Kill 1 enemy", reward: 50 },
    AchievementDef { id: "centurion", name: "Centurion", desc: "Kill 100 enemies", reward: 200 },
    AchievementDef { id: "survivor", name: "Survivor", desc: "Survive 60 seconds", reward: 150 },
    AchievementDef { id: "level_10", name: "Peak Performance", desc: "Reach level 10", reward: 300 },
    AchievementDef { id: "wave_10", name: "Tidal Wave", desc: "Survive 10 waves", reward: 250 },
    AchievementDef { id: "collector", name: "Collector", desc: "Unlock 3 upgrades", reward: 400 },
    AchievementDef { id: "rich", name: "Soul Hoarder", desc: "Earn 1000 total currency", reward: 500 },
];
