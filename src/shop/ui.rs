use crate::constants::{CHARACTERS, SHOP_ITEMS, ShopCategory};
use crate::save::{save_profile, PlayerProfile};
use bevy::prelude::*;

// ---------------------------------------------------------------------------
// Shop Components
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct ShopRoot;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct ShopItemButton {
    pub item_index: usize,
}

// ---------------------------------------------------------------------------
// Spawn the shop
// ---------------------------------------------------------------------------

pub fn spawn_shop(mut commands: Commands, profile: Res<PlayerProfile>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.04, 0.04, 0.07)),
            ShopRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("SHOP"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
            ));

            // Currency display
            parent.spawn((
                Text::new(format!("Souls: {}", profile.currency)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.4)),
            ));

            // Spacer
            parent.spawn(Node {
                height: Val::Px(20.0),
                ..default()
            });

            // Upgrades section
            parent.spawn((
                Text::new("UPGRADES"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 1.0)),
            ));

            // Upgrade items
            for (i, item) in SHOP_ITEMS
                .iter()
                .filter(|it| matches!(it.category, ShopCategory::Upgrade))
                .enumerate()
            {
                let owned_count = profile.item_purchased(item.id);
                let can_purchase = owned_count < item.max_purchases;
                let affordable = profile.currency >= item.cost;

                let (status_text, bg_color, border_color) = if !can_purchase {
                    ("MAXED".to_string(), Color::srgba(0.1, 0.1, 0.1, 0.95), Color::srgb(0.3, 0.3, 0.3))
                } else if affordable {
                    (
                        format!("Buy ({}) [{}]", item.cost, owned_count),
                        Color::srgba(0.15, 0.15, 0.3, 0.95),
                        Color::srgb(0.3, 0.3, 0.8),
                    )
                } else {
                    (
                        format!("{} (need {})", owned_count, item.cost),
                        Color::srgba(0.1, 0.1, 0.15, 0.95),
                        Color::srgb(0.3, 0.15, 0.15),
                    )
                };

                parent
                    .spawn((
                        Button,
                        BackgroundColor(bg_color),
                        BorderColor::all(border_color),
                        Node {
                            width: Val::Px(400.0),
                            height: Val::Px(40.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            border: UiRect::all(Val::Px(1.0)),
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        ShopItemButton { item_index: i },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("{} - {}", item.name, item.desc)),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::WHITE),
                        ));
                        btn.spawn((
                            Text::new(status_text),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(if can_purchase && affordable {
                                Color::srgb(0.2, 1.0, 0.5)
                            } else {
                                Color::srgb(0.6, 0.6, 0.6)
                            }),
                        ));
                    });
            }

            // Spacer
            parent.spawn(Node {
                height: Val::Px(20.0),
                ..default()
            });

            // Characters section
            parent.spawn((
                Text::new("CHARACTERS"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 1.0, 0.6)),
            ));

            for char_def in CHARACTERS.iter() {
                let owned =
                    profile.character_purchased(char_def.id) || char_def.cost == 0;
                let is_selected = profile.current_character == char_def.id;

                let (status_text, bg_color, border_color) = if is_selected {
                    (
                        "SELECTED".to_string(),
                        Color::srgba(0.2, 0.5, 0.4, 0.95),
                        Color::srgb(char_def.color.0, char_def.color.1, char_def.color.2),
                    )
                } else if owned {
                    (
                        "OWNED".to_string(),
                        Color::srgba(0.15, 0.15, 0.3, 0.95),
                        Color::srgb(0.3, 0.3, 0.3),
                    )
                } else if profile.currency >= char_def.cost {
                    (
                        format!("{} souls", char_def.cost),
                        Color::srgba(0.15, 0.2, 0.15, 0.95),
                        Color::srgb(0.3, 0.8, 0.3),
                    )
                } else {
                    (
                        format!("{} souls", char_def.cost),
                        Color::srgba(0.1, 0.1, 0.15, 0.95),
                        Color::srgb(0.3, 0.15, 0.15),
                    )
                };

                parent
                    .spawn((
                        Button,
                        BackgroundColor(bg_color),
                        BorderColor::all(border_color),
                        Node {
                            width: Val::Px(400.0),
                            height: Val::Px(40.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            border: UiRect::all(Val::Px(1.0)),
                            padding: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        ShopItemButton {
                            item_index: usize::MAX, // signals a character row
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("{} \u{2014} {}", char_def.name, char_def.desc)),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::WHITE),
                        ));
                        btn.spawn((
                            Text::new(status_text),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                    });
            }

            // Spacer
            parent.spawn(Node {
                height: Val::Px(30.0),
                ..default()
            });

            // Back button
            parent
                .spawn((
                    Button,
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.95)),
                    BorderColor::all(Color::srgb(1.0, 0.3, 0.3)),
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackButton,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("BACK TO MENU"),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.3, 0.3)),
                    ));
                });
        });
}

// ---------------------------------------------------------------------------
// Shop interaction
// ---------------------------------------------------------------------------

pub fn handle_shop_interactions(
    interaction_q: Query<
        (&Interaction, Option<&BackButton>, Option<&ShopItemButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_overlay: ResMut<NextState<crate::core::GameOverlayState>>,
    mut profile: ResMut<PlayerProfile>,
    shop_q: Query<Entity, With<ShopRoot>>,
    mut commands: Commands,
) {
    for (interaction, back, item) in &interaction_q {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Back button
        if back.is_some() {
            save_profile(&profile);
            for entity in &shop_q {
                commands.entity(entity).despawn();
            }
            next_overlay.set(crate::core::GameOverlayState::None);
            return;
        }

        // Shop item button
        if let Some(item_btn) = item {
            if item_btn.item_index < SHOP_ITEMS.len() {
                let shop_item = &SHOP_ITEMS[item_btn.item_index];
                let owned_count = profile.item_purchased(shop_item.id);
                if owned_count < shop_item.max_purchases && profile.currency >= shop_item.cost {
                    profile.currency -= shop_item.cost;
                    if let Some(existing) = profile
                        .purchased_upgrades
                        .iter_mut()
                        .find(|(id, _)| id == shop_item.id)
                    {
                        existing.1 += 1;
                    } else {
                        profile.purchased_upgrades.push((shop_item.id.to_string(), 1));
                    }
                    // Refresh shop
                    for entity in &shop_q {
                        commands.entity(entity).despawn();
                    }
                    next_overlay.set(crate::core::GameOverlayState::Shop);
                    return;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Despawn shop
// ---------------------------------------------------------------------------

pub fn despawn_shop(mut commands: Commands, q: Query<Entity, With<ShopRoot>>) {
    for entity in &q {
        commands.entity(entity).despawn();
    }
}
