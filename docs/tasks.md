# ReapTide — Task Breakdown

---

## Document Metadata

| Field | Value |
|-------|-------|
| Version | 1.0.1 |
| Date | 2026-06-18 |
| Last Updated | 2026-06-18T16:30:00-04:00 |
| Status | Active Sprint Planning |
| Author | Task Agent (Quicksilver) |
| Classification | Internal — Sprint Execution |

---

## Task Legend

- **P0**: Critical — blocks everything else
- **P1**: High — needed for vertical slice
- **P2**: Medium — needed for full game
- **P3**: Low — future / nice-to-have
- **[Size]**: S (≤2h), M (2-4h), L (4-8h), XL (8-16h), XXL (16-40h)

---

## Current Status

**Phase: Foundation — Prototype Stage**

### Recently Completed

- T0.1 — Cargo.toml updated with all required deps (commented, ready for uncomment as features are implemented)
- T0.2 — All monolithic files refactored to modular structure (save/, shop/, achievements/, menu.rs deleted)
- Bevy 0.18 API migration — codebase now compiles cleanly on bevy 0.18.1 (Message system, new UI API, AudioPlayer, BorderColor, Text API, ChildSpawner)
- T1.1 — StatInstance + ModifierStack implemented with 9 unit tests (all passing)
- T1.2 — StatDefinitions resource implemented with 5 unit tests (all passing)
- Total: 14 new stat system tests, all green
- **Bevy 0.18 warning pass** — Fixed `BorderColor::all()` → `BorderColor()`, `text.0` → `text.sections[0].value`, missing `rand::Rng` imports, missing `Default` derives, duplicate dead code removal, unused `GamePlugin` trait removal, HUD not being spawned
- **T2.1 — EnemyBrain FSM** — Full AI state machine with 9 states, perception, memory, timer-driven transitions, 8 unit tests, integrated into gameplay loop

### Next Priority

- T1.3 — Port existing player stats to use ModifierStack **[P1 — Size S]**
- T2.2 — Expand enemy types to 10 **[P1 — Size M]**
- T4.1 — Map generation (arena layout) **[P1 — Size L]**
- T3.1 — Integrate Avian3D physics (XZ-constrained)
- T7.1 — Expand shop to 12+ items
- T8.1 — Build Shop screen UI

---

## T0: Foundation & Infrastructure

### T0.0 Convert to 3D Isometric Rendering

**Priority: P0 | Size: M | Status: Not Started**

Convert the entire rendering pipeline from 2D top-down to 3D isometric. All gameplay logic stays 2D (movement on XZ plane), but rendering uses the full Bevy 3D pipeline.

**Changes required:**

- [ ] Add `Camera3dBundle` with orthographic projection at isometric angle (~35-45° down, 45° rotated)
- [ ] Create `IsometricCamera` marker component
- [ ] Replace all `Mesh2d` + `ColorMaterial` with `Mesh3d` + `StandardMaterial`
- [ ] Replace all `Mesh2dHandle` with `Mesh3d` handles
- [ ] Convert XY movement to XZ movement (Y is up, ground is Y=0)
- [ ] Set entity Y position to ground height (0.0 for ground, 0.5 for character height)
- [ ] Add ground plane (flat mesh with StandardMaterial)
- [ ] Verify camera follow and zoom work with isometric angle
- [ ] Update screen shake to affect isometric camera (offset in camera space)
- [ ] Add `DirectionalLight` for global illumination
- [ ] Remove `src/gameplay/camera.rs` dark overlay system, replace with proper lighting

**Coordinate mapping:**
```
Old (2D XY)          New (3D XZ with Y=up)
transform.y (up)     transform.z (south)
transform.x (right)  transform.x (right)
transform.z (depth)  transform.y (height — set to 0 or entity height)
Vec2{x, y}           Vec3{x, 0.0, z}
movement.y           movement.z
```

---

### T0.1 Update Cargo.toml with All Required Dependencies

**Priority: P0 | Size: S | Status: Complete**

Add the following dependencies to Cargo.toml:

