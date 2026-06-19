---
applyTo: '**'
---
# ReapTide — GitHub Copilot Instructions

---

## Project Overview

**ReapTide** is a top-down 2D action roguelite built in Rust with the Bevy 0.18.1 game engine. The game merges Vampire Survivors' horde-survival loop with Hades' movement/combat depth. The player controls a character that fights waves of enemies, collects XP, levels up, and unlocks permanent upgrades across runs.

**Tagline:** *Reap souls. Survive the tides.*

**Rendering model:** Currently renders with Bevy's 2D pipeline (Mesh2d + ColorMaterial + orthographic camera). Movement is on the XY plane; Z=0 for all entities. **3D isometric conversion is planned** (see T0.0 in docs/tasks.md).

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (edition 2021) |
| Engine | Bevy 0.18.1 |
| Serialization | serde 1 + serde_json |
| RNG | rand 0.10.1 |
| Planned (commented in Cargo.toml) | avian3d (physics), bevy_hanabi (particles), bevy_tweening (animation), bevy_gltf (assets), bevy_replicon (networking) |

### Bevy 0.18 Key APIs in Use
- **Rendering:** `Mesh2d`, `ColorMaterial`, `MeshMaterial2d`, `Camera` + `OrthographicProjection`
- **ECS:** `Query`, `Res`/`ResMut`, `Commands`, `EventWriter`/`EventReader`, `Component`, `Resource`
- **States:** `AppState` (Booting, Loading, MainMenu, Playing, GameOver), `GameOverlayState` — flat state + sub-state pattern
- **UI:** New Bevy 0.18 UI API with `Text::new()`, `TextFont`, `TextColor`, `Node`, `BorderColor`, `ChildBuildContext`
- **Audio:** `AudioPlayer` + `AudioSource` for procedural WAV playback
- **Input:** `ButtonInput<KeyCode>`, `EventReader<MouseWheel>`
- **Time:** `Time` resource with delta-based tick

### Planned Upgrades (in priority order)
1. 3D isometric rendering (Mesh3d + StandardMaterial, XZ movement, isometric camera)
2. Avian3D physics (XZ-constrained collision, knockback)
3. bevy_hanabi GPU particles
4. bevy_tweening animation/tweening
5. Map generation (arena layouts with biomes)
6. Post-processing (bloom, tonemapping)
7. Custom shaders via wgsl
8. Networking via bevy_replicon

---

## Coding Guidelines

### Rust Conventions
- **Edition:** 2021
- **Format:** `cargo fmt` — standard rustfmt. No tabs, 4-space indentation.
- **Clippy:** All warnings must be clean. Run `cargo clippy -- -D warnings` before commits.
- **No `unwrap()` in release paths** — use `if let Ok(x) = ... { }` or proper error handling. Zero tolerance for panics in game logic.
- **No dead code** — mark intentionally unused items with `#[allow(dead_code)]` and a comment explaining why.
- **Documentation:** Every public item gets a doc comment (`///`). Internal items get inline comments (`// ── Section ──` style section headers).
- **Imports:** Grouped as: standard library → external crates → `crate::` modules, separated by blank lines.
- **Constants:** Use `SCREAMING_SNAKE_CASE`. All game balance values in `src/constants.rs`.
- **Types:** Prefer `f32` for all game math. Use `u32`/`u64` for discrete counts (XP, kills, level).

### Bevy ECS Conventions
- **Plugin isolation:** Every module exports a `Plugin` struct. Systems are registered in the plugin's `build()` method.
- **Event-driven:** Cross-module communication uses Bevy events, never direct system calls. Fire events with `EventWriter`, listen with `EventReader`.
- **ECS-first:** Data in `Component`s, logic in systems, cross-cutting state in `Resource`s. No singletons, no globals.
- **No monolithic files:** Each module gets its own directory. Systems, components, resources, and plugins are in separate files within the module.
- **State machine:** Use `OnEnter`/`OnExit` for state transitions. Overlays use `GameOverlayState` sub-state, not additions to `AppState`.
- **Modifier stack:** All computed stats use `StatInstance` + `ModifierStack`. Never multiply or add to stats directly. Use `StatBundle::add_modifier()` with named source IDs.
- **Systems:** One clear responsibility per system. Prefer `Query` filters (`With<X>`, `Without<Y>`, `Changed<Z>`) over manual checks.
- **Resources:** Register at plugin build time. Use `#[derive(Resource)]` and `Default` where possible.

### Project-Specific Conventions
- **Character data** lives in `CharacterDef` structs in `src/constants.rs`. Adding a character = adding one struct entry.
- **Enemy definitions** are data-driven. Enemy kinds are in `EnemyKind` enum. Stats are computed from base constants + wave scaling.
- **Stats system** (`src/gameplay/stats/`): `StatDefinitions` resource holds all 24 stat definitions. `StatBundle` component on entities holds computed values. `ModifierType` supports `Flat`, `PercentAdd`, `PercentMult`, `Set`.
- **Enemy AI** uses the `EnemyBrain` FSM component (`src/gameplay/enemies/brain.rs`) with 9 states: Idle, Patrol, Alert, Chase, Attack, Flee, Stunned, Special, Dead.
- **File structure:** Each file = single responsibility. Systems, components, resources, and plugins are never mixed in one file.

