use crate::gameplay::components::*;
use bevy::prelude::*;

// ── AI State Enum ──
//
// Represents the current behavioral state of an enemy entity.
// Transitions are evaluated each frame by the `update_enemy_brain` system.
// The FSM is timer-driven with perception-based triggers.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AiState {
    /// Stationary, waiting for stimulus (player in sight range)
    Idle,
    /// Following a predefined path between waypoints (not yet implemented)
    Patrol,
    /// Detected the player — brief hesitation before committing to chase
    Alert,
    /// Actively pursuing the player's last known position
    #[default]
    Chase,
    /// In melee range, delivering contact damage
    Attack,
    /// Retreating from the player (low health, damage fear)
    Flee,
    /// Temporarily incapacitated (hit reaction, knockback stun)
    Stunned,
    /// Performing a unique ability (e.g., summon, charge wind-up)
    Special,
    /// Final state before entity despawn
    Dead,
}

// ── Perception ──
//
/// Defines how an enemy senses the world around it.
/// These values are tuned per enemy type for varied behavior.
#[derive(Clone, Debug)]
pub struct Perception {
    /// Maximum distance at which the enemy can detect the player
    pub sight_range: f32,
    /// Maximum distance for hearing-based detection (e.g., nearby combat)
    pub hearing_range: f32,
    /// How long (seconds) the enemy remembers the player after losing line of sight
    pub memory_duration: f32,
}

impl Default for Perception {
    fn default() -> Self {
        Self {
            sight_range: 500.0,
            hearing_range: 300.0,
            memory_duration: 3.0,
        }
    }
}

// ── Enemy Memory ──
//
/// Tracks the enemy's knowledge of the player position over time.
/// Used to give enemies a "hunting" behavior when the player breaks line of sight.
#[derive(Clone, Debug)]
pub struct EnemyMemory {
    /// Where the enemy last saw or heard the player
    pub last_known_position: Option<Vec2>,
    /// Game time (elapsed_secs) when the player was last sensed
    pub last_seen_time: f32,
    /// Cooldown timer for re-entering Alert state after losing the player
    pub alert_cooldown: Timer,
}

impl Default for EnemyMemory {
    fn default() -> Self {
        Self {
            last_known_position: None,
            last_seen_time: 0.0,
            alert_cooldown: Timer::from_seconds(1.5, TimerMode::Once),
        }
    }
}

// ── EnemyBrain Component ──
//
/// ECS component that drives enemy behavior through a state machine (FSM).
/// Attach to any enemy entity to give it AI-driven behavior.
///
/// The FSM is evaluated each frame by `update_enemy_brain`, which:
/// 1. Checks player visibility using sight_range
/// 2. Updates enemy memory (last known position, seen time)
/// 3. Evaluates state transitions based on current state + perception
/// 4. Updates `state` and `state_timer` accordingly
///
/// Movement itself is handled by `enemies_chase`, which reads the brain state
/// and only moves the enemy when it is in an aggressive state (Chase, Attack).
#[derive(Component, Clone)]
pub struct EnemyBrain {
    /// Current FSM state — determines behavior
    pub state: AiState,
    /// Countdown timer for timed states (stun duration, alert delay, flee duration)
    pub state_timer: Timer,
    /// How the enemy detects the player
    pub perception: Perception,
    /// What the enemy remembers about the player
    pub memory: EnemyMemory,
}

impl Default for EnemyBrain {
    fn default() -> Self {
        Self {
            // Default to Chase for backward compatibility with existing spawn code
            state: AiState::Chase,
            state_timer: Timer::from_seconds(0.0, TimerMode::Once),
            perception: Perception::default(),
            memory: EnemyMemory::default(),
        }
    }
}

impl EnemyBrain {
    /// Creates a new EnemyBrain in Chase state with the given sight range.
    /// This is the standard constructor for most enemies.
    pub fn new(sight_range: f32) -> Self {
        Self {
            perception: Perception {
                sight_range,
                ..default()
            },
            ..default()
        }
    }

    /// Creates a new EnemyBrain in Idle state with a periodic idle timer.
    /// Use for enemies that should stand still until the player is detected.
    pub fn idle(sight_range: f32) -> Self {
        Self {
            state: AiState::Idle,
            state_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            perception: Perception {
                sight_range,
                ..default()
            },
            ..default()
        }
    }