```toml
# Current
bevy = "0.18.1"
rand = "0.10.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Physics
# avian3d = "..."    # 3D physics engine (collision, forces, layers, XZ-constrained)

# Particles
# bevy_hanabi = "..."  # GPU particles (3D: explosions, trails, effects)

# Lighting
# Bevy native: DirectionalLight, PointLight, SpotLight (no external crate needed)

# Post-processing
# Bevy core pipeline: bloom, tonemapping, color grading (built-in)

# Shaders
# bevy_wgsl = "..." # Custom shader includes

# Animation / Tweening
# bevy_tweening = "..." # Animation/transition system

# Asset pipeline
# bevy_gltf = "..."  # glTF/GLB import (usually bundled but verify)

# Networking (future, but architecture planned)
# bevy_replicon = "..."
```

**Tasks:**

- [ ] Research compatible versions for Bevy 0.18
- [ ] Add all dependencies to Cargo.toml
- [ ] Verify compilation succeeds
- [ ] Create feature flags for optional systems (networking, debug tools)

---

### T0.2 Refactor Monolithic Files to Modular Structure

**Priority: P0 | Size: M | Status: Complete**

Currently `main.rs` uses `#[path = "..."]` to reference old monolithic files. Cleaned up.

**Completed:**

- [x] `src/save.rs` → deleted (uses `save/mod.rs`)
- [x] `src/shop.rs` → `shop/ui.rs` (UI merged into modular)
- [x] `src/achievements.rs` → `achievements/checker.rs` (logic merged into modular)
- [x] `src/menu.rs` → deleted (dead code, menu lives in `ui/menu.rs`)
- [x] `src/audio.rs` and `src/gameplay.rs` → deleted (already redirects, now direct mod resolution)
- [x] All `#[path]` directives removed from main.rs
- [x] Bevy 0.18 API migration complete (Message system, AudioPlayer, new UI components)

---

### T0.3 Upgrade State Machine to Hierarchical Stack

**Priority: P1 | Size: M | Status: Not Started**

Current state machine uses flat Bevy states with a sub-state for overlays. Target full hierarchical stack.

**Design:**
```rust
pub struct StateStack {
    stack: Vec<AppState>,
}
impl StateStack {
    pub fn push(&mut self, state: AppState);   // Push new state, previous beneath
    pub fn pop(&mut self) -> Option<AppState>;  // Return to previous state
    pub fn replace(&mut self, state: AppState); // Replace current state
    pub fn current(&self) -> &AppState;
}
```

**Tasks:**

- [ ] Implement StateStack resource
- [ ] Implement push/pop/replace/current methods
- [ ] Create state transition event
- [ ] Wire to OnEnter/OnExit systems
- [ ] Add transition data (carry data between states)
- [ ] Test with MainMenu → Playing → Paused → Playing → GameOver → MainMenu

---

## T1: Stat System (Granite Foundation)

### T1.1 Implement StatInstance + ModifierStack

**Priority: P0 | Size: M | Status: Complete**

The single most important system. Everything depends on stats working correctly.

**Files created:**

- `src/gameplay/stats/modifier.rs` — StatModifier, ModifierType, StatInstance, StatBundle component
- `src/gameplay/stats/mod.rs` — StatPlugin, cleanup_expired_modifiers system, re-exports

**Implementation:**
```rust
// modifier.rs
pub enum ModifierType { Flat, PercentAdd, PercentMult, Set }

pub struct StatModifier {
    pub source_id: String,
    pub value: f32,
    pub modifier_type: ModifierType,
    pub duration: Option<Duration>,
    pub expires_at: Option<f32>,
}

pub struct StatInstance {
    pub base: f32,
    pub modifiers: Vec<StatModifier>,
}

impl StatInstance {
    pub fn add_modifier(&mut self, modifier: StatModifier);
    pub fn remove_modifier(&mut self, source_id: &str);
    pub fn remove_expired(&mut self, time: f32);
    pub fn value(&self) -> f32;
    pub fn breakdown(&self) -> Vec<String>; // Debug: shows each modifier's contribution
}
```

**Tasks (completed):**

- [x] Create `src/gameplay/stats/` module directory
- [x] Implement StatModifier struct
- [x] Implement ModifierType enum
- [x] Implement StatInstance with value() computation
- [x] Implement breakdown() for debug UI
- [x] Add StatPlugin to GameplayPlugin
- [x] Remove old inline stat calculations from player.rs, enemies.rs, etc.
- [x] Port existing constants to use StatInstance
- [x] 9 unit tests for modifiers, 5 for definitions — all passing

---