---

## Project Structure

```
AGENTS.md                          ← Agent coordination guide (read first)
CHANGELOG.md                       ← All changes versioned
docs/
  SPEC.md                          ← Architecture & specification (always read)
  design.md                        ← Design decisions (read when implementing)
  requirements.md                  ← Functional/NFR requirements
  tasks.md                         ← Sprint plan & task breakdown (read every session)
src/
  main.rs                          ← App entry, DefaultPlugins + ReapTidePlugins
  constants.rs                     ← ALL game balance values, character/enemy/shop data
  core/
    plugin.rs                      ← ReapTidePlugins group (10 plugins in dependency order)
    state.rs                       ← AppState + GameOverlayState
    event.rs                       ← DamageEvent, KillEvent, etc.
  gameplay/
    mod.rs                         ← GameplayPlugin, setup_gameplay startup system
    components.rs                  ← Player, Enemy, Projectile, XpGem, WaveSpawner, etc.
    player.rs                      ← spawn_player, player_movement
    enemies/
      mod.rs                       ← Enemy spawning + chase AI
      brain.rs                     ← EnemyBrain FSM (9 states, perception, memory)
    combat.rs                      ← Auto-attack, projectile update, contact damage, enemy death
    xp.rs                          ← XP magnet, collection, leveling
    upgrades.rs                    ← Weapon upgrade choices
    particles.rs                   ← Simple transform-based particles (placeholder)
    camera.rs                      ← Follow, zoom, screen shake, dark overlay
    resources.rs                   ← GameMeshes, GameMaterials, GameStats, ScreenShake, etc.
    ui.rs                          ← HUD, pause menu, controls overlay, game over screen
    stats/
      mod.rs                       ← StatPlugin + cleanup_expired_modifiers
      definitions.rs               ← StatDefinitions (24 stats), create_bundle()
      modifier.rs                  ← StatInstance, StatModifier, ModifierType, StatBundle
  ui/
    mod.rs, screen.rs, components.rs, theme.rs, menu.rs, hud.rs
  audio/
    mod.rs, sfx.rs
  characters/
    mod.rs, registry.rs
  achievements/
    mod.rs, registry.rs, checker.rs
  shop/
    mod.rs, catalog.rs, purchases.rs
  save/
    mod.rs, profile.rs
  assets/
    mod.rs, loading.rs
```

---

## Cargo Commands & Tools

All cargo components are installed globally.

### Build & Check
| Command | Purpose |
|---------|---------|
| `cargo check` | Fast compilation check (no binary output) |
| `cargo build` | Full debug build |
| `cargo build --release` | Optimised release build |
| `cargo doc --open` | Build and open API docs |

### Linting & Quality
| Command | Purpose |
|---------|---------|
| `cargo clippy -- -D warnings` | Lint with warnings-as-errors |
| `cargo fmt` | Format all code |
| `cargo fmt --check` | Check formatting without changing |
| `typos` | Check for typos in code/docs |

### Testing
| Command | Purpose |
|---------|---------|
| `cargo test` | Run all tests |
| `cargo test test_name` | Run specific test |
| `cargo test -- --nocapture` | Run with stdout/stderr visible |

### Dependency Management
| Command | Purpose |
|---------|---------|
| `cargo add <crate>` | Add dependency (cargo-edit) |
| `cargo rm <crate>` | Remove dependency |
| `cargo upgrade` | Upgrade dependencies (cargo-edit) |
| `cargo outdated` | Check for outdated deps |
| `cargo audit` | Check for security vulnerabilities |

### Analysis
| Command | Purpose |
|---------|---------|
| `cargo expand` | Expand macros (cargo-expand) |
| `cargo watch -x check` | Auto-check on file changes (cargo-watch) |
| `cargo generate` | Use project templates (cargo-generate) |

### VS Code
- **Rust-Analyzer** provides live diagnostics, completions, and type info. Trust its feedback over running `cargo check` manually.
- Open files to trigger analysis. Use `get_errors` for targeted diagnostics on specific files.
- Do NOT run `cargo check` unless explicitly asked — Rust-Analyzer is the primary diagnostic tool.

---

## Do Not

- Do NOT run `cargo check` unless explicitly asked — use Rust-Analyzer diagnostics instead.
- Do NOT use `unwrap()` in game systems or release paths.
- Do NOT create monolithic files. Each file has one responsibility.
- Do NOT add magic numbers to system code. Constants go in `src/constants.rs`.
- Do NOT modify stat values inline — always use `StatBundle::add_modifier()`.
- Do NOT call another module's systems directly — fire events instead.
- Do NOT write Python scripts or use `execute_code` — use the Rust toolchain.
- Do NOT commit with clippy warnings or `cargo fmt` violations.
- Do NOT use `#[path]` directives — modules resolve by directory structure.
- Do NOT forget to read AGENTS.md at the start of every session.  
