use bevy::prelude::*;

// ── Game State Enum ──
// Hierarchical stack-based state machine.
// Lower-numbered states run beneath higher ones.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    Booting,
    Loading,
    MainMenu,
    Playing,
    GameOver,
}

// ── Sub-State for in-game overlays (pause, shop, settings) ──
// Pushed on top of Playing; pop returns to Playing.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameOverlayState {
    #[default]
    None,
    Paused,
    Shop,
    Settings,
    Achievements,
}

// ── Transition trigger resource ──
// Fired by systems to request a state transition with optional data.

#[derive(Resource, Default)]
pub struct StateTransitionRequest {
    pub target: Option<AppState>,
    pub overlay: GameOverlayState,
}

impl StateTransitionRequest {
    pub fn go(state: AppState) -> Self {
        Self { target: Some(state), overlay: GameOverlayState::None }
    }
    pub fn push_overlay(overlay: GameOverlayState) -> Self {
        Self { target: None, overlay }
    }
    pub fn pop_overlay() -> Self {
        Self { target: None, overlay: GameOverlayState::None }
    }
}