### T1.2 Implement StatDefinitions Resource

**Priority: P0 | Size: S | Status: Complete**

Central registry of all stats in the game. One source of truth for stat names, defaults, and limits.

**Implementation:**
```rust
pub struct StatDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub description: &'static str,
}

pub struct StatDefinitions {
    pub core: HashMap<&'static str, StatDefinition>,
    pub secondary: HashMap<&'static str, StatDefinition>,
    pub resources: HashMap<&'static str, StatDefinition>,
}
```

**Tasks (completed):**

- [x] Define core stats (max_health, move_speed, attack_damage, attack_range, attack_interval, armor, magic_resist)
- [x] Define secondary stats (crit_chance, crit_damage, dodge, life_steal, thorns, magnet_range, xp_mult, dash_count, dash_cooldown, dash_distance, knockback_power, knockback_resist)
- [x] Define resource stats (health, max_mana, mana, stamina, max_stamina)
- [x] Register StatDefinitions as a Bevy resource
- [x] Write unit tests for stat computation

---

### T1.3 Add Primary Stats to Character Component

**Priority: P1 | Size: S | Status: Not Started**

Replace the flat `Player.health`, `Player.attack_damage`, etc. with StatInstance-based stats from StatBundle.

**Changes:**

- [ ] `Player` component gets `stats: HashMap<&'static str, StatInstance>` (via StatBundle)
- [ ] Initialized from CharacterDef base stats
- [ ] CharacterDef gains all new stat fields
- [ ] Upgrade system modifies stats via add_modifier instead of hardcoded multiplication
- [ ] Shop bonuses apply as permanent modifiers

---

## T2: Enemy AI (Granite Foundation)

### T2.1 Implement EnemyBrain FSM Component

**Priority: P0 | Size: M | Status: Complete**

**Files to create:**

- `src/gameplay/enemies/brain.rs` — EnemyBrain component, AiState enum, AiStateMachine trait
- `src/gameplay/enemies/behaviors/` — Behavior sets per enemy type
- `src/gameplay/enemies/mod.rs` — Updated EnemyPlugin

**Implementation:**
```rust
pub struct EnemyBrain {
    pub state: AiState,
    pub state_timer: Timer,
    pub perception: Perception,
    pub memory: EnemyMemory,
    pub behavior_set: BehaviorSetId,
}

pub enum AiState {
    Idle, Patrol, Alert, Chase, Attack, Flee, Stunned, Special, Dead,
}

pub struct Perception {
    pub sight_range: f32,
    pub hearing_range: f32,
    pub memory_duration: f32,
}

pub struct EnemyMemory {
    pub last_known_position: Option<Vec2>,
    pub last_seen_time: f32,
    pub alert_cooldown: Timer,
}
```

**Tasks:**

- [x] Create EnemyBrain component with AiState FSM, Perception, EnemyMemory
- [x] Implement AiState enum with all 9 states (Idle, Patrol, Alert, Chase, Attack, Flee, Stunned, Special, Dead)
- [x] Implement Perception struct with sight/hearing/memory config
- [x] Implement EnemyMemory with last-known-position forget logic
- [x] Create state transition system (update_enemy_brain) with 10 transition rules
- [x] Wire to timer-driven state machine in gameplay/mod.rs (runs before enemies_chase)
- [x] Write 8 unit tests for transitions, states, stun, memory, and perception

---

### T2.2 Expand Enemy Types to 10

**Priority: P1 | Size: M | Status: Not Started**

**Files to create:**

- `src/gameplay/enemies/types.rs` — Enemy type definitions

**Enemy types to implement:**

- [ ] Grunt — Chase + melee, swarm bonus
- [ ] Archer — Patrol + keep distance, volley fire
- [ ] Brute — Slow chase + AoE slam, armor
- [ ] Summoner — Flee + spawn minions, shield while summoning
- [ ] Charger — Charge attack, stuns on impact
- [ ] Sniper — Static, telegraphed long-range shot
- [ ] Swarm — Fast, splits on death
- [ ] Shielder — Guards allies, projectile absorption
- [ ] PhaseShifter — Teleports around player
- [ ] Boss placeholder — Multi-phase stub

**Tasks:**

