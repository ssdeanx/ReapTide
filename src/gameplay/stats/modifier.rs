use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

// ── Modifier Types ──

/// How a modifier affects a stat's final value.
#[derive(Clone, Debug, PartialEq)]
pub enum ModifierType {
    /// +X to the stat (applied to base before percentages)
    Flat,
    /// +X% (additive with other PercentAdd modifiers)
    PercentAdd,
    /// ×X multiplier (multiplicative, applied last after all Flat + PercentAdd)
    PercentMult,
    /// Override base value entirely (ignores all other modifiers)
    Set,
}

// ── Stat Modifier ──

/// A single modifier applied to a stat instance.
/// Tracks its source for removal and debugging, plus optional expiry.
#[derive(Clone, Debug)]
pub struct StatModifier {
    pub source_id: String,
    pub value: f32,
    pub modifier_type: ModifierType,
    pub duration: Option<Duration>,
    pub expires_at: Option<f32>,
}

impl StatModifier {
    pub fn new(source_id: &str, value: f32, modifier_type: ModifierType) -> Self {
        Self {
            source_id: source_id.to_string(),
            value,
            modifier_type,
            duration: None,
            expires_at: None,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        let expires_at = Some(duration.as_secs_f32());
        self.duration = Some(duration);
        self.expires_at = expires_at;
        self
    }

    pub fn is_expired(&self, time: f32) -> bool {
        self.expires_at.map_or(false, |exp| time >= exp)
    }
}

// ── Stat Instance ──

/// A single stat with a base value and a list of modifiers.
/// Computes the final value by applying modifiers in order: Set > Flat > PercentAdd > PercentMult.
#[derive(Clone, Debug)]
pub struct StatInstance {
    pub base: f32,
    pub modifiers: Vec<StatModifier>,
}

impl StatInstance {
    pub fn new(base: f32) -> Self {
        Self {
            base,
            modifiers: Vec::new(),
        }
    }

    /// Add a modifier to this stat.
    pub fn add_modifier(&mut self, modifier: StatModifier) {
        self.modifiers.push(modifier);
    }

    /// Remove all modifiers from a given source.
    pub fn remove_modifier(&mut self, source_id: &str) {
        self.modifiers.retain(|m| m.source_id != source_id);
    }

    /// Remove expired modifiers (call each frame).
    pub fn remove_expired(&mut self, time: f32) {
        self.modifiers.retain(|m| !m.is_expired(time));
    }

    /// Compute the final stat value.
    /// ```
    /// Order: Set > Flat > PercentAdd > PercentMult
    /// If any Set modifier exists, returns its value directly.
    /// Otherwise: (base + sum(Flat)) * (1.0 + sum(PercentAdd)) * product(PercentMult)
    /// ```
    pub fn value(&self) -> f32 {
        let mut flat_sum = 0.0;
        let mut pct_sum = 0.0;
        let mut pct_mult = 1.0;
        let mut set_value: Option<f32> = None;

        for m in &self.modifiers {
            match m.modifier_type {
                ModifierType::Flat => flat_sum += m.value,
                ModifierType::PercentAdd => pct_sum += m.value,
                ModifierType::PercentMult => pct_mult *= m.value,
                ModifierType::Set => set_value = Some(m.value),
            }
        }

        if let Some(sv) = set_value {
            return sv;
        }

        (self.base + flat_sum) * (1.0 + pct_sum) * pct_mult
    }

    /// Returns a human-readable breakdown of each modifier's contribution.
    /// Useful for debug UI and tooltips.
    pub fn breakdown(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("Base: {:.1}", self.base));

        let mut flat_sum = 0.0;
        let mut pct_sum = 0.0;
        let mut pct_mult = 1.0;

        for m in &self.modifiers {
            match m.modifier_type {
                ModifierType::Flat => {
                    flat_sum += m.value;
                    lines.push(format!("  +{:.1} (Flat) [{}]", m.value, m.source_id));
                }
                ModifierType::PercentAdd => {
                    pct_sum += m.value;
                    lines.push(format!("  +{:.0}% (Add) [{}]", m.value * 100.0, m.source_id));
                }
                ModifierType::PercentMult => {
                    pct_mult *= m.value;
                    lines.push(format!("  ×{:.2} (Mult) [{}]", m.value, m.source_id));
                }
                ModifierType::Set => {
                    lines.push(format!("  ={:.1} (Set) [{}]", m.value, m.source_id));
                }
            }
        }

