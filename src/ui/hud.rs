use bevy::prelude::*;
use crate::gameplay::components::*;
use crate::ui::theme::*;

#[derive(Component)]
pub struct HealthBarRoot;

pub fn spawn_hud(commands: &mut Commands) {
    commands.spawn((Node { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, ..default() },))
    .with_children(|parent| {
        parent.spawn((Node { width: Val::Percent(100.0), height: Val::Px(18.0), ..default() }, BackgroundColor(Color::srgb(0.2, 0.2, 0.2)), HealthBarRoot))
        .with_children(|bar| {
            bar.spawn((Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() }, BackgroundColor(Color::srgb(0.0, 0.9, 0.2)), HealthBarFill));
        });
        parent.spawn((Text::new("Lv.1 | XP: 0/10 | HP: 100"), TextFont { font_size: 18.0, ..default() }, TextColor(TEXT_PRIMARY),
            Node { position_type: PositionType::Absolute, top: Val::Px(22.0), left: Val::Px(10.0), ..default() }, HudText));
    });
}
