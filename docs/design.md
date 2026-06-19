# ReapTide — Design Document

---

## Document Metadata

| Field | Value |
|-------|-------|
| Version | 1.0.1 |
| Date | 2026-06-18 |
| Last Updated | 2026-06-18T16:30:00-04:00 |
| Status | Draft — Active Development |
| Author | Design Agent (Quicksilver) |
| Classification | Internal — Implementation Reference |

---

## Table of Contents

1. Game Overview
2. Core Loop
3. Stat System — Detailed Design
4. Economy & Currencies
5. Combat System — Detailed Design
6. Enemy AI — Detailed Design
7. Map Generation — Detailed Design
8. Character Creation & Progression
9. Item & Equipment Systems
10. Elemental & Status Effect System
11. UI/UX Design
12. Audio Design
13. Technical Design Decisions

---

## 1. Game Overview

ReapTide is a **top-down 2D action roguelite** where the player controls a soul-reaping entity fighting through hordes of enemies across increasingly difficult waves. The game combines:

- **Vampire Survivors'** auto-attack, screen-clearing, horde-survival gameplay
- **Hades'** moment-to-moment movement depth, build variety, and character expression
- **Roguelite progression** with persistent meta-upgrades between runs

The player character auto-attacks nearby enemies. The player controls movement, positioning, dodge timing, and upgrade selection. Each run is unique — the weapon evolutions, passive items, and boons available create emergent build variety.

### Target Audience

- Players who enjoy roguelites (Hades, Dead Cells, Rogue Legacy)
- Players who enjoy survivor-likes (Vampire Survivors, Brotato, Halls of Torment)
- Casually competitive players (leaderboards, speedrunning)
- Players who enjoy buildcraft and theorycrafting

---

## 2. Core Loop

### Per-Run Loop

```bash
Enter biome → Survive waves → Level up (choose upgrade) → 
Defeat mid-boss → Enter next biome → 
Defeat biome boss → Survive more waves →
Defeat final boss → Victory / Restart
```

### Death Loop

```bash
Die → Game Over screen (stats) → Currency awarded → 
Main Menu → Shop (spend currency on permanent upgrades) → 
Select character → Enter new run
```

### Meta Loop

```bash
Complete runs → Earn Souls (currency) → 
Buy permanent upgrades in Shop → 
Unlock new characters → 
Unlock new starting weapons → 
Master characters (per-character progression) → 
Enable harder difficulty tiers → 
Earn more Souls faster
```

---

## 3. Stat System — Detailed Design

### 3.1 Core Character Stats

| Stat | Default | Min | Max | Description |
|------|---------|-----|-----|-------------|
| max_health | 100 | 1 | 9999 | Maximum HP |
| move_speed | 280 | 50 | 800 | Pixels per second |
| attack_damage | 18 | 1 | 9999 | Base damage per hit |
| attack_range | 180 | 30 | 1000 | Pixels, auto-attack range |
| attack_interval | 0.35 | 0.05 | 5.0 | Seconds between attacks |
| armor | 0 | 0 | 95 | Flat damage reduction % |
| magic_resist | 0 | 0 | 95 | Magic damage reduction % |

### 3.2 Secondary Stats

| Stat | Default | Min | Max | Description |
|------|---------|-----|-----|-------------|
| crit_chance | 0.0 | 0.0 | 1.0 | Probability of critical hit |
| crit_damage | 1.5 | 1.0 | 5.0 | Multiplier on critical hit |
| dodge_chance | 0.0 | 0.0 | 0.8 | Probability to dodge attack |
| life_steal | 0.0 | 0.0 | 1.0 | Fraction of damage healed |
| thorns | 0.0 | 0.0 | 999 | Damage reflected per hit |
| magnet_range | 150 | 50 | 800 | XP gem attraction radius |
| xp_mult | 1.0 | 0.5 | 5.0 | XP gain multiplier |
| dash_count | 1 | 0 | 5 | Consecutive dashes |
| dash_cooldown | 2.0 | 0.5 | 10.0 | Seconds between dash charges |
| dash_distance | 200 | 50 | 500 | Pixels per dash |
| knockback_power | 1.0 | 0.0 | 5.0 | Knockback multiplier |
| knockback_resist | 0.0 | 0.0 | 1.0 | Knockback reduction |

