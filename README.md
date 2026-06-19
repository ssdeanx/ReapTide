# ReapTide

**Reap souls. Survive the tides.**

A top-down 2D action roguelite built in Rust + Bevy 0.18. Merge Vampire Survivors' horde-survival loop with Hades' combat depth.

## Status

Pre-production — architecture phase. See [docs/SPEC.md](docs/SPEC.md) for full specification.

## Quick Start

```bash
cargo run
```

Requires: Rust 1.81+, Bevy 0.18 runtime dependencies (Vulkan/DX12/WebGPU).

## Controls

| Key | Action |
|-----|--------|
| WASD / Arrows | Move |
| Space / Shift | Dash |
| Mouse click | Attack / interact |
| Mouse wheel | Zoom |
| Escape | Pause |
| F1 | Controls overlay |
| F11 | Fullscreen toggle |
| R | Restart (game over) |

## Project Structure

```
src/
  core/          — Engine foundation (state machine, plugin system, events)
  gameplay/      — Game logic (player, enemies, combat, maps)
  ui/            — UI framework (screens, widgets, theme)
  audio/         — Audio system (busses, SFX, music)
  characters/    — Character registry & definitions
  achievements/  — Achievement tracking
  shop/          — Shop & permanent upgrades
  save/          — Profile persistence
  assets/        — Asset loading pipeline
```

## Architecture

Each module is a self-contained Bevy `Plugin`. No cross-module coupling. Event-driven communication. See [docs/SPEC.md](docs/SPEC.md).

## License

Proprietary — all rights reserved.
