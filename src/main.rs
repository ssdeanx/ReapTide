mod core;
mod constants;
mod save;
mod audio;
mod assets;
mod characters;
mod achievements;
mod shop;
mod gameplay;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            core::plugin::ReapTidePlugins,
        ))
        .run();
}
