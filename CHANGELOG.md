# Changelog

## [0.3.0] — 2026-06-19

### Added
- T2.1 — EnemyBrain FSM component with 9-state AI state machine (AiState enum)
- Perception system (sight_range, hearing_range, memory_duration) per enemy
- EnemyMemory tracking (last known position, forget logic, alert cooldown)
- Timer-driven state transition system (update_enemy_brain) with 10 transition rules
- 8 unit tests covering state defaults, transitions, stun, memory, and perception
- enemies module converted to directory module (enemies/mod.rs + enemies/brain.rs)
- EnemyBrain automatically attached to all spawned enemies via spawn_enemies
- enemies_chase now respects brain state (only moves if aggressive)

## [0.2.0] — 2026-06-19

### Fixed
- Bevy 0.18 API migration: `BorderColor::all()` → `BorderColor()` across 3 files (ui, menu, shop)
- Bevy 0.18 Text API: `text.0` → `text.sections[0].value` in gameplay/ui.rs
- Missing `rand::Rng` imports in camera.rs and combat.rs
- Missing `Default` derive on `UpgradeState` resource (needed for `init_resource`)
- HUD not being spawned on game start (setup_gameplay now calls spawn_hud)
- Removed duplicate `spawn_hud` from ui/hud.rs (unused dead code)
- Removed unused `GamePlugin` trait from core/plugin.rs
- Upgraded all `ChildBuilder` → `ChildSpawner` for Bevy 0.18 builder pattern

### Changed
- Updated docs (SPEC.md, design.md, requirements.md, tasks.md) to reflect current status

## [0.1.0] — 2026-06-17

### Added
- Project scaffolding with Bevy 0.18
- Modular architecture: core/, gameplay/, ui/, audio/, characters/, save/, achievements/, shop/
- Hierarchical state machine (stack-based with push/pop)
- Plugin-based module system
- Event bus for cross-module communication
- Character registry with Reaper, Harbinger, Shade, Wraith
- Enemy AI framework (state-machine brain with Chase/Attack/Flee/PhaseShift)
- Combat system with typed damage, knockback, status effects
- Procedural map generation with tile grid
- Audio system with procedural SFX generation
- UI screen stack with reusable components
- Save system with multi-slot JSON profiles
- Achievement registry with conditions and rewards
- Shop with permanent upgrades and character unlocks
- In-game HUD, pause menu, controls overlay, game over screen
- Camera follow, zoom, screen shake
- XP magnet, leveling, weapon evolution upgrades

### Changed
- Complete rewrite from initial prototype
- Structured from single file to 50+ focused modules
