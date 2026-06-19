# ReapTide — Architecture & Specification

---

## Document Metadata

| Field | Value |
|-------|-------|
| Version | 2.0.0 |
| Date | 2026-06-18 |
| Last Updated | 2026-06-18T11:30:00-04:00 |
| Status | Draft — Active Development |
| Author | Architectural Agent (Quicksilver) |
| Classification | Internal — Project Foundation |

---

## Overview

ReapTide is a top-down 2D action roguelite merging Vampire Survivors' horde-survival loop with Hades' movement/combat depth. Built in Rust + Bevy 0.18. Targeting AAA-quality systems, modular architecture, and professional-grade codebase standards.

**Tagline:** *Reap souls. Survive the tides.*

### Design Pillars

1. **Depth over complexity** — Simple to pick up, impossible to master. Systems interact meaningfully without overwhelming.
2. **Player expression** — Build variety through character choice, weapon evolution, upgrade paths, and playstyle.
3. **Paced chaos** — Waves escalate in density and threat. Player power scales to match. The screen fills but never feels unfair.
4. **Permanent progression** — Each run earns currency. Shop unlocks, character unlocks, achievement rewards persist across runs.
5. **Data-driven design** — Characters, enemies, items, achievements, and upgrades defined in data, not hard-coded. Balance passes touch data tables, not systems.

---

## Rendering Model — 3D Isometric

ReapTide renders in 3D with an isometric orthographic camera. This gives us real PBR lighting, real shadows, depth-of-field, and full 3D asset pipeline — with 2D gameplay feel.

### Why 3D Isometric

1. **Bevy's primary pipeline is 3D.** The PBR renderer, lighting, shadows, and post-processing are all first-class. 2D is a secondary path with fewer features.
2. **Blender-first assets.** You build models once. They render with real lighting and shadows. No sprite baking, no angle-limited art.
3. **Future-proof.** Same codebase works for 2D isometric, 3D isometric, or full 3D third-person. The gameplay systems don't care.
4. **AAA visual quality.** Real-time global illumination, volumetric effects, PBR materials — without hacks.

### Camera Configuration

```rust
let camera_transform = Transform::from_xyz(0.0, ISO_HEIGHT, ISO_HEIGHT)
    .looking_at(Vec3::ZERO, Vec3::Y);

commands.spawn((
    Camera3d {
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            scaling_mode: ScalingMode::WindowSize(1.0),
            near: -1000.0,
            far: 1000.0,
            ..default()
        }),
        ..default()
    },
    Camera,
    Transform::from_xyz(0.0, 600.0, 600.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
));
```

- **Y is up** (3D standard). Ground plane is Y=0.
- **Movement plane**: XZ (player, enemies, projectiles move on XZ).
- **Camera angle**: ~35-45° tilt, 45° rotation from axes — classic isometric.
- **Projection**: Orthographic (no perspective distortion, true isometric).
- **Follow**: Camera target follows player on XZ plane, Y stays fixed.

### Coordinate Mapping

| 2D Top-Down | 3D Isometric |
|-------------|--------------|
| XY movement | XZ movement |
| Y = down | Y = up (Z replaces old Y for vertical movement) |
| Z order | Y position (height above ground) |
| Vec2 | Vec3 (Y set to 0 or entity height) |
| Transform.translation.x | Transform.translation.x |
| Transform.translation.y | Transform.translation.z |
| Transform.translation.z | Transform.translation.y |

The gameplay change is almost entirely mechanical — find-and-replace `.y` → `.z` for movement vectors, set `.y` to the ground height.

---

## Current Status

**Phase: Foundation — Prototype Stage** (Target: Playable vertical slice)

