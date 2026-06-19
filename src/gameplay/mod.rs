use bevy::prelude::*;

pub mod camera;
pub mod combat;
pub mod components;
pub mod enemies;
pub mod particles;
pub mod player;
pub mod resources;
pub mod stats;
pub mod ui;
pub mod upgrades;
pub mod xp;
pub use components::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(stats::StatPlugin)
            .init_resource::<crate::gameplay::resources::GameStats>()
            .init_resource::<crate::gameplay::resources::Paused>()
            .init_resource::<crate::gameplay::resources::ZoomLevel>()
            .init_resource::<crate::gameplay::resources::GameOver>()
            .init_resource::<crate::gameplay::resources::UpgradeState>()
            // Initialise shared meshes/materials once at startup so they exist
            // before any Playing-state system tries to use them.
            .add_systems(
                Startup,
                (resources::init_game_resources, camera::spawn_scene),
            )
            // Gameplay startup — spawns player + HUD when entering Playing state
            .add_systems(OnEnter(crate::core::AppState::Playing), setup_gameplay)
            // Player systems
            .add_systems(
                Update,
                player::player_movement.run_if(in_state(crate::core::AppState::Playing)),
            )
            // Enemy systems
            .add_systems(
                Update,
                enemies::spawn_enemies.run_if(in_state(crate::core::AppState::Playing)),
            )
            // Enemy brain FSM must run before chase so state is current
            .add_systems(
                Update,
                enemies::brain::update_enemy_brain.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                enemies::enemies_chase.run_if(in_state(crate::core::AppState::Playing)),
            )
            // Combat systems
            .add_systems(
                Update,
                combat::auto_attack.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                combat::update_projectiles.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                combat::enemy_death.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                combat::contact_damage.run_if(in_state(crate::core::AppState::Playing)),
            )
            // XP systems
            .add_systems(
                Update,
                xp::magnet_xp.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                xp::collect_xp.run_if(in_state(crate::core::AppState::Playing)),
            )
            // Upgrade systems
            .add_systems(
                Update,
                upgrades::spawn_upgrade_menu.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                upgrades::handle_upgrade_choice.run_if(in_state(crate::core::AppState::Playing)),
            )
            // Particle systems
            .add_systems(
                Update,
                particles::update_particles.run_if(in_state(crate::core::AppState::Playing)),
            )
            // Camera systems
            .add_systems(
                Update,
                camera::camera_follow_and_zoom.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                camera::update_zoom.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                camera::camera_shake.run_if(in_state(crate::core::AppState::Playing)),
            )
            // UI systems (in-game)
            .add_systems(
                Update,
                ui::update_ui.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::update_zoom_display.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::toggle_pause.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::manage_pause_overlay.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::toggle_controls_overlay.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::toggle_fullscreen.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::check_death.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::track_survival_time.run_if(in_state(crate::core::AppState::Playing)),
            )
            .add_systems(
                Update,
                ui::handle_game_over_input.run_if(in_state(crate::core::AppState::GameOver)),
            );
    }
}

/// Set up gameplay state and spawn the player + HUD when the game starts.
///
/// Resources (GameMeshes, GameMaterials, PlayerProfile, StatDefinitions) are
/// injected by Bevy's system parameter plumbing — they must be initialised by
/// their respective plugins before this system runs.
fn setup_gameplay(
    mut commands: Commands,
    meshes: Res<resources::GameMeshes>,
    materials: Res<resources::GameMaterials>,
    profile: Res<crate::save::PlayerProfile>,
    stat_defs: Res<crate::gameplay::stats::StatDefinitions>,
) {
    commands.insert_resource(PlayerState::default());
    commands.insert_resource(WaveState::default());
    player::spawn_player(&mut commands, &meshes, &materials, &profile, &stat_defs);
    ui::spawn_hud(&mut commands);
}
