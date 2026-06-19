use crate::constants::{WeaponUpgrade, ATTACK_DAMAGE, ATTACK_INTERVAL, ATTACK_RANGE, UPGRADE_ATTACK_SPEED_MULT, UPGRADE_DAMAGE_MULT, UPGRADE_RANGE_MULT};
use crate::gameplay::components::*;
use crate::gameplay::resources::*;
use bevy::prelude::*;

pub fn spawn_upgrade_menu(
    mut commands: Commands,
    upgrade_state: Res<UpgradeState>,
    menu_q: Query<Entity, With<UpgradeMenuRoot>>,
    player_q: Query<&Player>,
) {
    if !upgrade_state.active { return; }
    if !menu_q.is_empty() { return; }
    let Ok(_p) = player_q.single() else { return; };

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
            UpgradeMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LEVEL UP! — Choose a Weapon Evolution"),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::srgb(1.0, 1.0, 0.4)),
            ));
            parent.spawn(Node { height: Val::Px(30.0), ..default() });

            spawn_upgrade_btn(parent, WeaponUpgrade::AttackSpeed, "⚡ +50% Attack Speed", "Fire 50% faster!", Color::srgb(1.0, 0.85, 0.2));
            parent.spawn(Node { height: Val::Px(16.0), ..default() });
            spawn_upgrade_btn(parent, WeaponUpgrade::Damage, "🔥 +50% Damage", "Each shot deals 50% more!", Color::srgb(1.0, 0.3, 0.1));
            parent.spawn(Node { height: Val::Px(16.0), ..default() });
            spawn_upgrade_btn(parent, WeaponUpgrade::Range, "🌐 +50% Range", "Hit enemies from farther away!", Color::srgb(0.2, 0.6, 1.0));
        });
}

fn spawn_upgrade_btn(parent: &mut ChildBuilder, upgrade: WeaponUpgrade, title: &str, desc: &str, color: Color) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(320.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.95)),
            BorderColor(color),
            UpgradeChoiceButton { upgrade },
        ))
        .with_children(|btn| {
            btn.spawn((Text::new(title), TextFont { font_size: 20.0, ..default() }, TextColor(color)));
            btn.spawn((Text::new(desc), TextFont { font_size: 14.0, ..default() }, TextColor(Color::srgb(0.8, 0.8, 0.8))));
        });
}

pub fn handle_upgrade_choice(
    mut commands: Commands,
    interaction_q: Query<(&Interaction, &UpgradeChoiceButton), (Changed<Interaction>, With<Button>)>,
    mut player_q: Query<&mut Player>,
    mut upgrade_state: ResMut<UpgradeState>,
    menu_q: Query<Entity, With<UpgradeMenuRoot>>,
) {
    if !upgrade_state.active { return; }
    for (interaction, choice) in &interaction_q {
        if *interaction == Interaction::Pressed {
            let Ok(mut p) = player_q.single_mut() else { return; };

            match choice.upgrade {
                WeaponUpgrade::AttackSpeed => {
                    p.attack_timer = Timer::from_seconds(ATTACK_INTERVAL / UPGRADE_ATTACK_SPEED_MULT, TimerMode::Repeating);
                }
                WeaponUpgrade::Damage => {
                    p.attack_damage = ATTACK_DAMAGE * UPGRADE_DAMAGE_MULT;
                }
                WeaponUpgrade::Range => {
                    p.attack_range = ATTACK_RANGE * UPGRADE_RANGE_MULT;
                }
            }

            p.upgrade_chosen = true;
            p.chosen_upgrade = Some(choice.upgrade);
            upgrade_state.active = false;

            for entity in &menu_q { commands.entity(entity).despawn(); }
            break;
        }
    }
}