### 3.3 Resource Stats

| Stat | Default | Regen | Description |
|------|---------|-------|-------------|
| health | =max_hp | 0/sec | Current HP |
| mana | 50 | 1/sec | Current mana for abilities |
| max_mana | 50 | — | Maximum mana |
| stamina | 100 | 15/sec | Stamina for dashing/sprinting |
| max_stamina | 100 | — | Maximum stamina |

### 3.4 Modifier Stack Implementation

```rust
#[derive(Clone)]
pub struct StatModifier {
    pub source_id: String,     // Identifies what applied this
    pub value: f32,
    pub modifier_type: ModifierType,
    pub duration: Option<f32>, // None = permanent
}

pub enum ModifierType {
    Flat,           // +X to stat
    PercentAdd,     // +X% (additive with other percent mods)
    PercentMult,    // ×X (multiplicative, applied last)
    Set,            // Override base value
}

pub struct StatInstance {
    pub base: f32,
    pub modifiers: Vec<StatModifier>,
}

impl StatInstance {
    pub fn value(&self) -> f32 {
        let mut base = self.base;
        let mut flat_sum = 0.0;
        let mut pct_sum = 0.0;
        let mut pct_mult = 1.0;
        let mut set_value: Option<f32> = None;
        
        for mod in &self.modifiers {
            match mod.modifier_type {
                Flat => flat_sum += mod.value,
                PercentAdd => pct_sum += mod.value,
                PercentMult => pct_mult *= mod.value,
                Set => set_value = Some(mod.value),
            }
        }
        
        if let Some(s) = set_value { return s; }
        (base + flat_sum) * (1.0 + pct_sum) * pct_mult
    }
}
```

---

## 4. Economy & Currencies

### 4.1 Currency Types

| Currency | Earned By | Spent On | Persists? |
|----------|-----------|----------|-----------|
| Souls | End of run (based on performance) | Shop upgrades, character unlocks | Yes — cross-run |
| Essence | In-run drops | In-run shop, shrines | No — per run |
| Bones | Boss kills, achievements | Cosmetic unlocks | Yes — cross-run |
| Tears of the Fallen | Secret finds, challenges | Hidden content | Yes — account |

### 4.2 Soul Economy Balance

| Source | Souls Earned | Notes |
|--------|-------------|-------|
| Per wave survived | 10 + wave × 5 | Up to wave 30 |
| Per boss killed | 100 | Biome bosses |
| Level milestone bonus | Level × 20 | At run end |
| Kill count bonus | Kills × 2 | At run end |
| Achievement unlock | 50-2000 | One-time |

---

## 5. Combat System — Detailed Design

### 5.1 Damage Types

```bash
Physical
├── Slashing   (swords, claws)
├── Blunt      (hammers, fists)
├── Piercing   (arrows, projectiles)
└── Explosive  (AoE, environmental)

Magic
├── Fire       (burn DOT)
├── Ice        (slow, freeze)
├── Lightning  (chain, stun)
├── Poison     (stacking DOT)
└── Arcane     (true damage, rare)

True Damage (bypasses all defenses)
```

### 5.2 Damage Formula

```rust
RawDamage = BaseDamage × DamageMultiplier
DefenseMultiplier = (1.0 - Armor/100.0) × (1.0 - MagicResist/100.0)
FinalDamage = RawDamage × DefenseMultiplier × RandomFactor(0.95-1.05)

If Crit: FinalDamage × CritMultiplier
If Dodge: FinalDamage = 0
If Block: FinalDamage × 0.3
```

### 5.3 Knockback Physics

```rust
pub struct Knockback {
    pub force: f32,           // Base knockback distance
    pub direction: Vec2,      // Normalized direction
    pub duration: f32,        // How long knockback applies
    pub falloff: f32,         // 0.0-1.0 how much force reduces over duration
}
```

Knockback is additive with physics engine forces. Enemies with `knockback_resist = 1.0` are immune. Walls stop knockback.

### 5.4 Status Effect Interactions

