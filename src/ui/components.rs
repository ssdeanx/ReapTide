use bevy::prelude::*;

// Reusable UI button builder
pub fn spawn_button(parent: &mut ChildSpawner, text: &str, font_size: f32, color: Color, bg: Color) {
    parent.spawn((
        Button,
        Node { width: Val::Px(300.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
        BackgroundColor(bg),
        BorderColor(color),
    )).with_children(|btn| {
        btn.spawn((Text::new(text), TextFont { font_size, ..default() }, TextColor(color)));
    });
}

// Reusable panel
pub fn spawn_panel(parent: &mut ChildSpawner, width: Val, height: Val, bg: Color) {
    parent.spawn((Node { width, height, ..default() }, BackgroundColor(bg)));
}