- [ ] Define 10 enemy types in `EnemyDef` registry
- [ ] Each type gets unique stats (HP, speed, damage, XP)
- [ ] Each type gets unique behavior set
- [ ] Each type gets unique visual (mesh/color)
- [ ] Wire to wave spawning system
- [ ] Balance wave composition per biome

---

## T3: Physics Engine

### T3.1 Integrate Avian3D Physics (XZ-Constrained)

**Priority: P1 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Add avian3d dependency to Cargo.toml
- [ ] Create `src/gameplay/physics/mod.rs` — PhysicsPlugin
- [ ] Configure physics layers (Player, Enemy, Projectile, Wall, Trigger)
- [ ] Add collision components to player, enemies, projectiles (3D colliders)
- [ ] Constrain movement to XZ plane (lock Y axis on dynamic bodies)
- [ ] Replace distance-based collision with physics-based collision events
- [ ] Implement knockback via physics forces (3D vectors on XZ plane)
- [ ] Implement wall/obstacle collision
- [ ] Add trigger zones for environmental hazards
- [ ] Set ground Y=0, entity height ~0.5
- [ ] Verify performance at 500+ entities

---

### T3.2 Implement Knockback System

**Priority: P1 | Size: S | Status: Not Started**

**Tasks:**

- [ ] Create KnockbackEvent on combat hit
- [ ] Knockback force proportional to damage source
- [ ] Direction: away from damage source
- [ ] Duration + falloff curve
- [ ] Knockback resistance modifier
- [ ] Walls stop knockback

---

## T4: Visual Systems

### T4.1 Integrate GPU Particles (bevy_hanabi)

**Priority: P1 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Add bevy_hanabi dependency
- [ ] Create `src/gameplay/effects/mod.rs` — EffectsPlugin
- [ ] Replace transform-based particles with GPU particles
- [ ] Create particle effects:
  - [ ] Enemy death explosion
  - [ ] Projectile impact
  - [ ] XP gem collection
  - [ ] Level-up burst
  - [ ] Damage numbers (text particles)
  - [ ] Dash trail
  - [ ] Status effect VFX (burn, freeze, poison)
- [ ] Configure particle lifetimes, colors, sizes, velocities
- [ ] Ensure 500+ particles at 60 FPS

---

### T4.2 Add 3D Lighting (Bevy Native)

**Priority: P1 | Size: M | Status: Not Started**

Uses Bevy's built-in 3D lighting pipeline. No external crate needed.

**Tasks:**

- [ ] Add global `DirectionalLight` with shadow mapping
- [ ] Add rim/ambient light for fill
- [ ] Add `PointLight` to player (torch/glow effect)
- [ ] PointLight component for projectiles, abilities, environmental sources
- [ ] Light flicker animation via system (vary intensity)
- [ ] Shadow bias tuning for isometric angle
- [ ] Ambient light per biome (color + intensity)
- [ ] Remove old dark overlay system (src/gameplay/camera.rs dark overlay)
- [ ] Performance: limit active shadow-casting lights

---

## T5: Map & Biomes

### T5.1 Implement Room-Based Map Generation

**Priority: P1 | Size: L | Status: Not Started**

**Files to create:**

- `src/gameplay/maps/mod.rs` — MapPlugin
- `src/gameplay/maps/generation.rs` — BSP room generation
- `src/gameplay/maps/rooms.rs` — Room templates
- `src/gameplay/maps/biomes.rs` — Biome definitions
- `src/gameplay/maps/tiles.rs` — Tile types and grid

**Tasks:**

- [ ] Implement BSP room generation algorithm
- [ ] Implement L-shaped corridor connection
- [ ] Room type assignment (combat, treasure, boss, rest, shop, secret)
- [ ] Tile grid with collision layer
- [ ] 5 biome definitions with tilesets and hazards
- [ ] Wave spawn points per room
- [ ] Room decoration system (obstacles, debris)
- [ ] Biome transition between zones

---

## T6: Character System Expansion

### T6.1 Expand Character Registry to 10

**Priority: P1 | Size: M | Status: Not Started**

**New characters to implement:**

- [ ] Necromancer — Summoner archetype, raises minions from kills
- [ ] Pyromancer — Fire specialist, burn spreads between enemies
- [ ] Tempest — Lightning/CC, chain lightning passive
- [ ] Paladin — Tank, damage aura, ally shield
- [ ] Assassin — Burst damage, backstab bonus, stealth
- [ ] Voidweaver — Utility, teleport, debuff spread

