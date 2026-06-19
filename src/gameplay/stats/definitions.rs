use bevy::prelude::*;
use std::collections::HashMap;

// ── Stat Definition ──

/// Metadata for a single stat: its ID, human-readable name, defaults, and limits.
#[derive(Clone, Debug)]
pub struct StatDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub description: &'static str,
}

impl StatDefinition {
    pub const fn new(id: &'static str, name: &'static str, default: f32, min: f32, max: f32, description: &'static str) -> Self {
        Self { id, name, default, min, max, description }
    }

    /// Clamp a value to this stat's min/max range.
    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min, self.max)
    }
}

// ── Stat Category ──

/// Broad category for grouping stats in UI and tools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StatCategory {
    Core,
    Secondary,
    Resource,
}

// ── StatDefinitions Resource ──

/// Central registry of all stats in the game.
/// One source of truth for stat names, defaults, min/max, and descriptions.
#[derive(Resource, Clone, Debug)]
pub struct StatDefinitions {
    pub core: HashMap<&'static str, StatDefinition>,
    pub secondary: HashMap<&'static str, StatDefinition>,
    pub resource: HashMap<&'static str, StatDefinition>,
}

impl StatDefinitions {
    /// Look up a stat definition by ID across all categories.
    pub fn get(&self, id: &str) -> Option<&StatDefinition> {
        self.core
            .get(id)
            .or_else(|| self.secondary.get(id))
            .or_else(|| self.resource.get(id))
    }

    /// Get default value for a stat.
    pub fn default_value(&self, id: &str) -> f32 {
        self.get(id).map(|d| d.default).unwrap_or(0.0)
    }

    /// Get the category a stat belongs to.
    pub fn category_of(&self, id: &str) -> Option<StatCategory> {
        if self.core.contains_key(id) { Some(StatCategory::Core) }
        else if self.secondary.contains_key(id) { Some(StatCategory::Secondary) }
        else if self.resource.contains_key(id) { Some(StatCategory::Resource) }
        else { None }
    }

    /// Iterate over all stat definitions.
    pub fn all(&self) -> Vec<&StatDefinition> {
        let mut all: Vec<&StatDefinition> = self.core.values().collect();
        all.extend(self.secondary.values());
        all.extend(self.resource.values());
        all
    }

    /// Create a StatBundle initialized with default values for all stats.
    pub fn create_bundle(&self) -> crate::gameplay::stats::modifier::StatBundle {
        let mut bundle = crate::gameplay::stats::modifier::StatBundle::new();
        for def in self.all() {
            bundle.stats.insert(def.id, crate::gameplay::stats::modifier::StatInstance::new(def.default));
        }
        bundle
    }
}

