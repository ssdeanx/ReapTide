use crate::constants::ACHIEVEMENTS;
use crate::save::{award_currency, save_profile, PlayerProfile};
use bevy::prelude::*;

// ---------------------------------------------------------------------------
// Achievement Notification
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct AchievementNotification {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct AchievementChecker {
    pub checked_first_blood: bool,
    pub checked_centurion: bool,
    pub checked_survivor: bool,
    pub checked_level_10: bool,
    pub checked_wave_10: bool,
    pub checked_collector: bool,
    pub checked_rich: bool,
}

// ---------------------------------------------------------------------------
// Check and award achievements
// ---------------------------------------------------------------------------

pub fn check_achievements(
    mut checker: ResMut<AchievementChecker>,
    mut profile: ResMut<PlayerProfile>,
    mut commands: Commands,
    game_stats: Option<Res<crate::gameplay::resources::GameStats>>,
    game_over: Option<Res<crate::gameplay::resources::GameOver>>,
) {
    if let Some(stats) = game_stats {
        if !checker.checked_first_blood && stats.kills >= 1 {
            try_unlock_achievement(&mut profile, "first_blood", &mut commands);
            checker.checked_first_blood = true;
        }

        if !checker.checked_centurion && stats.kills >= 100 {
            try_unlock_achievement(&mut profile, "centurion", &mut commands);
            checker.checked_centurion = true;
        }

        if !checker.checked_survivor && stats.survival_time >= 60.0 {
            try_unlock_achievement(&mut profile, "survivor", &mut commands);
            checker.checked_survivor = true;
        }
    }

    if !checker.checked_rich && profile.total_stats.total_currency_earned >= 1000 {
        try_unlock_achievement(&mut profile, "rich", &mut commands);
        checker.checked_rich = true;
    }

    if !checker.checked_collector && profile.purchased_upgrades.len() >= 3 {
        try_unlock_achievement(&mut profile, "collector", &mut commands);
        checker.checked_collector = true;
    }

    if let Some(go) = game_over {
        if go.active {
            if !checker.checked_level_10 && go.level >= 10 {
                try_unlock_achievement(&mut profile, "level_10", &mut commands);
                checker.checked_level_10 = true;
            }
        }
    }
}

fn try_unlock_achievement(
    profile: &mut PlayerProfile,
    achievement_id: &str,
    commands: &mut Commands,
) {
    if profile.achievements_unlocked.contains(achievement_id) {
        return;
    }

    if let Some(def) = ACHIEVEMENTS.iter().find(|a| a.id == achievement_id) {
        profile.achievements_unlocked.insert(achievement_id.to_string());
        award_currency(profile, def.reward);
        save_profile(profile);

        commands.spawn((
            Text::new(format!("Achievement Unlocked: {} (+{} gold)", def.name, def.reward)),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.85, 0.2)),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                left: Val::Px(10.0),
                ..default()
            },
            AchievementNotification {
                timer: Timer::from_seconds(4.0, TimerMode::Once),
            },
        ));
    }
}

// ---------------------------------------------------------------------------
// Clear expired achievement notifications
// ---------------------------------------------------------------------------

pub fn cleanup_achievement_notifications(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut AchievementNotification)>,
) {
    for (entity, mut notification) in &mut q {
        notification.timer.tick(time.delta());
        if notification.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
