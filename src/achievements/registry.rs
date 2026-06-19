use bevy::prelude::*;

#[derive(Clone)]
pub struct AchievementDef {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub reward: u64,
}

#[derive(Resource)]
pub struct AchievementRegistry {
    pub achievements: Vec<AchievementDef>,
}

pub fn load_achievement_registry() -> AchievementRegistry {
    AchievementRegistry {
        achievements: vec![
            AchievementDef { id: "first_blood", name: "First Blood", desc: "Kill 1 enemy", reward: 50 },
            AchievementDef { id: "centurion", name: "Centurion", desc: "Kill 100 enemies", reward: 200 },
            AchievementDef { id: "survivor", name: "Survivor", desc: "Survive 60 seconds", reward: 150 },
            AchievementDef { id: "level_10", name: "Peak Performance", desc: "Reach level 10", reward: 300 },
            AchievementDef { id: "wave_10", name: "Tidal Wave", desc: "Survive 10 waves", reward: 250 },
            AchievementDef { id: "collector", name: "Collector", desc: "Unlock 3 upgrades", reward: 400 },
            AchievementDef { id: "rich", name: "Soul Hoarder", desc: "Earn 1000 total currency", reward: 500 },
        ],
    }
}
