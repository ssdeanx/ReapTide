use crate::core::AppState;
use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use crate::gameplay::stats::modifier::StatBundle;
use crate::save::{award_currency, save_profile, PlayerProfile};
use bevy::prelude::*;

// ── Spawn HUD (health bar + text at game start) ──

pub fn spawn_hud(commands: &mut Commands) {
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|parent| {
            // Health bar background
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(18.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 0.9, 0.2)),
                        HealthBarFill,
                    ));
                });

            // HUD text
            parent.spawn((
                Text::new("Lv.1 | XP: 0/10 | HP: 100"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(22.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                HudText,
            ));
        });
}

// ── Update HUD each frame ──
//
// Reads max_health from StatBundle so that modifiers (shop, level-up, etc.)
// are reflected in the health bar and HUD text.

pub fn update_ui(
    player_q: Query<(&Player, &StatBundle)>,
    wave_q: Query<&WaveSpawner>,
    mut bar_q: Query<&mut Node, With<HealthBarFill>>,
    mut text_q: Query<&mut Text, With<HudText>>,
) {
    let Ok((p, stats)) = player_q.single() else {
        return;
    };
    let max_hp = stats.get("max_health");
    let wave = wave_q.single().ok().map_or(0, |s| s.wave);
    if let Ok(mut style) = bar_q.single_mut() {
        let pct = (p.health / max_hp * 100.0).max(0.0);
        style.width = Val::Percent(pct);
    }
    if let Ok(mut text) = text_q.single_mut() {
        let upgrade_indicator = if p.upgrade_chosen {
            match p.chosen_upgrade {
                Some(WeaponUpgrade::AttackSpeed) => " [⚡ Speed]",
                Some(WeaponUpgrade::Damage) => " [🔥 Damage]",
                Some(WeaponUpgrade::Range) => " [🌐 Range]",
                None => "",
            }
        } else {
            ""
        };
        text.sections[0].value = format!(
            "Lv.{} | XP: {}/{} | HP: {:.0} | Wave: {}{}",
            p.level,
            p.xp,
            p.xp_to_next,
            p.health.max(0.0),
            wave,
            upgrade_indicator
        );
    }
}

pub fn update_zoom_display(zoom: Res<ZoomLevel>, mut text_q: Query<&mut Text, With<HudText>>) {
    if let Ok(mut text) = text_q.single_mut() {
        let current = &text.sections[0].value;
        if let Some(base) = current.split(" | Zoom:").next() {
            text.sections[0].value = format!("{} | Zoom: {:.1}x", base.trim_end(), zoom.0);
        } else {
            text.sections[0].value = format!("{} | Zoom: {:.1}x", current.trim_end(), zoom.0);
        }
    }
}

// ── Pause ──

pub fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    upgrade_state: Res<UpgradeState>,
    game_over: Res<GameOver>,
    mut paused: ResMut<Paused>,
) {
    if keys.just_pressed(KeyCode::Escape) && !upgrade_state.active && !game_over.active {
        paused.0 = !paused.0;
    }
}

pub fn manage_pause_overlay(
    mut commands: Commands,
    paused: Res<Paused>,
    q: Query<Entity, With<PauseOverlay>>,
) {
    if paused.0 && q.is_empty() {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                PauseOverlay,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("PAUSED"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.8, 0.2)),
                ));
                parent.spawn((
                    Text::new("Press Escape to resume"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            });
    } else if !paused.0 {
        for entity in &q {
            commands.entity(entity).despawn();
        }
    }
}

// ── Controls Overlay ──