    /// Returns true if the enemy is aware of the player and actively engaged
    /// (Chase, Attack, or Alert states).
    pub fn is_aggressive(&self) -> bool {
        matches!(
            self.state,
            AiState::Chase | AiState::Attack | AiState::Alert
        )
    }

    /// Returns true if the enemy cannot act (stunned, dead, or performing a special action).
    pub fn is_immobilized(&self) -> bool {
        matches!(
            self.state,
            AiState::Stunned | AiState::Dead | AiState::Special
        )
    }

    /// Transition to a new state and reset the state timer to the given duration.
    fn transition_to(&mut self, new_state: AiState, timer_duration: f32) {
        self.state = new_state;
        self.state_timer = Timer::from_seconds(timer_duration, TimerMode::Once);
    }
}

// ── FSM Driver System ──
//
/// Main AI FSM driver — evaluates perception and state transitions for all enemies
/// that have an EnemyBrain component. Runs each frame during the Playing state.
///
/// Transition rules:
///   Idle    → Alert   (player enters sight range)
///   Idle    → Idle    (timer resets — waiting)
///   Alert   → Chase   (alert delay expired)
///   Alert   → Idle    (player lost before committing)
///   Chase   → Attack  (player in melee range)
///   Chase   → Idle    (player lost, memory expired)
///   Attack  → Chase   (player leaves melee range but is visible/in memory)
///   Attack  → Idle    (player leaves melee range and lost/memory expired)
///   Stunned → Chase   (stun duration expired, player visible/in memory)
///   Stunned → Idle    (stun duration expired, player lost)
pub fn update_enemy_brain(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(&Transform, &Enemy, &mut EnemyBrain)>,
) {
    let elapsed = time.elapsed_secs();
    let dt = time.delta().as_secs_f32();

    // Get player position if they exist
    let player_pos = player_q.single().ok().map(|tf| tf.translation.truncate());

    for (tf, _enemy, mut brain) in &mut enemy_q {
        let enemy_pos = tf.translation.truncate();
        brain.state_timer.tick(time.delta());

        // ── Perception Check ──
        //
        // Calculate distance to player and update enemy memory.
        let dist_to_player = player_pos.map(|pp| pp.distance(enemy_pos));

        if let Some(dist) = dist_to_player {
            if dist <= brain.perception.sight_range {
                // Player is within sight range — refresh memory
                brain.memory.last_known_position = player_pos;
                brain.memory.last_seen_time = elapsed;
                brain.memory.alert_cooldown.reset();
            }
        }

        // Derived perception booleans for use in state transitions
        let player_visible = dist_to_player
            .map(|d| d <= brain.perception.sight_range)
            .unwrap_or(false);

        let player_in_memory = brain.memory.last_known_position.is_some()
            && (elapsed - brain.memory.last_seen_time) <= brain.perception.memory_duration;

        let player_in_attack_range = dist_to_player
            .map(|d| d <= crate::constants::CONTACT_RANGE)
            .unwrap_or(false);

        // ── State Transitions ──
        //
        // Each state has defined transitions. Only one transition fires per evaluation.
        // States not explicitly handled below retain their current state by default.
        match brain.state {
            AiState::Idle => {
                if player_visible {
                    // Player spotted — brief alert delay before committing to chase
                    brain.transition_to(AiState::Alert, 0.5);
                } else if brain.state_timer.just_finished() {
                    // Idle pulse: remain idle but keep scanning
                    brain.state_timer.reset();
                }
            }

            AiState::Alert => {
                if brain.state_timer.just_finished() {
                    // Alert delay expired — commit to chase direction
                    brain.transition_to(AiState::Chase, 0.0);
                }
                // If player disappears during alert hesitation, back to idle
                if !player_visible && !player_in_memory {
                    brain.transition_to(AiState::Idle, 2.0);
                }
            }

            AiState::Chase => {
                if player_in_attack_range {
                    // Reached melee distance — start attacking
                    brain.transition_to(AiState::Attack, 0.0);
                } else if !player_visible && !player_in_memory {
                    // Lost the player entirely — give up and return to idle
                    brain.transition_to(AiState::Idle, 2.0);
                }
            }

            AiState::Attack => {
                if !player_in_attack_range {
                    // Player moved out of melee range — resume chase
                    if player_visible || player_in_memory {
                        brain.transition_to(AiState::Chase, 0.0);
                    } else {
                        brain.transition_to(AiState::Idle, 2.0);
                    }
                }
            }

            AiState::Stunned => {
                if brain.state_timer.just_finished() {
                    // Stun wore off — re-engage or idle
                    if player_visible || player_in_memory {
                        brain.transition_to(AiState::Chase, 0.0);
                    } else {
                        brain.transition_to(AiState::Idle, 2.0);
                    }
                }
            }

            AiState::Flee => {
                if brain.state_timer.just_finished() {
                    // Flee duration expired — reassess
                    if player_visible || player_in_memory {
                        brain.transition_to(AiState::Chase, 0.0);
                    } else {
                        brain.transition_to(AiState::Idle, 2.0);
                    }
                }
            }

            // Patrol, Special, Dead — no automatic transitions yet (placeholder)
            AiState::Patrol | AiState::Special | AiState::Dead => {}
        }
    }
}