| Effect | Source | Stacks? | Can Cure? | Interaction |
|--------|--------|---------|-----------|-------------|
| Burn | Fire damage | Yes (intensity) | Water/ice cleanse | +X% damage per stack |
| Freeze | Ice damage | No | Fire damage thaws | Roots + -50% armor while frozen |
| Poison | Poison damage | Yes (count) | No | % max HP per stack per tick |
| Stun | Lightning, heavy hits | No | Tenacity stat | Cannot act for duration |
| Slow | Ice, debuffs | Partial | Haste buffs | -% move speed, can't go below 20% |
| Vulnerable | Dark magic, curses | No | Purify | +30% damage taken |
| Bleed | Slashing weapons | Yes (intensity) | Bandages/heal | Physical DOT, ignores armor |
| Taunt | Special enemies | No | Tenacity | Forces target to face taunter |

---

## 6. Enemy AI — Detailed Design

### 6.1 Brain Component

```rust
#[derive(Component)]
pub struct EnemyBrain {
    pub state: AiState,
    pub state_timer: Timer,
    pub aggression_range: f32,
    pub memory_duration: f32,
    pub last_known_player_pos: Option<Vec2>,
    pub alert_timer: Timer,
}

#[derive(Clone, PartialEq)]
pub enum AiState {
    Idle,       // Wandering, waiting
    Patrol,     // Following waypoints
    Alert,      // Heard something, investigating
    Chase,      // Pursuing player
    Attack,     // Executing attack pattern
    Flee,       // Retreating at low HP
    Stunned,    // Cannot act
    Special,    // Boss-specific behavior
    Dead,       // Death sequence
}
```

### 6.2 State Transition Rules

```bash
Idle → Patrol:        Always (if waypoints exist)
Idle/Patrol → Alert:  Event heard within hearing range
Alert → Chase:        Player sighted within sight range
Alert → Patrol:       Lost player, memory expired
Chase → Attack:       Within attack range
Attack → Chase:       Player left attack range
Attack → Idle:        Attack completed, player lost
Any → Flee:           HP < flee_threshold
Any → Stunned:        Stun status effect applied
Stunned → Previous:   Stun duration expired
```

### 6.3 Behavior Sets

Each enemy type has a behavior set defining which states are available, transition timings, and attack patterns.

| Enemy | States | Attack Pattern | Special |
|-------|--------|---------------|---------|
| Grunt | Chase, Attack, Flee | Melee swing on contact | Swarm bonus (+X% damage per nearby grunt) |
| Archer | Patrol, Chase, Attack, Flee | Ranged volley (3 shots) | Flees when player < 100px |
| Brute | Chase, Attack | Heavy slam (AoE) | 50% knockback resist, armor 30 |
| Summoner | Flee, Attack | Spawns 2 minions | Shielded while summoning (invuln) |
| Charger | Chase, Attack | Charge (linear dash) | Stuns on impact, 3s cooldown |
| Sniper | Idle, Attack | Telegraphed laser shot | 500px range, 3s charge time |
| Swarm | Chase, Attack | Melee nibble | Splits into 2 smaller on death |
| Shielder | Patrol, Attack | Body blocks | Deploys shield absorbing 5 hits |
| Phase_Shifter | Chase, Attack | Teleport strike | Blinks within 200px of player |

### 6.4 Boss AI

Bosses use a multi-phase FSM. Each phase has its own behavior set, and transitions happen at HP thresholds.

```bash
Phase 1 (100%-66%):  Standard attacks, occasional summons
Phase 2 (66%-33%):   Faster attacks, new pattern, enrage adds
Phase 3 (33%-0%):    All patterns active, berserk mode, AoE spam
```

---

## 7. Map Generation — Detailed Design

### 7.1 Room-Based Generation

Algorithm: Modified BSP (Binary Space Partition) with corridor connection.

```text
1. Divide map area into rooms via BSP
2. Connect rooms via L-shaped corridors
3. Assign room types (combat, treasure, boss, rest, shop, secret)
4. Place wave spawn points based on room type
5. Decorate each room with biome-specific tileset
6. Place obstacles, hazards, and interactables
```

### 7.2 Room Templates

Each biome has a pool of room templates. Templates define:

- Tile layout (open, corridors, choke points)
- Obstacle positions (pillars, walls, pits)
- Spawn zones (edge, center, corners)
- Exit positions (doors, corridors)

### 7.3 Biome Progression

```
Biome 1: Soul Wastes  (easy, 8-12 waves, mid-boss)
Biome 2: Crimson Cathedral (medium, 10-15 waves, mid-boss)
Biome 3: Void Depths  (hard, 12-18 waves, mid-boss)
Biome 4: Frost Maw    (harder, 14-20 waves, mid-boss)
Biome 5: Ember Forge  (hardest, 16-25 waves, final boss)
```

Each biome has 3-5 room layouts, 2-4 hazard types, and unique biome-specific mechanics (e.g., Void Depths have darkness zones that conceal enemies).

---

## 8. Character Creation & Progression

### 8.1 Character Unlock Progression

| Character | Unlock Condition | Cost |
|-----------|-------------------|------|
| Reaper | Default | Free |
| Harbinger | Reach account level 2 | 500 Souls |
| Shade | Reach account level 4 | 800 Souls |
| Wraith | Kill 500 total enemies | 1200 Souls |
| Necromancer | Complete a run with all 3 weapon upgrades | 2000 Souls |
| Pyromancer | Inflict 10000 fire damage total | 2500 Souls |
| Tempest | Reach wave 20 | 3000 Souls |
| Paladin | Survive 30 minutes total | 3500 Souls |
| Assassin | Unlock all other characters | 5000 Souls |
| Voidweaver | Full-clear a run (all biomes) | 8000 Souls |

### 8.2 Character Mastery

Each character has a mastery track (10 levels):

- XP earned per run with that character
- Each level unlocks a character-specific bonus
- Max level unlocks exclusive cosmetic

| Mastery Level | Unlock |
|---------------|--------|
| 1 | +5% base damage |
| 2 | Unique starting passive |
| 3 | +10% move speed |
| 4 | Bonus starting weapon evolution option |
| 5 | Unique dash variant |
| 6 | +15% XP gain on this character |
| 7 | Unique stat bonus |
| 8 | +1 starting level |
| 9 | Exclusive cosmetic |
| 10 | Mastery achievement + bonus perk slot |

---

## 9. Item & Equipment Systems

### 9.1 Item Rarity

| Rarity | Color | Drop Chance | Modifier Count |
|--------|-------|-------------|----------------|
| Common | White | 50% | 0-1 |
| Uncommon | Green | 30% | 1-2 |
| Rare | Blue | 15% | 2-3 |
| Epic | Purple | 4% | 3-4 |
| Legendary | Gold | 1% | 4-5 |

### 9.2 Item Types

| Type | Slots | Examples |
|------|-------|----------|
| Weapon | 1 | Scythe, Bow, Staff, Daggers |
| Armor | 1 | Robe, Plate, Cloak |
| Accessory | 2 | Ring, Amulet, Relic |
| Consumable | — | Health potion, Bomb, Scroll |

### 9.3 Weapon Types

| Weapon | Behavior | Base Damage | Range | Speed |
|--------|----------|-------------|-------|-------|
| Scythe | Wide arc melee | 25 | 120 | 0.4s |
| Bow | Single projectile | 18 | 300 | 0.5s |
| Staff | Magic bolt, pierces | 15 | 250 | 0.6s |
| Daggers | Fast, dual strike | 10 | 80 | 0.15s |
| Spear | Long thrust, line AoE | 30 | 200 | 0.7s |
| Chakram | Bouncing projectile | 20 | 200 | 0.5s |
| Scepter | Summon orb familiar | 12 | 150 | 0.8s |
| Greatsword | Slow, huge arc | 45 | 160 | 1.0s |

---

## 10. Elemental & Status Effect System

### 10.1 Elemental Interactions

| Primary → Target | Result |
|------------------|--------|
| Fire → Wet | Steam cloud (blind area) |
| Fire → Frozen | Shatter (+200% damage) |
| Ice → Wet | Freeze (root) |
| Ice → Burning | Extinguish (removes burn, creates smoke) |
| Lightning → Wet | Chain lightning (+50% damage per jump) |
| Lightning → Metal | Magnetic field (attracts enemies) |
| Poison → Fire | Toxic cloud (AoE DOT) |
| Poison → Lightning | Nerve shock (stun + DOT) |