| Area | Status | Notes |
|------|--------|-------|
| Core state machine | Complete | Stack-based, hierarchical states with OnEnter/OnExit |
| Plugin framework | Complete | ReapTidePlugins group, dependency-ordered |
| Event bus | Complete | DamageEvent, KillEvent, XpPickupEvent, LevelUpEvent, etc. |
| Player movement | Complete | WASD/arrows, rotation towards movement |
| Auto-attack system | Complete | Nearest-target, timer-based, projectile spawn |
| Enemy wave spawning | Complete | 3 types (Small/Medium/Big), wave scaling |
| Enemy chase AI | Complete | Basic movement towards player |
| Combat + projectiles | Complete | Damage, projectile update, contact damage |
| XP + leveling | Complete | Magnet, collection, level-up with stat growth |
| Weapon upgrades | Complete | 3 choices: AttackSpeed, Damage, Range |
| XP gem particles | Basic | Burst on death, scale+fade |
| Camera system | Complete | Follow, zoom, screen shake |
| Audio | Basic | Procedural WAV generation, 6 sound effects |
| UI screens | Complete | Main menu, HUD, pause, game over, controls overlay |
| Fullscreen toggle | Complete | F11 |
| Save system | Complete | Multi-slot JSON, auto-save on milestones |
| Character registry | Complete | 4 characters (Reaper, Harbinger, Shade, Wraith) |
| Shop catalog | Complete | 6 permanent upgrades |
| Achievement registry | Complete | 7 achievements |
| Loading screen | Placeholder | Stub only |
| Shop UI | Not started | No in-game shop screen |
| Settings screen | Not started | No audio/video settings |
| Map generation | Spec-only | Described but not implemented |
| Boss enemies | Not started | Described in spec, not built |
| Map biomes | Not started | Not implemented |
| Advanced AI | Not started | Need finite state machine, perception, behavior sets |
| Physics engine | Not started | Need collision, knockback physics |
| 2D lighting | Not started | Need point lights, shadows |
| Particle system | Not started | Need GPU particles (Hanabi) |
| Post-processing | Not started | Need bloom, distortion |
| Shader system | Not started | Need custom shaders, post-processing |
| Networking | Not started | Future multiplayer backbone |
| Expanded stats | **Complete** | StatInstance + ModifierStack + StatDefinitions — 24 stat definitions, 14 tests |
| Animation system | Not started | Need sprite/animation pipeline |
| Equipment/items | Not started | Need loot, inventory, equippable system |
| Crafting | Not started | Need material drops, recipes |
| Localization | Not started | Need i18n framework |
| Debug console | Not started | Need developer tools |

---

## Architecture

```
src/
  main.rs                    → App entry, plugin registration
  constants.rs               → Global constants
  core/                      → Foundation (no gameplay logic)
    ├── mod.rs               → CorePlugin
    ├── state.rs             → Hierarchical state machine
    ├── plugin.rs            → Plugin trait definitions
    └── event.rs             → Global event bus
  gameplay/                  → Gameplay systems
    ├── mod.rs               → GameplayPlugin
    ├── components.rs        → ECS components (Player, Enemy, Projectile, etc.)
    ├── player.rs            → Player movement, spawning
    ├── enemies.rs           → Enemy spawning, chase AI
    ├── combat.rs            → Damage, knockback, projectiles, contact damage
    ├── xp.rs                → Magnet, collection, leveling
    ├── upgrades.rs          → Weapon evolution choices
    ├── particles.rs         → Simple transform-based particles
    ├── camera.rs            → Follow, zoom, shake, lighting overlay
    ├── resources.rs         → GameMeshes, GameMaterials, GameStats, etc.
    └── ui.rs               → HUD, pause, controls, death, game over
  ui/                        → UI framework
    ├── mod.rs               → UIPlugin
    ├── screen.rs            → Screen stack (push/pop)
    ├── components.rs        → Reusable UI widgets
    ├── theme.rs             → Theming system
    ├── menu.rs              → Main menu screen
    └── hud.rs               → In-game HUD
  audio/                     → Audio system
    ├── mod.rs               → AudioPlugin
    └── sfx.rs               → Procedural SFX generation + playback
  characters/                → Character definitions & registry
    ├── mod.rs               → CharacterPlugin
    └── registry.rs          → CharacterDef, CharacterRegistry
  achievements/              → Achievement system
    ├── mod.rs               → AchievementPlugin
    ├── registry.rs          → AchievementDef, AchievementRegistry
    └── conditions.rs        → Condition checking
  shop/                      → Shop & permanent upgrades
    ├── mod.rs               → ShopPlugin
    ├── catalog.rs           → ShopItemDef, ShopCatalog
    └── purchases.rs         → Purchase processing
  save/                      → Profile persistence
    ├── mod.rs               → SavePlugin
    └── profile.rs           → PlayerProfile, serialization
  assets/                    → Asset loading pipeline
    ├── mod.rs               → AssetPlugin
    └── loading.rs           → Loading screen system
```

