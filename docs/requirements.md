# ReapTide — Requirements Specification

---

## Document Metadata

| Field | Value |
|-------|-------|
| Version | 1.0.0 |
| Date | 2026-06-18 |
| Last Updated | 2026-06-18T11:30:00-04:00 |
| Status | Draft — Active Development |
| Author | Requirements Agent (Quicksilver) |
| Classification | Internal — Project Foundation |

---

## 1. Functional Requirements

### F1: Core Gameplay Loop

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F1.1 | Player can move in 8 directions using WASD or arrow keys | P0 | Complete |
| F1.2 | Player character auto-attacks nearest enemy within range | P0 | Complete |
| F1.3 | Enemies spawn in escalating waves | P0 | Partial |
| F1.4 | Enemies chase the player with basic AI | P0 | Complete |
| F1.5 | Player takes contact damage from enemies | P0 | Complete |
| F1.6 | Enemies drop XP gems on death | P0 | Complete |
| F1.7 | Player collects XP gems via magnet attraction | P0 | Complete |
| F1.8 | Player levels up when XP threshold reached | P0 | Complete |
| F1.9 | Player chooses a weapon upgrade on level-up | P0 | Complete |
| F1.10 | Player dies when HP reaches 0 | P0 | Complete |
| F1.11 | Game over screen shows run stats | P0 | Complete |
| F1.12 | Player earns currency based on run performance | P0 | Complete |
| F1.13 | Player can restart from game over screen | P0 | Complete |

### F2: State Management

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F2.1 | Game boots into loading state | P0 | Complete |
| F2.2 | Game transitions to main menu after loading | P0 | Complete |
| F2.3 | Main menu shows title and options | P0 | Complete |
| F2.4 | "Start Game" transitions to playing state | P0 | Complete |
| F2.5 | Escape key pauses the game | P0 | Complete |
| F2.6 | Pause menu allows resume or quit | P0 | Complete |
| F2.7 | Player death transitions to game over state | P0 | Complete |
| F2.8 | Shop state available from main menu | P0 | Complete |
| F2.9 | Settings state accessible from main menu | P1 | Not started |
| F2.10 | Achievements state accessible from main menu | P1 | Not started |

### F3: Combat System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F3.1 | Projectiles travel toward target enemy | P0 | Complete |
| F3.2 | Projectiles deal damage on contact | P0 | Complete |
| F3.3 | Contact damage applies periodic DPS | P0 | Complete |
| F3.4 | Damage types include Physical, Magic, True, Elemental | P0 | Complete |
| F3.5 | Damage events propagate through event bus | P0 | Complete |
| F3.6 | Knockback applied on projectile hit | P1 | Partial (tracked but not applied) |
| F3.7 | Status effects can be applied (Burn, Freeze, Poison, Stun, Slow, Vulnerable) | P1 | Spec only |
| F3.8 | Elemental damage interacts with status effects | P2 | Not started |
| F3.9 | Damage numbers display on hit | P2 | Not started |
| F3.10 | Critical hits have visual feedback | P2 | Not started |
| F3.11 | Status effect icons display on enemy | P2 | Not started |

### F4: Enemy System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F4.1 | At least 3 enemy types (Small, Medium, Big) | P0 | Complete |
| F4.2 | Enemy stats scale with wave number | P0 | Complete |
| F4.3 | Enemies spawn at random positions around the player | P0 | Complete |
| F4.4 | At least 10 enemy types with unique behaviors | P1 | Not started |
| F4.5 | Enemies use finite state machine AI (Idle/Patrol/Chase/Attack/Flee) | P1 | Not started |
| F4.6 | Boss enemies with multi-phase AI | P2 | Not started |
| F4.7 | Enemy variety across biomes | P2 | Not started |

### F5: Progression & Economy

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F5.1 | Player profile persists across sessions | P0 | Complete |
| F5.2 | Currency (Souls) persists across sessions | P0 | Complete |
| F5.3 | 16+ permanent upgrades available in shop | P0 | Partial (6) |
| F5.4 | 10+ unlockable characters | P0 | Partial (4) |
| F5.5 | Achievements with currency rewards | P0 | Complete (7) |
| F5.6 | 25+ achievements total | P1 | Not started |
| F5.7 | Character mastery system with per-character progression | P2 | Not started |
| F5.8 | Item drops with rarity system | P2 | Not started |
| F5.9 | In-run shop with Essence currency | P2 | Not started |
| F5.10 | Weapon types beyond base auto-attack | P2 | Not started |

### F6: Stat System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
|| F6.1 | Core stats: Health, Speed, Damage, Range, AttackInterval | P0 | Complete |
|| F6.2 | Modifier stack system with Flat, PercentAdd, PercentMult | P1 | Complete |
|| F6.3 | Secondary stats: CritChance, CritDamage, Dodge, LifeSteal, Thorns | P1 | Complete (12 defined) |
|| F6.4 | Resource stats: Mana, Stamina with regen | P2 | Complete (5 defined) |
|| F6.5 | Stat breakdown UI showing modifier sources | P2 | Implemented (breakdown()) |
|| F6.6 | Temporary buffs/debuffs with duration tracking | P2 | Complete (expiry system) |