### 10.2 Status Effect Stacking Rules

- **Burn**: Stacks by intensity (max 10). Each tick deals intensity * 5 damage. Duration refreshes on application.
- **Poison**: Stacks by count (max 5). Each stack deals 2% max HP per tick. Independent timers.
- **Freeze**: Does not stack. Replaces existing freeze duration if longer.
- **Slow**: Stacks by percentage (max 80% slow). Multiple sources add diminishingly.
- **Bleed**: Stacks by intensity (max 20). Each stack deals 3 flat damage per tick.

---

## 11. UI/UX Design

### 11.1 Screen Hierarchy

```bash
MainMenu
├── Title / Logo animation
├── Start Game
├── Shop
│   ├── Permanent Upgrades
│   ├── Character Unlocks
│   └── Cosmetics [Future]
├── Achievements
├── Settings
│   ├── Audio (master/music/sfx sliders)
│   ├── Video (resolution, fullscreen, vsync, quality presets)
│   └── Controls (key rebinding)
└── Quit

In-Game
├── HUD (top bar)
│   ├── Health bar (with regen preview)
│   ├── Mana bar
│   ├── Stamina bar
│   ├── Level / XP bar
│   ├── Wave counter
│   └── Build summary (active upgrades + items)
├── Upgrade Menu (modal on level-up)
│   ├── 3-4 card-style choices
│   ├── Reroll button [consumes resource]
│   └── Tooltip on hover
├── Pause Menu (overlay)
│   ├── Resume
│   ├── Settings shortcut
│   └── Quit Run
├── Game Over Screen
│   ├── Run stats (level, kills, time, damage, gold)
│   ├── Souls earned
│   ├── New unlocks (if any)
│   └── Return to Menu / Play Again
└── Inventory [Future]
    ├── Current items
    ├── Stats breakdown
    └── Item tooltips
```

### 11.2 UI Philosophy

- **Minimal during combat** — HUD presents only essential info during gameplay
- **Rich on pause** — Stats breakdown, build analysis on pause screen
- **Tooltip everything** — Hover over any upgrade, item, or stat for detailed explanation
- **Juice** — UI animations on level-up, damage numbers, item pickups, achievement popups
- **Controller-friendly** — All UI navigable by gamepad (d-pad + buttons)

---

## 12. Audio Design

### 12.1 Current (Procedural) — Phase 1

- 6 synthetic sound effects generated at startup
- No music
- No ambient/environmental audio
- No spatial audio

### 12.2 Target (Asset-based) — Phase 2+

| Category | Assets Needed | Format |
|----------|---------------|--------|
| SFX: Combat | Hits, misses, projectiles, impacts | WAV/OGG |
| SFX: UI | Hover, click, confirm, cancel | WAV/OGG |
| SFX: Environment | Wind, fire, water, footsteps | WAV/OGG |
| SFX: Powers | Spells, abilities, buffs, debuffs | WAV/OGG |
| Music: Menu | Atmospheric theme | OGG |
| Music: Biome 1 | Desolate, wind-driven | OGG |
| Music: Biome 2 | Gothic, organ-driven | OGG |
| Music: Biome 3 | Dark ambient, drone | OGG |
| Music: Biome 4 | Frigid, minimal | OGG |
| Music: Biome 5 | Intense, percussive | OGG |
| Music: Boss | Epic, dynamic layers | OGG |
| Music: Victory | Triumphant | OGG |
| Voice: Player | Damage, death, ability lines | OGG [Future] |

### 12.3 Audio Architecture

```rust
pub struct AudioManager {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub ambient_volume: f32,
    pub voice_volume: f32,
    pub current_music: Option<Handle<AudioSource>>,
    pub music_bus: AudioBus,
    pub sfx_bus: AudioBus,
}
```

Planned: ECS-based audio events with spatial audio for 3D positioning (when game goes 3D). Audio mixing through Bevy's audio system with per-bus volume control.

---

## 13. Technical Design Decisions

