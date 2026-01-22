use bevy::prelude::*;
mod menu;
use menu::{AppState, MenuPlugins};
mod board;
use board::{CachePlugin, GameState, LoadingPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_plugins(MenuPlugins)
        .add_plugins(CachePlugin)
        .add_plugins(LoadingPlugin)
        .run();
}
