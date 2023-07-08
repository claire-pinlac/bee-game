//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

mod bee_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(AudioPlugin)
        .add_plugin(bee_game::BeeGame)
        .run();
}