---

## Design Principles

1. **Plugin isolation** — Every module is a `Plugin` that registers its own systems, resources, events, and state transitions. No cross-module coupling via systems.

2. **Event-driven** — Gameplay events (DamageEvent, KillEvent, LevelUpEvent, etc.) are fired on the Bevy event bus. Systems listen, not call.

3. **Hierarchical state** — A stack-based state machine replaces the flat enum. Menus push onto the stack; gameplay sits beneath. Pop returns to previous state.

4. **ECS-first** — Data is in components. Systems read/write components. Resources for cross-cutting state. No singletons, no globals.

5. **Prefab system** — Entity blueprints with traits, not ad-hoc spawn logic. Characters, enemies, projectiles all use templates.

6. **AI state machines** — Every enemy has a brain component with a state machine (Idle/Patrol/Chase/Attack/Flee/PhaseShift). States are modular and reusable.

7. **Modifier stack** — Stats have a base value + modifier list. Buffs, debuffs, gear all add modifiers. No magic number multiplication.

8. **Data-driven** — All gameplay definitions (characters, enemies, items, upgrades, achievements) are data tables. Balance changes never touch system code.

---

## State Machine

```
Booting       → Loading       → MainMenu
                                    ├── Playing → Paused → Playing
                                    ├── Shop → MainMenu
                                    ├── Settings → MainMenu
                                    └── Achievements → MainMenu
                              Playing → GameOver → MainMenu
```

### States

| State | Type | Purpose |
|-------|------|---------|
| Booting | Root | Initialize systems, load config |
| Loading | Transient | Asset loading screen |
| MainMenu | Root | Game title, start, shop, settings |
| Playing | Root | Active gameplay |
| Paused | Overlay | In-game pause menu |
| Shop | Overlay | Permanent upgrade shop |
| Settings | Overlay | Audio/video settings |
| Achievements | Overlay | Achievement list |
| GameOver | Root | Results screen, restart |

---

## Core Systems

### State Machine (v2)

```rust
pub struct AppState {
    Booting,
    Loading,
    MainMenu,
    Playing,
    GameOver,
}

pub struct GameOverlayState {
    None,
    Paused,
    Shop,
    Settings,
    Achievements,
}
```

Each transition fires OnEnter/OnExit systems and carries optional transition data (e.g., GameOverData with final stats).

### Event Bus

| Event | Source | Listeners |
|-------|--------|-----------|
| DamageEvent | Combat system | Enemy death, UI, screen shake |
| KillEvent | Combat system | XP drop, stats, achievements |
| XpPickupEvent | XP system | Leveling |
| LevelUpEvent | XP system | UI, upgrade menu |
| GameOverEvent | Player death | UI, save, stats |
| WaveStartEvent | Wave spawner | UI, difficulty scaling |
| PlayerDeathEvent | Player health | Game over trigger |
| AudioEvent | Various | Audio playback |

---

## Combat System

### Damage Pipeline
1. Source deals `DamageEvent { amount, kind, source, target }`
2. Target processes through defense/resistance modifiers
3. Resulting damage applied to health component
4. Knockback vector computed from source position + power
5. Status effects checked for proc

### Damage Kinds
- `Physical` — melee, projectile
- `Magic` — spells, area effects
- `True` — bypasses all defenses
- `Elemental(Fire/Ice/Lightning/Poison)` — interacts with status effects

### Status Effects
- Burn — DOT over time
- Freeze — reduced move speed into stun
- Poison — DOT, stacks
- Stun — can't act
- Slow — move speed debuff
- Vulnerable — increased damage taken