        lines.push(format!("Final: {:.1}", self.value()));
        lines
    }
}

// ── Convenient resource for entity-scoped stats ──

/// A Bevy component that holds a map of stat instances for an entity.
/// Attach this to player, enemies, etc.
#[derive(Component, Clone)]
pub struct StatBundle {
    pub stats: HashMap<&'static str, StatInstance>,
}

impl StatBundle {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn with(mut self, id: &'static str, base: f32) -> Self {
        self.stats.insert(id, StatInstance::new(base));
        self
    }

    pub fn get(&self, id: &str) -> f32 {
        self.stats
            .get(id)
            .map(|s| s.value())
            .unwrap_or(0.0)
    }

    pub fn get_instance(&self, id: &str) -> Option<&StatInstance> {
        self.stats.get(id)
    }

    pub fn get_instance_mut(&mut self, id: &str) -> Option<&mut StatInstance> {
        self.stats.get_mut(id)
    }

    pub fn add_modifier(&mut self, stat_id: &str, modifier: StatModifier) {
        if let Some(instance) = self.stats.get_mut(stat_id) {
            instance.add_modifier(modifier);
        }
    }

    pub fn remove_modifier(&mut self, source_id: &str) {
        for instance in self.stats.values_mut() {
            instance.remove_modifier(source_id);
        }
    }
}

impl Default for StatBundle {
    fn default() -> Self {
        Self::new()
    }
}

// ── Unit Tests ──

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_only() {
        let mut s = StatInstance::new(100.0);
        s.add_modifier(StatModifier::new("test", 20.0, ModifierType::Flat));
        assert!((s.value() - 120.0).abs() < 0.001);
    }

    #[test]
    fn test_percent_add() {
        let mut s = StatInstance::new(100.0);
        s.add_modifier(StatModifier::new("test", 0.5, ModifierType::PercentAdd));
        assert!((s.value() - 150.0).abs() < 0.001);
    }

    #[test]
    fn test_percent_mult() {
        let mut s = StatInstance::new(100.0);
        s.add_modifier(StatModifier::new("test", 2.0, ModifierType::PercentMult));
        assert!((s.value() - 200.0).abs() < 0.001);
    }

    #[test]
    fn test_combined_modifiers() {
        let mut s = StatInstance::new(100.0);
        s.add_modifier(StatModifier::new("armor", 50.0, ModifierType::Flat));
        s.add_modifier(StatModifier::new("buff", 0.2, ModifierType::PercentAdd));
        s.add_modifier(StatModifier::new("debuff", 0.5, ModifierType::PercentMult));
        // (100 + 50) * (1.0 + 0.2) * 0.5 = 150 * 1.2 * 0.5 = 90
        assert!((s.value() - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_set_overrides() {
        let mut s = StatInstance::new(100.0);
        s.add_modifier(StatModifier::new("flat", 50.0, ModifierType::Flat));
        s.add_modifier(StatModifier::new("set", 999.0, ModifierType::Set));
        assert_eq!(s.value(), 999.0);
    }

    #[test]
    fn test_remove_modifier() {
        let mut s = StatInstance::new(100.0);
        s.add_modifier(StatModifier::new("buff", 50.0, ModifierType::Flat));
        assert!((s.value() - 150.0).abs() < 0.001);
        s.remove_modifier("buff");
        assert!((s.value() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_remove_expired() {
        let mut s = StatInstance::new(100.0);
        let mut m = StatModifier::new("temp", 50.0, ModifierType::Flat);
        m.expires_at = Some(1.0);
        s.add_modifier(m);
        assert!((s.value() - 150.0).abs() < 0.001);
        s.remove_expired(2.0);
        assert!((s.value() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_no_modifiers() {
        let s = StatInstance::new(50.0);
        assert_eq!(s.value(), 50.0);
    }

    #[test]
    fn test_stat_bundle() {
        let mut bundle = StatBundle::new()
            .with("max_health", 100.0)
            .with("move_speed", 280.0);

        assert!((bundle.get("max_health") - 100.0).abs() < 0.001);
        assert!((bundle.get("move_speed") - 280.0).abs() < 0.001);

        bundle.add_modifier("move_speed", StatModifier::new("boots", 50.0, ModifierType::Flat));
        assert!((bundle.get("move_speed") - 330.0).abs() < 0.001);
    }
}
