# Changelog

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