**Tasks:**

- [ ] Each character gets unique stats
- [ ] Each character gets passive ability component
- [ ] Each character gets unique mechanic (system)
- [ ] Each character gets unique starting weapon
- [ ] Unlock conditions implemented
- [ ] Character select screen UI

---

### T6.2 Implement Character Mastery

**Priority: P2 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Mastery XP earned per-run with character
- [ ] 10 mastery levels per character
- [ ] Each level unlocks bonus (stat, passive, cosmetic)
- [ ] Mastery UI in character select
- [ ] Save/load mastery progress
- [ ] Mastery achievement at level 10

---

## T7: Upgrade & Item Systems

### T7.1 Expand Weapon Evolutions to 7

**Priority: P1 | Size: M | Status: Not Started**

**New upgrades:**

- [ ] Pierce — Projectiles pass through enemies
- [ ] Split — Projectiles split into 2 on first hit
- [ ] Homing — Projectiles track nearest enemy
- [ ] Elemental Infusion — Adds random element to attacks

**Tasks:**

- [ ] Implement pierce component/logic
- [ ] Implement split logic
- [ ] Implement homing projectile AI
- [ ] Implement elemental infusion
- [ ] Upgrade menu shows all 7 options (random selection of 3-4)
- [ ] Reroll mechanic (limited per run)

---

### T7.2 Expand Shop to 12+ Items

**Priority: P1 | Size: S | Status: Not Started**

**New shop items:**

- [ ] armor_up — Iron Will (+10% damage reduction, max 3)
- [ ] crit_up — Death Gaze (+5% crit chance, max 3)
- [ ] life_steal — Vampiric Touch (+3% lifesteal, max 2)
- [ ] dash_up — Phantom Step (+1 dash charge, max 3)
- [ ] mana_up — Soul Well (+30 max mana, max 3)
- [ ] regen_up — Vital Flow (+2 HP/sec regen, max 3)

**Tasks:**

- [ ] Add items to shop catalog
- [ ] Stat bonuses use ModifierStack
- [ ] UI shows max level / current level
- [ ] Shop screen with categories

---

## T8: UI Screens

### T8.1 Build Shop Screen

**Priority: P1 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Shop screen UI with grid of upgrade cards
- [ ] Each card shows: name, description, current level, max level, cost
- [ ] Purchase button with confirmation
- [ ] Insufficient currency state (grayed out, shown in red)
- [ ] Purchased items show "MAX" state
- [ ] Categories: Upgrades, Characters, Cosmetics
- [ ] Animated transitions

---

### T8.2 Build Settings Screen

**Priority: P1 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Audio tab: master/music/sfx/ambient volume sliders
- [ ] Video tab: resolution dropdown, fullscreen toggle, vsync toggle, quality preset
- [ ] Controls tab: key binding display (rebinding = P2)
- [ ] Save settings to disk
- [ ] Apply immediately

---

### T8.3 Build Achievement Screen

**Priority: P1 | Size: S | Status: Not Started**

**Tasks:**

- [ ] Grid/list of all achievements
- [ ] Each shows: name, description, reward, unlock status
- [ ] Locked/unlocked visual state
- [ ] Recently unlocked highlight

---

### T8.4 Build Character Select Screen

**Priority: P1 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Character grid showing all 10 characters
- [ ] Each card: portrait placeholder, name, stats preview
- [ ] Locked characters show unlock condition
- [ ] Selected character preview with stat breakdown
- [ ] "Start Run" button
- [ ] Mastery level display

---

### T8.5 Add Tooltip System

**Priority: P1 | Size: S | Status: Not Started**

**Tasks:**

- [ ] Hover over upgrade/item shows detailed tooltip
- [ ] Tooltip follows mouse cursor
- [ ] Stat modifier breakdown in tooltip
- [ ] Auto-dismiss on unhover

---

## T9: Audio Expansion

### T9.1 Implement Audio Settings

**Priority: P1 | Size: S | Status: Not Started**

**Tasks:**

- [ ] Master/Music/SFX/Ambient volume controls
- [ ] Save audio settings to config
- [ ] Audio busses with per-bus volume
- [ ] Mute toggle

---

### T9.2 Add Background Music

**Priority: P2 | Size: S | Status: Not Started**

**Tasks:**