### 13.1 Why 3D Isometric?

Chosen over 2D top-down and pure 3D for the following reasons:

**vs 2D Top-Down:**

- Bevy's 3D pipeline (PBR, lights, shadows) is far more capable than its 2D pipeline
- One Blender model instead of 8 sprite angles or billboard hacks
- Real-time lighting with real shadows — not faked overlays
- Post-processing (bloom, depth of field) works natively

**vs Full 3D (third-person):**

- Gameplay is still 2D — movement on a plane, enemies chase on a plane
- No complex camera controls, no z-targeting, no vertical aiming
- No animation complexity of a full 3D game
- Easier to balance, easier to test, easier to ship

**The architecture:**

```rust
// Y is up. Ground plane is Y=0.
// Player movement is on XZ plane.
// Camera is orthographic at 45° down, 45° rotated.
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
    IsometricCamera, // Marker component
    Transform::from_xyz(0.0, ISO_HEIGHT, ISO_HEIGHT)
        .looking_at(Vec3::ZERO, Vec3::Y),
));
```

### 13.2 Why Bevy 0.18?

- **ECS architecture** naturally fits game development
- **Rust** provides memory safety and performance without GC
- **Plugin system** matches our modular architecture
- **WASM support** for future web deployment
- **Active development** with strong community

### 13.3 Physics: Avian3D (2D movement on 3D plane)

We use Avian3D with movement constrained to the XZ plane and Y=0. This gives us proper 3D collision shapes (for the isometric visual) with 2D gameplay mechanics. Collision layers separate player, enemy, projectile, and environment. Physics forces drive knockback instead of manual vector math.

### 13.4 Why Event-Driven?

- Decouples modules completely
- Easy to add new listeners without modifying producers
- Enables easy debugging (log all events)
- Parallelizable system execution in Bevy

### 13.5 Data-Driven Design

All gameplay-defining data lives in registries/tables, not in system code. This enables:

- Balance patches without code changes
- Hot-reloading data during development
- Future modding support
- Easy A/B testing of balance values

### 13.6 Versioning Strategy

- **Major** (.0.0): Complete phase delivery, breaking changes
- **Minor** (0..0): New features, non-breaking additions
- **Patch** (0.0.): Bug fixes, balance adjustments

---

## Appendix: Migration Path (2D Top-Down → 3D Isometric)

| # | Task | Effort | Priority |
|---|------|--------|----------|
| 0 | **Switch to 3D rendering** | Medium | Critical |
|   | • Replace Mesh2d + ColorMaterial with Mesh3d + StandardMaterial | | |
|   | • Replace Camera2d with Camera3d (orthographic, isometric angle) | | |
|   | • Convert XY movement to XZ, Y becomes up/height | | |
|   | • Ground plane at Y=0, entities at Y=0.5 (or their height) | | |
| 1 | Update Cargo.toml with all new dependencies (avian3d, hanabi, etc.) | Small | Critical |
| 2 | Implement StatInstance + ModifierStack system | Medium | Critical |
| 3 | Implement EnemyBrain FSM | Medium | Critical |
| 4 | Expand character registry (10 characters) | Medium | High |
| 5 | Expand enemy registry (10 types) | Medium | High |
| 6 | Integrate Avian3D physics (XZ-constrained) | Medium | High |
| 7 | Add bevy_hanabi 3D GPU particle system | Medium | High |
| 8 | Add 3D lighting (DirectionalLight + PointLight) | Medium | High |
| 9 | Implement biome + room generation | Large | High |
| 10 | Implement item/equipment system | Large | High |
| 11 | Add post-processing pipeline (bloom, vignette, color grading) | Medium | Medium |
| 12 | Build shop screen UI | Medium | Medium |
| 13 | Build settings screen UI | Medium | Medium |
| 14 | Implement elemental interactions | Medium | Medium |
| 15 | Implement boss AI framework | Large | Medium |
| 16 | Implement character mastery | Medium | Medium |
| 17 | Add proper audio asset pipeline (OGG) | Medium | Low |
| 18 | Implement controller support | Medium | Low |
| 19 | Implement debug console | Medium | Low |
| 20 | Networking foundation | Large | Future |
