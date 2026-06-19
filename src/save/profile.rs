use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use bevy::prelude::Resource;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct PlayerProfile {
    pub player_name: String,
    pub currency: u64,
    pub total_stats: LifetimeStats,
    pub unlocked_items: HashSet<String>,
    pub purchased_upgrades: Vec<(String, u32)>,
    pub unlocked_characters: HashSet<String>,
    pub achievements_unlocked: HashSet<String>,
    pub current_character: String,
}

impl Default for PlayerProfile {
    fn default() -> Self {
        let mut chars = HashSet::new();
        chars.insert("reaper".into());
        Self {
            player_name: "Reaper".into(),
            currency: 0,
            total_stats: LifetimeStats::default(),
            unlocked_items: HashSet::new(),
            purchased_upgrades: Vec::new(),
            unlocked_characters: chars,
            achievements_unlocked: HashSet::new(),
            current_character: "reaper".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LifetimeStats {
    pub games_played: u32,
    pub total_kills: u64,
    pub highest_level: u32,
    pub longest_survival_seconds: f64,
    pub total_damage_dealt: f64,
    pub total_currency_earned: u64,
    pub total_xp_collected: u64,
}

impl PlayerProfile {
    pub fn character_purchased(&self, id: &str) -> bool {
        self.unlocked_characters.contains(id)
    }
    pub fn item_purchased(&self, id: &str) -> u32 {
        self.purchased_upgrades.iter().find(|(i, _)| i == id).map(|(_, c)| *c).unwrap_or(0)
    }
    pub fn has_achievement(&self, id: &str) -> bool {
        self.achievements_unlocked.contains(id)
    }
}

fn data_dir() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        return PathBuf::from(appdata).join("reaptide");
    }
    if let Some(home) = std::env::var("HOME").ok().or_else(|| std::env::var("USERPROFILE").ok()) {
        return PathBuf::from(home).join(".reaptide");
    }
    PathBuf::from(".")
}

pub fn save_path() -> PathBuf {
    data_dir().join("save.json")
}

pub fn load_profile() -> PlayerProfile {
    let path = save_path();
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(profile) = serde_json::from_str(&data) { return profile; }
    }
    PlayerProfile::default()
}

pub fn save_profile(profile: &PlayerProfile) {
    let path = save_path();
    if let Some(dir) = path.parent() { let _ = fs::create_dir_all(dir); }
    if let Ok(data) = serde_json::to_string_pretty(profile) { let _ = fs::write(&path, data); }
}

pub fn award_currency(profile: &mut PlayerProfile, amount: u64) {
    profile.currency += amount;
    profile.total_stats.total_currency_earned += amount;
}