---

## Enemy AI System

### Brain Component
- Current state machine state
- Timer for state transitions
- Memory of player position, last seen

### Perception
- Sight range, hearing range
- Memory duration (time before forgetting player)
- Alert propagation (nearby enemies share awareness)

### Enemy Types (Target)

| Type | Behavior | Special Mechanic |
|------|----------|------------------|
| Grunt | Chase + melee | Swarm tactics, basic |
| Archer | Keep distance + ranged | Volley fire, flees when approached |
| Brute | Slow chase + big melee | Armor, knockback resist |
| Summoner | Flee + spawn minions | Shielded while summoning |
| Charger | Charge at player | Stuns on impact |
| Boss_01 | Multi-phase | Summon, AoE, enrage |
| Sniper | Static, long-range | Telegraphed shot, high damage |
| Swarm | Fast, low HP | Splits on death into more units |
| Shielder | Guards other enemies | Projectile-absorbing shield |
| Phase_Shifter | Teleports | Blinks around player, attacks from behind |

---

## Stat System

### Core Stats
- Health (HP) / Max Health
- Move Speed
- Attack Damage
- Attack Range
- Attack Speed (interval)
- Armor (damage reduction)
- Magic Resistance

### Secondary Stats
- Crit Chance / Crit Damage
- Dodge Chance
- Life Steal
- Thorns (reflect damage)
- Magnet Range
- XP Gain Multiplier
- Dash Count / Dash Cooldown

### Resource Stats
- Health (HP)
- Mana / Energy
- Stamina
- Dash Charges

### Modifier Stack System
```
Stat[AttackDamage] = Base × (1 + Sum(FlatBonuses)) × Product(Multipliers)
```

Each modifier tracks:
- source_id (which item/buff/gave it)
- value
- modifier_type (Flat, PercentAdd, PercentMult)
- duration (None for permanent)

---

## Character System

### Character Template

```rust
CharacterDef {
    id, name, desc,
    // Base stats
    speed, max_hp, attack_damage, attack_range, attack_interval,
    mana, stamina,
    // Secondary
    crit_chance, crit_damage, dodge_chance, life_steal,
    // Unique
    passive_ability_id,
    starting_weapon_id,
    unique_mechanic_id,
    // Meta
    cost, color,
    // Progression
    mastery_curve: Vec<(level, unlock)>,
}
```

### Planned Characters

| ID | Name | Archetype | Unique Mechanic |
|----|------|-----------|-----------------|
| reaper | Reaper | Balanced | Soul Siphon — kills restore HP |
| harbinger | Harbinger | Heavy Hitter | Overcharge — attacks build a heavy hit |
| shade | Shade | Speedster | Shadow Step — dodge through enemies |
| wraith | Wraith | Rapid Fire | Phantom Volley — bonus projectiles |
| necromancer | Necromancer | Summoner | Raise minions from kills |
| pyromancer | Pyromancer | Elemental | Burn spreads between enemies |
| tempest | Tempest | CC Focus | Chain lightning between enemies |
| paladin | Paladin | Tank | Damage aura, ally shield |
| assassin | Assassin | Burst | Backstab bonus, stealth |
| voidweaver | Voidweaver | Utility | Telefragging, debuff spread |

---

## Map & Biomes

### Tile System
- Grid-based (32-64px tiles)
- Prefab rooms connected by corridors
- Biome tilesets (color palette, obstacles, decorations)

### Planned Biomes

| Biome | Theme | Hazards | Special |
|-------|-------|---------|---------|
| Soul Wastes | Desert of bones | Sandstorms | XP geysers |
| Crimson Cathedral | Gothic | Blood pools (slow/damage) | Altars (bless/curse) |
| Void Depths | Underdark | Darkness patches | Teleporters |
| Frost Maw | Ice caves | Slippery floors | Ice crystals (cover) |
| Ember Forge | Volcanic | Lava pools | Smith altars (upgrade) |