// ── Stun Helper ──
//
/// Applies a stun to an enemy, interrupting current behavior.
/// Call from damage/kill systems when an enemy takes a hit.
/// Uses the state_timer to track stun duration.
pub fn apply_stun(mut brain: &mut EnemyBrain, duration: f32) {
    brain.transition_to(AiState::Stunned, duration);
}

// ── Unit Tests ──
//
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a newly created EnemyBrain defaults to Chase state
    /// and is aggressive by default.
    #[test]
    fn test_brain_defaults_to_chase() {
        let brain = EnemyBrain::new(500.0);
        assert_eq!(brain.state, AiState::Chase);
        assert!(brain.is_aggressive());
        assert!(!brain.is_immobilized());
    }

    /// Verifies that idle-created brains start non-aggressive
    /// and become aggressive only after detecting the player.
    #[test]
    fn test_idle_brain_not_aggressive() {
        let brain = EnemyBrain::idle(500.0);
        assert_eq!(brain.state, AiState::Idle);
        assert!(!brain.is_aggressive());
    }

    /// Verifies that is_immobilized returns true for Stunned and Dead states.
    #[test]
    fn test_immobilized_states() {
        let mut brain = EnemyBrain::new(500.0);
        brain.state = AiState::Stunned;
        assert!(brain.is_immobilized());
        brain.state = AiState::Dead;
        assert!(brain.is_immobilized());
        brain.state = AiState::Special;
        assert!(brain.is_immobilized());
    }

    /// Verifies that transition_to changes the state and resets the timer.
    #[test]
    fn test_transition_to_changes_state_and_timer() {
        let mut brain = EnemyBrain::new(500.0);
        brain.transition_to(AiState::Stunned, 2.0);
        assert_eq!(brain.state, AiState::Stunned);
        assert!(!brain.state_timer.finished());
        // After ticking past the duration, timer should be finished
        brain.state_timer.tick(Duration::from_secs_f32(2.1));
        assert!(brain.state_timer.just_finished());
    }

    /// Verifies that is_aggressive returns true for all aggressive states.
    #[test]
    fn test_aggressive_states() {
        let mut brain = EnemyBrain::new(500.0);
        for state in &[AiState::Chase, AiState::Attack, AiState::Alert] {
            brain.state = *state;
            assert!(
                brain.is_aggressive(),
                "State {:?} should be aggressive",
                state
            );
        }
    }

    /// Verifies that apply_stun sets the state to Stunned with the given duration.
    #[test]
    fn test_apply_stun() {
        let mut brain = EnemyBrain::new(500.0);
        assert_eq!(brain.state, AiState::Chase);
        apply_stun(&mut brain, 1.5);
        assert_eq!(brain.state, AiState::Stunned);
        assert!(brain.is_immobilized());
    }

    /// Verifies that memory tracking works correctly.
    #[test]
    fn test_enemy_memory_defaults() {
        let memory = EnemyMemory::default();
        assert!(memory.last_known_position.is_none());
        assert_eq!(memory.last_seen_time, 0.0);
    }

    /// Verifies that perception defaults are reasonable.
    #[test]
    fn test_perception_defaults() {
        let perception = Perception::default();
        assert_eq!(perception.sight_range, 500.0);
        assert_eq!(perception.hearing_range, 300.0);
        assert_eq!(perception.memory_duration, 3.0);
    }
}