### F7: Map System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F7.1 | Tile-based grid for the game world | P0 | Spec only |
| F7.2 | Procedural room generation with BSP algorithm | P1 | Not started |
| F7.3 | 5+ biomes with distinct tilesets and hazards | P1 | Not started |
| F7.4 | Room types: Combat, Treasure, Boss, Rest, Shop, Secret | P2 | Not started |
| F7.5 | Corridor connections between rooms | P2 | Not started |
| F7.6 | Biome-specific environmental hazards | P2 | Not started |
| F7.7 | Wave spawn points marked per room | P2 | Not started |

### F8: Character System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F8.1 | Character registry with base stat definitions | P0 | Complete (4) |
| F8.2 | Each character has unique base stats | P0 | Complete |
| F8.3 | 10 characters total with unique mechanics | P1 | Not started |
| F8.4 | Character unlock conditions | P1 | Not started |
| F8.5 | Character mastery progression track | P2 | Not started |
| F8.6 | Character select screen before run | P1 | Not started |

### F9: Audio System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F9.1 | 6+ sound effects for combat/UI events | P0 | Complete (procedural) |
| F9.2 | Audio event system with event reader | P0 | Complete |
| F9.3 | Background music per biome | P2 | Not started |
| F9.4 | Master/Music/SFX volume sliders | P1 | Not started |
| F9.5 | Ambient/environmental audio per biome | P2 | Not started |
| F9.6 | Spatial audio for 3D positioning | P3 | Not started |
| F9.7 | High-quality audio asset pipeline (OGG/WAV) | P2 | Not started |

### F10: UI System

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F10.1 | Main menu with Start, Shop, Achievements, Settings, Quit | P0 | Partial |
| F10.2 | HUD with health, XP, wave, level display | P0 | Complete |
| F10.3 | Pause overlay with resume/quit | P0 | Complete |
| F10.4 | Game over screen with run stats | P0 | Complete |
| F10.5 | Level-up upgrade choice with 3 options | P0 | Complete |
| F10.6 | Controls overlay (F1) | P0 | Complete |
| F10.7 | Fullscreen toggle (F11) | P0 | Complete |
| F10.8 | Shop screen UI with purchases | P1 | Not started |
| F10.9 | Settings screen with audio/video/controls | P1 | Not started |
| F10.10 | Achievement screen with progress | P1 | Not started |
| F10.11 | Character select screen | P1 | Not started |
| F10.12 | Inventory screen showing items/equipment | P2 | Not started |
| F10.13 | Damage number popups | P2 | Not started |
| F10.14 | Tooltip system for upgrades/items | P1 | Not started |
| F10.15 | Notification toasts for unlocks/achievements | P1 | Not started |

### F11: Camera & Visuals (3D Isometric)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F11.1 | Camera follows player smoothly (XZ plane, isometric angle) | P0 | Refactoring (was 2D) |
| F11.2 | Mouse wheel zoom (orthographic scale) | P0 | Refactoring (was 2D) |
| F11.3 | Screen shake on damage (offset in camera space) | P0 | Refactoring (was 2D) |
| F11.4 | Replace dark overlay with proper 3D lighting | P0 | Refactoring (was 2D overlay) |
| F11.5 | 3D PBR rendering pipeline (StandardMaterial) | P1 | Not started (was Mesh2d) |
| F11.6 | DirectionalLight with shadow mapping | P1 | Not started |
| F11.7 | Point light sources on player, projectiles, abilities | P1 | Not started |
| F11.8 | GPU particle system (bevy_hanabi 3D) | P1 | Not started |
| F11.9 | Bloom post-processing | P1 | Not started |
| F11.10 | Color grading per biome | P2 | Not started |
| F11.11 | Vignette effect on low HP | P2 | Not started |
| F11.12 | Screen transitions between states | P2 | Not started |
| F11.13 | glTF/GLB asset pipeline for Blender models | P1 | Not started |

### F12: Physics & Collision

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F12.1 | Basic distance-based collision detection | P0 | Complete |
| F12.2 | Proper 2D physics engine integration | P1 | Not started |
| F12.3 | Knockback physics with forces | P1 | Not started |
| F12.4 | Collision layers (player, enemy, projectile, walls) | P1 | Not started |
| F12.5 | Wall/obstacle collision | P1 | Not started |
| F12.6 | Trigger zones for environmental hazards | P2 | Not started |

### F13: Save & Load

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F13.1 | Player profile saved to disk as JSON | P0 | Complete |
| F13.2 | Multiple save slots | P1 | Not started |
| F13.3 | Auto-save on milestones | P0 | Partial |
| F13.4 | Cloud save ready architecture | P2 | Not started |
| F13.5 | Save corruption detection | P2 | Not started |
| F13.6 | Config/settings saved separately | P1 | Not started |

### F14: Networking (Future)

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| F14.1 | Network architecture designed for future multiplayer | P3 | Not started |
| F14.2 | Replication layer for entity state sync | P3 | Not started |
| F14.3 | Lobby/room system | P3 | Not started |
| F14.4 | Online leaderboards | P3 | Not started |

