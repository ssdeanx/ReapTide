use bevy::prelude::*;
use crate::ui::theme::*;
use crate::save::PlayerProfile;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ShopButton;

pub fn spawn_main_menu(mut commands: Commands, profile: Res<PlayerProfile>) {
    commands.spawn((
        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, ..default() },
        BackgroundColor(BG_DARK),
        MenuRoot,
    )).with_children(|parent| {
        parent.spawn((Text::new("REAPTIDE"), TextFont { font_size: 72.0, ..default() }, TextColor(ACCENT_BLUE)));
        parent.spawn(Node { height: Val::Px(40.0), ..default() });
        parent.spawn((Text::new(format!("Souls: {}  |  Best Level: {}", profile.currency, profile.total_stats.highest_level)),
            TextFont { font_size: 18.0, ..default() }, TextColor(ACCENT_GOLD)));
        parent.spawn(Node { height: Val::Px(30.0), ..default() });
        parent.spawn((
            Button,
            BackgroundColor(BG_PANEL),
            BorderColor(ACCENT_BLUE),
            Node { width: Val::Px(300.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
        )).with_children(|btn| {
            btn.spawn((Text::new("START GAME"), TextFont { font_size: 24.0, ..default() }, TextColor(ACCENT_BLUE)));
        });
        parent.spawn(Node { height: Val::Px(16.0), ..default() });
        parent.spawn((
            Button,
            BackgroundColor(BG_PANEL),
            BorderColor(ACCENT_GOLD),
            Node { width: Val::Px(300.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
        )).with_children(|btn| {
            btn.spawn((Text::new("SHOP"), TextFont { font_size: 24.0, ..default() }, TextColor(ACCENT_GOLD)));
        });
    });
}

pub fn handle_menu_interactions(
    interaction_q: Query<(&Interaction, Option<&StartButton>, Option<&ShopButton>), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<crate::core::AppState>>,
    mut next_overlay: ResMut<NextState<crate::core::GameOverlayState>>,
) {
    for (interaction, start, shop) in &interaction_q {
        if *interaction != Interaction::Pressed { continue; }
        if start.is_some() { next_state.set(crate::core::AppState::Playing); return; }
        if shop.is_some() { next_overlay.set(crate::core::GameOverlayState::Shop); return; }
    }
}
