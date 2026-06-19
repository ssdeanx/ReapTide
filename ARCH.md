# ReapTide — Production Architecture

## State Machine
Menu → Playing → GameOver → Menu
                    ↓             
              LevelUp (pause overlay)
              
Shop accessible from Menu (between runs).

## Project Structure (multi-module)
```
src/
  main.rs          — App entry, state transitions, plugin registration
  constants.rs     — Game balance constants, shared enums/types
  save.rs          — Profile struct, serde save/load (JSON), stats tracking
  menu.rs          — Start menu UI, character select, shop access
  gameplay.rs      — ALL gameplay systems (player, enemies, combat, XP, etc.)
  shop.rs          — Shop UI, item definitions, purchasing
  achievements.rs  — Achievement defs, condition checking, UI notifications
  upgrades.rs      — Weapon evolution menu + choices
```

## Data Model

### PlayerProfile (persistent JSON, ~/.reaptide/save.json)
```rust
struct PlayerProfile {
    player_name: String,
    currency: u64,                // "Souls" earned from runs
    total_stats: LifetimeStats,
    unlocks: Vec<String>,         // IDs of unlocked items/characters
    equipped_upgrades: Vec<String>, // Active permanent account upgrades
    characters: Vec<CharacterUnlock>,
    achievements: Vec<AchievementStatus>,
    settings: GameSettings,
}
```

### LifetimeStats — tracked across all runs
```rust
struct LifetimeStats {
    games_played: u32,
    total_kills: u64,
    highest_level: u32,
    longest_survival_seconds: f32,
    total_damage_dealt: f64,
    total_currency_earned: u64,
    total_enemies_killed: u64,
    // etc
}
```

## Shop System (data-driven)
Items defined as const arrays with ID, name, cost, effect.
Categories: Characters, Permanent Upgrades, Cosmetics.

## Achievement System (condition-driven)
Achievements defined with an ID, name, description, and condition function.
Checked at various game events (kill, levelup, death, wave complete).
Unlock awards currency.

## Game Flow
1. Menu → Start → Character Select → Playing
2. Playing → (level up → upgrade choice) → Playing
3. Playing → Game Over → Show stats, currency earned → Shop → Menu
4. Menu → Shop → Buy items with Souls → Menu