### Room Types
- Combat arena (most rooms)
- Treasure room (gold/items)
- Boss room (boss encounter)
- Rest shrine (heal/upgrade)
- Shop room (in-run shop)
- Secret room (hidden, rare)

---

## Upgrade & Progression

### Weapon Evolutions
- Attack Speed (+50% fire rate)
- Damage (+50% raw damage)
- Range (+50% reach)
- Pierce (projectiles pass through enemies)
- Split (projectiles split on hit)
- Homing (projectiles track nearest enemy)
- Elemental Infusion (adds elemental damage)

### Permanent Upgrades (Shop)
| ID | Name | Max | Effect |
|----|------|-----|--------|
| hp_up | Vitality Boon | 5 | +20 max HP |
| dmg_up | Soul Render | 3 | +15% damage |
| speed_up | Wind Walker | 3 | +10% move speed |
| magnet_up | Magnetism | 3 | +30% magnet range |
| xp_up | Wisdom | 3 | +20% XP gain |
| extra_proj | Duality | 1 | Extra projectile |
| armor_up | Iron Will | 3 | +10% damage reduction |
| crit_up | Death Gaze | 3 | +5% crit chance |
| life_steal | Vampiric Touch | 2 | +3% lifesteal |
| dash_up | Phantom Step | 3 | +1 dash charge |
| mana_up | Soul Well | 3 | +30 max mana |
| regen_up | Vital Flow | 3 | +2 HP/sec regen |

### In-Run Upgrades
- Weapon evolutions (chosen at level-up)
- Passive items (drops from enemies/chests)
- Boons (temporary blessings from shrines)
- Curses (risk/reward debuffs)

---

## Achievement System

### New Achievements (Target)

| ID | Name | Condition | Reward |
|----|------|-----------|--------|
| first_blood | First Blood | Kill 1 enemy | 50 |
| centurion | Centurion | Kill 100 enemies | 200 |
| survivor | Survivor | Survive 60 seconds | 150 |
| level_10 | Peak Performance | Reach level 10 | 300 |
| wave_10 | Tidal Wave | Survive 10 waves | 250 |
| wave_25 | Endless Tide | Survive 25 waves | 500 |
| collector | Collector | Unlock 3 upgrades | 400 |
| rich | Soul Hoarder | Earn 1000 total currency | 500 |
| character_unlock | World Walker | Unlock all characters | 1000 |
| no_hit | Untouchable | Beat a wave without taking damage | 350 |
| boss_slayer | Boss Slayer | Defeat 5 bosses | 600 |
| perfect_run | Perfect Run | Win without dying | 2000 |
| combo_king | Combo King | Chain 50 kills within 3 seconds | 400 |
| full_upgrades | Ascended | Purchase all shop upgrades | 1500 |
| elementalist | Elementalist | Apply all 4 elements in one run | 450 |

---

## Module Dependency Graph

```
Core (no deps)
├── Save (Core: state)
├── Audio (Core: events)
├── Assets (Core: state)
├── Characters (Core: none)
├── Achievements (Core: events, Save: profile)
├── Shop (Core: state, Save: profile)
├── Gameplay (Core, Characters, Save, Shop)
│   ├── Player
│   ├── Enemies (AI)
│   ├── Combat (events)
│   ├── XP (events)
│   ├── Upgrades
│   ├── Maps (biomes, rooms)
│   └── Items/Equipment
├── UI (Gameplay, Save)
│   ├── Menu
│   ├── HUD
│   ├── Shop Screen
│   ├── Settings Screen
│   ├── Achievements Screen
│   └── Inventory Screen
└── Networking (Core, Gameplay) [Future]
```

---

## Tech Stack