pub fn toggle_controls_overlay(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q: Query<Entity, With<ControlsOverlayRoot>>,
) {
    if keys.just_pressed(KeyCode::F1) {
        if q.is_empty() {
            commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                    ControlsOverlayRoot,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("CONTROLS"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 0.8, 1.0)),
                    ));
                    let controls = [
                        "WASD / Arrow Keys — Move",
                        "Mouse Wheel — Zoom In/Out",
                        "F11 — Toggle Fullscreen",
                        "Escape — Pause/Unpause",
                        "F1 — Close Help",
                        "R — Restart (after death)",
                    ];
                    for line in &controls {
                        parent.spawn((
                            Text::new(*line),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                    }
                    parent.spawn(Node {
                        height: Val::Px(20.0),
                        ..default()
                    });
                    parent.spawn((
                        Text::new("Press F1 to close"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));
                });
        } else {
            for entity in &q {
                commands.entity(entity).despawn();
            }
        }
    }
}

// ── Fullscreen Toggle ──

pub fn toggle_fullscreen(keys: Res<ButtonInput<KeyCode>>, mut window_q: Query<&mut Window>) {
    if keys.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = window_q.single_mut() {
            window.mode = match window.mode {
                bevy::window::WindowMode::Windowed => bevy::window::WindowMode::Fullscreen,
                _ => bevy::window::WindowMode::Windowed,
            };
        }
    }
}

// ── Death Check ──

pub fn check_death(
    mut commands: Commands,
    q: Query<(Entity, &Player)>,
    overlay_q: Query<Entity, With<GameOverOverlay>>,
    mut stats: ResMut<GameStats>,
    mut game_over: ResMut<GameOver>,
    mut next_state: ResMut<NextState<AppState>>,
    mut audio_events: MessageWriter<crate::audio::AudioEvent>,
) {
    if game_over.active {
        return;
    }
    if !overlay_q.is_empty() {
        return;
    }
    let Ok((e, p)) = q.single() else {
        return;
    };
    if p.health <= 0.0 {
        let level = p.level;
        game_over.active = true;
        game_over.level = level;
        game_over.kills = stats.kills;
        game_over.survival_time = stats.survival_time;
        game_over.damage_dealt = stats.damage_dealt;
        game_over.xp_collected = stats.xp_collected;

        next_state.set(AppState::GameOver);
        commands.entity(e).despawn();
        audio_events.write(crate::audio::AudioEvent::GameOver);

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
                GameOverOverlay,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("GAME OVER"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.1, 0.1)),
                ));
                parent.spawn((
                    Text::new(format!("Level Reached: {}", level)),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                parent.spawn((
                    Text::new(format!("Kills: {}", stats.kills)),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
                parent.spawn(Node {
                    height: Val::Px(20.0),
                    ..default()
                });
                parent.spawn((
                    Text::new("Press R to return to Menu"),
                    TextFont {
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.8, 1.0)),
                ));
            });
    }
}

// ── Track Survival Time ──

pub fn track_survival_time(time: Res<Time>, mut stats: ResMut<GameStats>) {
    stats.survival_time += time.delta().as_secs_f64();
}

// ── Game Over: award currency & save, go to menu ──

pub fn handle_game_over_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    game_over: Res<GameOver>,
    all_entities: Query<Entity>,
    mut next_state: ResMut<NextState<AppState>>,
    mut profile: ResMut<PlayerProfile>,
) {
    if !game_over.active {
        return;
    }
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    let reward = game_over.level as u64 * 5 + game_over.kills * 2;
    award_currency(&mut profile, reward);

    profile.total_stats.games_played += 1;
    profile.total_stats.total_kills += game_over.kills;
    if game_over.level > profile.total_stats.highest_level {
        profile.total_stats.highest_level = game_over.level;
    }
    if game_over.survival_time > profile.total_stats.longest_survival_seconds {
        profile.total_stats.longest_survival_seconds = game_over.survival_time;
    }
    profile.total_stats.total_damage_dealt += game_over.damage_dealt;
    profile.total_stats.total_xp_collected += game_over.xp_collected;

    save_profile(&profile);

    for entity in &all_entities {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<ScreenShake>();

    next_state.set(AppState::MainMenu);
}