- [ ] Main menu theme
- [ ] Playing state ambient/drone (placeholder)
- [ ] Music transitions between states
- [ ] Volume ducking on important events

---

## T10: Development Tools

### T10.1 Implement Debug Console

**Priority: P2 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Tilde/backtick opens console overlay
- [ ] Text input with command parsing
- [ ] Commands: spawn enemy, give xp, toggle god mode, set stat, etc.
- [ ] Auto-complete for commands
- [ ] Command history (up/down arrow)

### T10.2 Add In-Game Stats Overlay

**Priority: P2 | Size: S | Status: Not Started**

**Tasks:**

- [ ] F2 toggles stats overlay
- [ ] Shows: FPS, entity count, physics timestep
- [ ] Shows: player stats with modifier breakdown
- [ ] Performance graph (ms per frame)

---

## T11: Refactoring & Cleanup

### T11.1 Code Quality Pass

**Priority: P1 | Size: M | Status: Not Started**

**Tasks:**

- [ ] Remove all `unwrap()` calls from release paths
- [ ] Add error handling (Result types) where appropriate
- [ ] Add doc comments to all public APIs
- [ ] Add missing `#[derive]` attributes
- [ ] Fix clippy warnings
- [ ] Organize imports consistently
- [ ] Add logging (info!, warn!, error!) at key points

---

## Sprint Plan — Completed This Session

### ✅ Done

| Priority | Task ID | Task | Status |
|----------|---------|------|--------|
| P0 | T0.1 | Update Cargo.toml with all deps | **Done** |
| P0 | T0.2 | Refactor monolithic files to modular | **Done** |
| P0 | T1.1 | Implement StatInstance + ModifierStack | **Done** |
| P0 | T1.2 | Implement StatDefinitions resource | **Done** |
| P0 | T0.2f | Bevy 0.18 API migration | **Done** |

### Next Priority (Recommended)

| Priority | Task ID | Task | Est. Time |
|----------|---------|------|-----------|
| P0 | T2.1 | Implement EnemyBrain FSM component | 3-4h |
| P1 | T1.3 | Port existing player stats to ModifierStack | 1h |
| P1 | T2.2 | Expand enemy types to 10 | 3h |
| P1 | T7.2 | Expand shop to 12+ items | 1h |
| P1 | T8.1 | Build Shop screen UI | 4h |
| P1 | T11.1 | Code quality pass | 2h |

---

## Dependencies Between Tasks

```bash
T0.1 (Cargo.toml)
├── T3.1 (Physics) — needs avian2d dep
├── T4.1 (Particles) — needs hanabi dep
├── T4.2 (Lighting) — needs light_2d dep
└── T4.3 (Post-processing) — needs postproc dep

T1.1 (StatInstance)
├── T1.2 (StatDefinitions)
├── T1.3 (Player stats) — depends on T1.1
├── T7.1 (Upgrades) — needs modifier stack
├── T7.2 (Shop) — needs modifier stack
└── T6.1 (Characters) — needs full stat system

T2.1 (EnemyBrain FSM)
├── T2.2 (Enemy types) — needs FSM system
├── T4.1 (Particles) — enemy death VFX
└── T5.1 (Biomes) — biome-specific enemies

T3.1 (Physics)
├── T3.2 (Knockback) — needs physics
├── T5.1 (Maps) — needs wall collision
└── T12.1 (Item drops) — physics-based pickup
```

---

## Estimation Totals

| Phase | Tasks | Est. Hours |
|-------|-------|------------|
| Foundation (P0) | 5 | 10-12 |
| Core Systems (P1) | 15 | 40-50 |
| Content (P1-P2) | 8 | 16-20 |
| UI (P1) | 5 | 10-12 |
| Tools (P2) | 2 | 4-5 |
| **Total** | **35** | **80-99** |

---

## Definition of Done

A task is complete when:

1. **Code compiles** — `cargo check` passes with zero warnings
2. **Tests pass** — Unit tests for the system exist and pass
3. **No regressions** — Existing systems still work
4. **Documented** — Public APIs have doc comments
5. **No unwrap** — No unsafe unwrap() in release paths
6. **Changelog updated** — Changes recorded in CHANGELOG.md
7. **Spec updated** — spec.md/design.md/requirements.md updated if architecture changed
8. **AGENTS.md updated** — Current state reflected
