use bevy::prelude::*;

// ── Global Event Bus ──
// Events for cross-module communication.
// Each plugin reads events it cares about; no direct system calls between modules.

// ── Combat Events ──

#[derive(Event)]
pub struct DamageEvent {
    pub amount: f32,
    pub kind: DamageKind,
    pub source: Entity,
    pub target: Entity,
    pub knockback: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DamageKind {
    Physical,
    Magic,
    TrueDamage,
    Elemental(Element),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Poison,
}

#[derive(Event)]
pub struct KillEvent {
    pub killer: Entity,
    pub victim: Entity,
    pub xp_value: u32,
}

// ── XP Events ──

#[derive(Event)]
pub struct XpPickupEvent {
    pub amount: u32,
    pub target: Entity,
}

#[derive(Event)]
pub struct LevelUpEvent {
    pub entity: Entity,
    pub new_level: u32,
}

// ── Game Events ──

#[derive(Event)]
pub struct GameOverEvent {
    pub level: u32,
    pub kills: u64,
    pub survival_time: f64,
    pub xp_collected: u64,
}

#[derive(Event)]
pub struct WaveStartEvent {
    pub wave: u32,
    pub count: u32,
}

#[derive(Event)]
pub struct PlayerDeathEvent;
