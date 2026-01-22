use std::{fs, str::FromStr};

use bevy::prelude::*;
use serde::{Deserialize, Serialize, ser::SerializeStruct};

use sudoku_gen::{Board, Difficulty};

pub struct CachePlugin;

impl Plugin for CachePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppCache>()
            .add_systems(Startup, load_cache);
    }
}

/// Resource to cache the current game state.
#[derive(Resource, Default)]
pub struct AppCache {
    /// The current unfinished game, if any.
    pub unfinished_game: Option<CachedPuzzle>,
}

/// Cached data for an ongoing Sudoku game.
pub struct CachedPuzzle {
    /// The current state of the Sudoku board.
    pub puzzle: Board,

    /// The number of seconds elapsed since the start of the game.
    pub elapsed_time: u32,

    /// The puzzle's completed solution.
    pub solution: Board,

    /// The number of errors made by the player.
    pub errors: u32,

    /// The selected difficulty level of the puzzle.
    pub difficulty: Difficulty,
}

fn load_cache(mut cache: ResMut<AppCache>) {
    if let Ok(cached_data) = fs::read_to_string("game_cache.json")
        && let Ok(cached_data) = serde_json::from_str::<CachedPuzzle>(&cached_data)
    {
        cache.unfinished_game = Some(cached_data);
    }
}

impl Serialize for CachedPuzzle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CachedPuzzle", 5)?;
        state.serialize_field("puzzle", &self.puzzle.to_string())?;
        state.serialize_field("solution", &self.solution.to_string())?;
        state.serialize_field("elapsed_time", &self.elapsed_time)?;
        state.serialize_field("errors", &self.errors)?;
        state.serialize_field("difficulty", self.difficulty.as_str())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for CachedPuzzle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CachedPuzzleHelper {
            puzzle: String,
            solution: String,
            elapsed_time: u32,
            errors: u32,
            difficulty: String,
        }

        let helper = CachedPuzzleHelper::deserialize(deserializer)?;
        Ok(CachedPuzzle {
            puzzle: Board::from_str(&helper.puzzle).map_err(serde::de::Error::custom)?,
            solution: Board::from_str(&helper.solution).map_err(serde::de::Error::custom)?,
            elapsed_time: helper.elapsed_time,
            errors: helper.errors,
            difficulty: Difficulty::from_str(&helper.difficulty)
                .map_err(|_| serde::de::Error::custom("Failed to parse difficulty from string"))?,
        })
    }
}
