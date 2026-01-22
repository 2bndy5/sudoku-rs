use bevy::prelude::*;
pub mod cache;
pub use cache::CachePlugin;
pub mod loading;
pub use loading::LoadingPlugin;

#[allow(dead_code)]
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Loading,
    Playing,
    Paused,
    #[default]
    Ended,
}
