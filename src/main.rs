//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_prototype_debug_lines::*;

mod bee_game;
mod menu;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(bee_game::BeeGame)
        .add_plugin(menu::MenuPlugin)
        .run();
}