| Layer | Technology | Status |
|-------|-----------|--------|
| Engine | Bevy 0.18 | Active |
| Rendering | Bevy PBR (StandardMaterial, Mesh3d) — 3D pipeline | Active |
| Camera | Orthographic, isometric angle (45° tilt, 45° rotation) | Planned |
| Physics | Avian3D (collision, forces, layers, constrained to XZ plane) | Planned |
| Lighting | Bevy native 3D lights (DirectionalLight, PointLight, SpotLight) | Planned |
| Particles | bevy_hanabi (GPU particles, 3D) | Planned |
| Shaders | Custom WGSL + Bevy post-processing | Planned |
| Post FX | Bloom, color grading, vignette (Bevy core pipeline) | Planned |
| Audio | bevy_audio + ogg/wav pipeline | Active (basic) |
| Networking | bevy_replicon (future) | Planned |
| Serialization | serde + serde_json | Active |
| RNG | rand 0.8 | Active |
| ECS Queries | bevy_ecs query filters | Active |
| Asset Pipeline | bevy_asset + custom loader + glTF | Active |
| Input | bevy_input (keyboard + controller planned) | Active |
| UI Framework | bevy_ui | Active |

---

## Development Roadmap

### Phase 1: Foundation (Current — Prototype)
- [x] Project scaffolding
- [x] Core state machine
- [x] Plugin framework
- [x] Event bus
- [x] Basic gameplay loop (move, attack, level, die)
- [x] Save/load profile
- [x] Character registry (4 characters)
- [x] Shop catalog (6 upgrades)
- [x] Achievement registry (7 achievements)
- [x] UI screens (menu, HUD, pause, game over)
- [x] Camera (follow, zoom, shake)
- [x] Audio (procedural SFX)
- [x] **Modular refactor (T0.2)** — all monolithic files eliminated, clean mod structure
- [x] **Bevy 0.18 API migration** — Message system, new UI, AudioPlayer
- [x] **Comprehensive stat system (T1.1 + T1.2)** — StatInstance, ModifierStack, 24 stat definitions, 14 tests
- [x] **Cargo.toml dep updates (T0.1)** — all future deps listed (commented)
- [ ] Upgrade Cargo.toml with all required dependencies (uncomment as used)
- [ ] Add comprehensive stat system with modifier stack
- [ ] Expand character roster (10 characters)
- [ ] Implement proper enemy AI state machine
- [ ] Expand enemy types (8+ types)
- [ ] Implement map generation with biomes
- [ ] Add physics engine (collision, knockback physics)
- [ ] Add 2D lighting
- [ ] Add GPU particle system
- [ ] Add post-processing pipeline (bloom)
- [ ] Create proper asset pipeline for Blender import

### Phase 2: Content (Vertical Slice)
- [ ] Build full biome set (5+ biomes)
- [ ] Implement boss encounters
- [ ] In-run item system
- [ ] Weapon evolution variants (7+)
- [ ] Passive item drops
- [ ] Shop screen UI
- [ ] Settings screen (audio, video, controls)
- [ ] Achievement UI
- [ ] Character unlock screen
- [ ] Progression mastery system
- [ ] Elemental synergy system
- [ ] Status effect interactions

### Phase 3: Polish
- [ ] Custom shaders and post-effects
- [ ] Screen transitions and animations
- [ ] Controller support
- [ ] Accessibility features
- [ ] Performance optimization
- [ ] Localization framework
- [ ] Full sound design pipeline
- [ ] UI animations and juice

### Phase 4: Expansion
- [ ] Debug/developer console
- [ ] Replay system
- [ ] Mod support framework
- [ ] Online leaderboards
- [ ] Co-op multiplayer (networking)
- [ ] Modding API

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Frame rate | 60 FPS (144 FPS target for high-end) |
| Simultaneous entities | 500+ enemies + projectiles + particles |
| Loading time | < 3 seconds cold start |
| Save file size | < 100 KB |
| Memory budget | < 512 MB |
| Build time (debug) | < 30 seconds |
| Build time (release) | < 3 minutes |
| WASM support | Future target |

---

## Future Considerations

- **WebAssembly** — Bevy supports WASM. Architecture should remain WASM-compatible.
- **Mobile** — Touch controls and performance profiles for mobile.
- **Modding** — Data-driven design enables modding via content packs.
- **Local co-op** — Split-screen or shared-screen co-op on roadmap.
- **Online co-op** — bevy_replicon for deterministic multiplayer.
- **Steam integration** — Achievements, cloud saves, leaderboards.
