use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use super::GameState;
use crate::board::cache::{AppCache, CachedPuzzle};
use crate::board::inputs::{InputsPlugin, spawn_input_panel};
use sudoku_gen::{Cell, Coord};

#[derive(Component)]
pub struct GameBoard;

#[derive(Component)]
pub struct GameContainer;

#[derive(Component)]
pub struct ResponsiveLayout;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputsPlugin)
            .add_systems(OnEnter(GameState::Playing), setup_game_scene);
    }
}

/// Setup the game scene with responsive layout
fn setup_game_scene(mut commands: Commands, cache: Res<AppCache>) {
    // Only render if there is a cached puzzle in memory
    if let Some(cached_puzzle) = cache.unfinished_game.as_ref() {
        // Main game container - responsive
        commands
            .spawn((
                GameContainer,
                ResponsiveLayout,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row, // Start with landscape
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            ))
            .with_children(|parent| {
                // Sudoku Board
                spawn_game_board(parent, cached_puzzle);
                let max_cells = cached_puzzle.puzzle.size.max_cells();
                // Input Controls Panel
                spawn_input_panel(parent, max_cells);
            });
    }
}

/// Spawn the sudoku game board
fn spawn_game_board(
    parent: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>,
    cached_puzzle: &CachedPuzzle,
) {
    let max_cells = cached_puzzle.puzzle.size.max_cells();
    parent
        .spawn((
            GameBoard,
            Node {
                width: Val::Px(400.0),
                height: Val::Px(400.0),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(max_cells as u16, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(max_cells as u16, 1.0),
                aspect_ratio: Some(1.0),
                // border_radius: BorderRadius::all(Val::Px(4.0)),
                // border: UiRect::all(Val::Px(2.0)),
                ..Default::default()
            },
            // BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|board| {
            let region_colors = generate_region_colors(max_cells as usize);
            for row in 0..max_cells as usize {
                for col in 0..max_cells as usize {
                    let cell_text = match cached_puzzle.puzzle.get_cell(Coord { row, col }) {
                        Some(Cell::Value(val)) => (val + 1).to_string(), // Convert 0-indexed to 1-indexed
                        _ => String::new(),
                    };

                    // Get region index to determine shading
                    let region_index = match cached_puzzle
                        .puzzle
                        .kind
                        .index(&cached_puzzle.puzzle.size, Coord { row, col })
                    {
                        Some(idx) => idx,
                        None => continue, // should be unreachable; skip if coord is invalid
                    };

                    let region_bg_color = &region_colors[region_index as usize];

                    board
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                // border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..Default::default()
                            },
                            BorderColor {
                                top: {
                                    if row == 0 {
                                        region_bg_color.to_owned()
                                    } else {
                                        let above_region = cached_puzzle.puzzle.kind.index(
                                            &cached_puzzle.puzzle.size,
                                            Coord { row: row - 1, col },
                                        );
                                        match above_region {
                                            Some(idx) if idx != region_index => {
                                                region_bg_color.to_owned()
                                            }
                                            _ => region_bg_color.with_alpha(0.35),
                                        }
                                    }
                                },
                                bottom: {
                                    if row == max_cells as usize - 1 {
                                        region_bg_color.to_owned()
                                    } else {
                                        let below_region = cached_puzzle.puzzle.kind.index(
                                            &cached_puzzle.puzzle.size,
                                            Coord { row: row + 1, col },
                                        );
                                        match below_region {
                                            Some(idx) if idx != region_index => {
                                                region_bg_color.to_owned()
                                            }
                                            _ => region_bg_color.with_alpha(0.35),
                                        }
                                    }
                                },
                                left: {
                                    if col == 0 {
                                        region_bg_color.to_owned()
                                    } else {
                                        let left_region = cached_puzzle.puzzle.kind.index(
                                            &cached_puzzle.puzzle.size,
                                            Coord { row, col: col - 1 },
                                        );
                                        match left_region {
                                            Some(idx) if idx != region_index => {
                                                region_bg_color.to_owned()
                                            }
                                            _ => region_bg_color.with_alpha(0.35),
                                        }
                                    }
                                },
                                right: {
                                    if col == max_cells as usize - 1 {
                                        region_bg_color.to_owned()
                                    } else {
                                        let right_region = cached_puzzle.puzzle.kind.index(
                                            &cached_puzzle.puzzle.size,
                                            Coord { row, col: col + 1 },
                                        );
                                        match right_region {
                                            Some(idx) if idx != region_index => {
                                                region_bg_color.to_owned()
                                            }
                                            _ => region_bg_color.with_alpha(0.35),
                                        }
                                    }
                                },
                            },
                            BackgroundColor(region_bg_color.with_alpha(0.15)),
                            Button,
                        ))
                        .with_child((
                            Text::new(cell_text),
                            TextFont {
                                font_size: FontSize::Px(24.0),
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        ));
                }
            }
        });
}

fn generate_region_colors(size: usize) -> Vec<Color> {
    use sudoku_gen::rand::{rng, seq::SliceRandom};

    let mut rand = rng();

    let mut colors = Vec::with_capacity(size);
    for i in 0..size {
        // Generate a color based on the region index
        let hue = (i as f32 / size as f32) * 360.0;
        let color = Color::hsl(hue, 0.5, 0.5);
        colors.push(color);
    }

    colors.shuffle(&mut rand);
    colors
}
