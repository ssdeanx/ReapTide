# AGENTS.md — Agent Coordination Guide

---

## Document Metadata

| Field | Value |
|-------|-------|
| Version | 2.0.0 |
| Last Updated | 2026-06-18T11:30:00-04:00 |
| Status | Active |

---

This file defines how AI agents should work on this project across sessions. Load this at the start of every session to understand context, architecture, and next steps.

## Project Identity

**ReapTide** — A top-down 2D action roguelite (Vampire Survivors × Hades). Rust + Bevy 0.18. AAA-quality target. Professional-grade architecture, not a prototype.

## Core Documents

All foundation documents live in `docs/`. These are versioned, timestamped, and updated each session. ALWAYS read the relevant doc before starting work.

| Document | Purpose | Priority |
|----------|---------|----------|
| `docs/spec.md` | Architecture & specification — system architecture, state machine, module map, roadmaps | Always |
| `docs/design.md` | Design decisions — stat system formulas, AI design, economy balance, UI philosophy, migration path | When implementing a system |
| `docs/requirements.md` | Functional + non-functional requirements with traceability matrix | When validating completeness |
| `docs/tasks.md` | Task breakdown with priorities, sizes, dependencies, sprint plan | Every session — this is your todo list |

## Key Conventions

- **No monolithic files** — Every module gets its own directory. Systems, components, resources, and plugins are separated within each module.
- **Plugin isolation** — Each module exports a `Plugin` struct. Cross-module communication happens through Bevy events, not direct system calls.
- **State machine** — Stack-based, hierarchical. Don't add states to a flat enum; use push/pop semantics.
- **Event-driven** — Never call another module's systems directly. Fire events instead.
- **ECS-first** — Data in components, logic in systems, cross-cutting in resources. No singletons, no globals.
- **Data-driven** — Gameplay definitions live in registries/tables, not in system code. Balance changes never touch system code.
- **Modifier stack** — Every stat uses `StatInstance` with `ModifierStack`. No inline multiplication. No magic numbers.

## Architecture Overview

```
src/
  core/          — Foundation (state machine, plugin system, event bus)
  gameplay/      — Game logic (player, enemies, combat, xp, maps, stats, effects)
    stats/       — StatInstance, ModifierStack, StatDefinitions [NEW — critical priority]
    enemies/     — EnemyBrain FSM, behavior sets, enemy type registry [NEW — critical priority]
  ui/            — UI framework (screens, widgets, theme)
  audio/         — Audio (busses, SFX, music)
  characters/    — Character registry & definitions (10+ characters planned)
  achievements/  — Achievement tracking (25+ achievements planned)
  shop/          — Shop & permanent upgrades (12+ items planned)
  save/          — Profile persistence
  assets/        — Asset loading pipeline
```

Each is a Bevy `Plugin` registered in `ReapTidePlugins` in dependency order.

## Current Sprint

**Phase: Foundation — Prototype Stage**

We are building the GRANITE foundation. The following systems are P0/P1 and should be addressed in priority order:

### Critical Priority (This Session Focus)
1. T0.1 — Update Cargo.toml with all required dependencies (physics, particles, lighting, post-processing)
2. T1.1 — Implement StatInstance + ModifierStack (the backbone of all stat calculations)
3. T1.2 — Implement StatDefinitions resource (central stat registry)
4. T2.1 — Implement EnemyBrain FSM component (proper AI state machine)
5. T0.2 — Refactor monolithic files to modular structure
6. T1.3 — Port existing player stats to use ModifierStack
7. T2.2 — Expand enemy types (target: 10 distinct types)

### Current State (v2.0.0 — Foundation)
- [x] Project scaffolding with Bevy 0.18
- [x] Modular architecture structure (core/, gameplay/, ui/, audio/, etc.)
- [x] Core state machine (AppState, GameOverlayState)
- [x] Plugin framework (ReapTidePlugins group)
- [x] Event bus (DamageEvent, KillEvent, XpPickupEvent, LevelUpEvent, etc.)
- [x] Player movement (WASD/arrows, rotation)
- [x] Auto-attack system (nearest-target, projectile spawn)
- [ ] ModifierStack stat system — NOT STARTED
- [ ] EnemyBrain FSM — NOT STARTED
- [x] Basic enemy chase AI (simple movement towards player)
- [x] Wave spawning system (3 types, scaling)
- [x] Combat (projectiles, contact damage, damage types)
- [x] XP magnet + leveling + basic upgrades
- [x] Camera (follow, zoom, shake, overlay)
- [x] Audio (procedural SFX, event-driven)
- [x] UI (menu, HUD, pause, controls, game over)
- [x] Save (JSON persistence)
- [x] 4 characters defined
- [x] 6 shop items defined
- [x] 7 achievements defined
- [ ] Physics engine — NOT STARTED
- [ ] GPU particles — NOT STARTED
- [ ] 2D lighting — NOT STARTED
- [ ] Post-processing — NOT STARTED
- [ ] Map generation — NOT STARTED
- [ ] 10 enemy types — NOT STARTED (only 3)
- [ ] 10 characters — NOT STARTED (only 4)

## Session Handoff Protocol

When starting a session:
1. Read this file (AGENTS.md)
2. Check CHANGELOG.md for recent changes
3. Read `docs/spec.md` for full architecture reference
4. Read `docs/tasks.md` for sprint plan and next tasks
5. Check `docs/design.md` for system-specific design decisions
6. List the current src/ structure to see what exists
7. Begin from the highest-priority incomplete task in `docs/tasks.md`

When ending a session:
1. Update AGENTS.md with completion status (check off completed tasks)
2. Update CHANGELOG.md with all changes made
3. Update `docs/tasks.md` — mark completed tasks, update status, adjust priorities
4. Update `docs/spec.md` if architecture changed
5. Update `docs/design.md` if design decisions changed
6. Leave files in a compilable state
7. Note any blockers or decisions made

## Rules

- **No terminal commands** — Use `write_file` and `patch` exclusively.
- **No Python scripts** — Do not use `execute_code` or spawn subprocesses.
- **No monolithic writes** — Each file has one responsibility.
- **Test with LSP** — Rust-Analyzer provides feedback; don't run `cargo check` unless asked.
- **Document as you go** — All four docs (spec, design, requirements, tasks) stay current.
- **No unwrap() in release paths** — Zero tolerance.
