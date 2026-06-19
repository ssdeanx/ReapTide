use bevy::prelude::*;

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

#[derive(Resource)]
pub struct CharacterRegistry {
    pub characters: Vec<CharacterDef>,
}

impl CharacterRegistry {
    pub fn get(&self, id: &str) -> Option<&CharacterDef> {
        self.characters.iter().find(|c| c.id == id)
    }
}

pub fn load_character_registry() -> CharacterRegistry {
    CharacterRegistry {
        characters: vec![
            CharacterDef { id: "reaper", name: "Reaper", desc: "Balanced reaper of souls", speed: 280.0, max_hp: 100.0, attack_damage: 18.0, attack_range: 180.0, attack_interval: 0.35, cost: 0, color: (0.2, 0.5, 1.0) },
            CharacterDef { id: "harbinger", name: "Harbinger", desc: "Heavy damage, shorter reach", speed: 250.0, max_hp: 80.0, attack_damage: 28.0, attack_range: 140.0, attack_interval: 0.45, cost: 500, color: (1.0, 0.2, 0.2) },
            CharacterDef { id: "shade", name: "Shade", desc: "Swift assassin, fragile", speed: 340.0, max_hp: 65.0, attack_damage: 14.0, attack_range: 200.0, attack_interval: 0.25, cost: 800, color: (0.6, 0.2, 0.8) },
            CharacterDef { id: "wraith", name: "Wraith", desc: "Rapid fire, low damage", speed: 290.0, max_hp: 90.0, attack_damage: 10.0, attack_range: 170.0, attack_interval: 0.18, cost: 1200, color: (0.1, 0.9, 0.6) },
        ],
    }
}