/// Load the full set of game stat definitions.
pub fn load_stat_definitions() -> StatDefinitions {
    let mut core = HashMap::new();
    let mut secondary = HashMap::new();
    let mut resource = HashMap::new();

    // ── Core Stats ──
    core.insert("max_health", StatDefinition::new("max_health", "Max Health", 100.0, 1.0, 9999.0, "Maximum hit points"));
    core.insert("move_speed", StatDefinition::new("move_speed", "Move Speed", 280.0, 50.0, 800.0, "Movement speed in units per second"));
    core.insert("attack_damage", StatDefinition::new("attack_damage", "Attack Damage", 18.0, 1.0, 9999.0, "Base damage per hit"));
    core.insert("attack_range", StatDefinition::new("attack_range", "Attack Range", 180.0, 30.0, 1000.0, "Auto-attack range in units"));
    core.insert("attack_interval", StatDefinition::new("attack_interval", "Attack Interval", 0.35, 0.05, 5.0, "Seconds between attacks"));
    core.insert("armor", StatDefinition::new("armor", "Armor", 0.0, 0.0, 95.0, "Flat damage reduction percentage"));
    core.insert("magic_resist", StatDefinition::new("magic_resist", "Magic Resist", 0.0, 0.0, 95.0, "Magic damage reduction percentage"));

    // ── Secondary Stats ──
    secondary.insert("crit_chance", StatDefinition::new("crit_chance", "Crit Chance", 0.0, 0.0, 1.0, "Probability of a critical hit"));
    secondary.insert("crit_damage", StatDefinition::new("crit_damage", "Crit Damage", 1.5, 1.0, 5.0, "Damage multiplier on critical hits"));
    secondary.insert("dodge_chance", StatDefinition::new("dodge_chance", "Dodge", 0.0, 0.0, 0.8, "Probability to dodge an attack"));
    secondary.insert("life_steal", StatDefinition::new("life_steal", "Life Steal", 0.0, 0.0, 1.0, "Fraction of damage healed as HP"));
    secondary.insert("thorns", StatDefinition::new("thorns", "Thorns", 0.0, 0.0, 999.0, "Damage reflected per hit received"));
    secondary.insert("magnet_range", StatDefinition::new("magnet_range", "Magnet Range", 150.0, 50.0, 800.0, "XP gem attraction radius"));
    secondary.insert("xp_mult", StatDefinition::new("xp_mult", "XP Multiplier", 1.0, 0.5, 5.0, "Experience gain multiplier"));
    secondary.insert("dash_count", StatDefinition::new("dash_count", "Dash Count", 1.0, 0.0, 5.0, "Maximum consecutive dashes"));
    secondary.insert("dash_cooldown", StatDefinition::new("dash_cooldown", "Dash Cooldown", 2.0, 0.5, 10.0, "Seconds between dash charges"));
    secondary.insert("dash_distance", StatDefinition::new("dash_distance", "Dash Distance", 200.0, 50.0, 500.0, "Units traveled per dash"));
    secondary.insert("knockback_power", StatDefinition::new("knockback_power", "Knockback Power", 1.0, 0.0, 5.0, "Knockback force multiplier"));
    secondary.insert("knockback_resist", StatDefinition::new("knockback_resist", "Knockback Resist", 0.0, 0.0, 1.0, "Knockback reduction"));

    // ── Resource Stats ──
    resource.insert("health", StatDefinition::new("health", "Health", 100.0, 0.0, 9999.0, "Current hit points"));
    resource.insert("mana", StatDefinition::new("mana", "Mana", 50.0, 0.0, 9999.0, "Current mana for abilities"));
    resource.insert("max_mana", StatDefinition::new("max_mana", "Max Mana", 50.0, 0.0, 9999.0, "Maximum mana"));
    resource.insert("stamina", StatDefinition::new("stamina", "Stamina", 100.0, 0.0, 9999.0, "Current stamina for dashing"));
    resource.insert("max_stamina", StatDefinition::new("max_stamina", "Max Stamina", 100.0, 0.0, 9999.0, "Maximum stamina"));

    StatDefinitions { core, secondary, resource }
}

// ── Unit Tests ──

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_definitions() {
        let defs = load_stat_definitions();
        assert_eq!(defs.core.len(), 7);
        assert_eq!(defs.secondary.len(), 12);
        assert_eq!(defs.resource.len(), 5);
        assert_eq!(defs.all().len(), 24);
    }

    #[test]
    fn test_get_by_id() {
        let defs = load_stat_definitions();
        let hp = defs.get("max_health").unwrap();
        assert_eq!(hp.name, "Max Health");
        assert!((hp.default - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_category_of() {
        let defs = load_stat_definitions();
        assert_eq!(defs.category_of("max_health"), Some(StatCategory::Core));
        assert_eq!(defs.category_of("crit_chance"), Some(StatCategory::Secondary));
        assert_eq!(defs.category_of("mana"), Some(StatCategory::Resource));
        assert_eq!(defs.category_of("nonexistent"), None);
    }

    #[test]
    fn test_create_bundle() {
        let defs = load_stat_definitions();
        let bundle = defs.create_bundle();
        assert!((bundle.get("max_health") - 100.0).abs() < 0.001);
        assert!((bundle.get("move_speed") - 280.0).abs() < 0.001);
        assert!((bundle.get("nonexistent") - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_clamp() {
        let def = StatDefinition::new("test", "Test", 50.0, 0.0, 100.0, "Test stat");
        assert!((def.clamp(150.0) - 100.0).abs() < 0.001);
        assert!((def.clamp(-10.0) - 0.0).abs() < 0.001);
        assert!((def.clamp(50.0) - 50.0).abs() < 0.001);
    }
}