---

## 2. Non-Functional Requirements

### N1: Performance

| ID | Requirement | Target | Priority |
|----|-------------|--------|----------|
| N1.1 | Frame rate minimum | 60 FPS | P0 |
| N1.2 | Frame rate target (high-end) | 144 FPS | P1 |
| N1.3 | Max concurrent entities | 500+ without frame drops | P0 |
| N1.4 | Cold start loading time | < 3 seconds | P1 |
| N1.5 | Save file size | < 100 KB | P0 |
| N1.6 | Runtime memory budget | < 512 MB | P1 |
| N1.7 | Debug build time | < 30 seconds | P1 |
| N1.8 | Release build time | < 3 minutes | P2 |

### N2: Code Quality

| ID | Requirement | Standard | Priority |
|----|-------------|----------|----------|
| N2.1 | No unwrap() calls in release paths | Zero tolerance | P0 |
| N2.2 | All public APIs documented | 100% | P0 |
| N2.3 | Module isolation (no cross-module coupling) | Enforced | P0 |
| N2.4 | Event-driven communication | Standard | P0 |
| N2.5 | Unit test coverage for critical systems | > 80% | P1 |
| N2.6 | Integration tests for gameplay systems | > 60% | P1 |
| N2.7 | Compiler warnings treated as errors | Yes | P0 |
| N2.8 | Clippy lint compliance | Pedantic level | P1 |

### N3: Reliability

| ID | Requirement | Standard | Priority |
|----|-------------|----------|----------|
| N3.1 | No crashes on valid input | Zero tolerance | P0 |
| N3.2 | Graceful error handling on invalid states | Resource fallback | P0 |
| N3.3 | Save file corruption detection | CRC or hash | P2 |
| N3.4 | Auto-backup before save overwrite | 3 most recent | P2 |
| N3.5 | Input validation on all user-facing data | Strict | P0 |

### N4: Portability

| ID | Requirement | Target | Priority |
|----|-------------|--------|----------|
| N4.1 | Windows support | Current target | P0 |
| N4.2 | Linux support | Buildable | P1 |
| N4.3 | macOS support | Buildable | P2 |
| N4.4 | Web (WASM) support | Future target | P3 |
| N4.5 | Controller support | Xbox/PS/Switch layouts | P1 |

### N5: Maintainability

| ID | Requirement | Standard | Priority |
|----|-------------|----------|----------|
| N5.1 | Spec/documentation kept current with implementation | Every session | P0 |
| N5.2 | Changelog updated each session | Mandatory | P0 |
| N5.3 | AGENTS.md updated each session | Mandatory | P0 |
| N5.4 | Data tables separate from system logic | Architectural | P0 |
| N5.5 | Dependency diagram maintained | Current | P1 |
| N5.6 | Migration path documented for breaking changes | Required | P1 |

### N6: Security

| ID | Requirement | Standard | Priority |
|----|-------------|----------|----------|
| N6.1 | No unsafe code blocks without review | Required | P0 |
| N6.2 | No file system access outside save directory | Enforced | P0 |
| N6.3 | No network calls without user consent | Modals | P2 |
| N6.4 | No telemetry or data collection | Opt-in only | P2 |

---

## 3. Requirements Traceability Matrix

| System | Spec Reference | Design Reference | Requirements |
|--------|---------------|------------------|--------------|
| Core/State | spec.md §StateMachine | design.md §13 | F2.1-F2.10 |
| Core/Events | spec.md §EventBus | — | F3.5 |
| Combat | spec.md §Combat | design.md §5 | F3.1-F3.11 |
| Enemies | spec.md §EnemyAI | design.md §6 | F4.1-F4.7 |
| XP/Leveling | spec.md §Upgrades | design.md §2 | F1.6-F1.9 |
| Stats | spec.md §StatSystem | design.md §3 | F6.1-F6.6 |
| Characters | spec.md §Characters | design.md §8 | F8.1-F8.6 |
| Economy | — | design.md §4 | F5.1-F5.10 |
| Maps | spec.md §Maps | design.md §7 | F7.1-F7.7 |
| Items | — | design.md §9 | F5.8, F5.10 |
| Elements | — | design.md §10 | F3.7, F3.8 |
| Audio | — | design.md §12 | F9.1-F9.7 |
| UI | — | design.md §11 | F10.1-F10.15 |
| Camera | spec.md §StateMachine | — | F11.1-F11.9 |
| Physics | — | — | F12.1-F12.6 |
| Save | spec.md §Save | — | F13.1-F13.6 |
| Networking | spec.md §Future | — | F14.1-F14.4 |
| Performance | spec.md §Performance | — | N1.1-N1.8 |
| Quality | spec.md §DesignPrinciples | — | N2.1-N2.8 |
| Reliability | — | — | N3.1-N3.5 |
| Portability | — | — | N4.1-N4.5 |
| Maintainability | spec.md §Metadata | design.md §13.5 | N5.1-N5.6 |
| Security | — | — | N6.1-N6.4 |
