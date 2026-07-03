use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use sudoku_gen::rand::{rng, seq::SliceRandom};

#[derive(Component, Debug, PartialEq, Eq)]
pub struct LoadingCell {
    pub x: u8,
    pub y: u8,
}

#[derive(Resource, Deref, DerefMut)]
pub struct LoadingAnimationInterval(pub Timer);

#[derive(Resource)]
pub struct LoadingAnimationOrder(Vec<(u8, u8)>);

use super::GameState;

/// Generate a random sequence of (number, cell_index) pairs for the loading animation.
fn generate_random_numbers() -> Vec<(u8, u8)> {
    let mut rand = rng();
    let mut numbers = (1..=9u8).collect::<Vec<u8>>();
    numbers.shuffle(&mut rand);
    let mut indexes = (0..9u8).collect::<Vec<u8>>();
    indexes.shuffle(&mut rand);
    numbers
        .iter()
        .zip(indexes.iter())
        .map(|(&n, &c)| (n, c))
        .collect()
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingAnimationOrder(generate_random_numbers()))
            .add_systems(OnEnter(GameState::Loading), load_animation)
            .add_systems(Update, populate_grid.run_if(in_state(GameState::Loading)));
    }
}

/// Setup the loading screen.
fn load_animation(mut commands: Commands, mut order: ResMut<LoadingAnimationOrder>) {
    commands
        .spawn((
            DespawnOnExit(GameState::Loading),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
        ))
        .with_children(spawn_loading_grid);
    // Previous animations (from other loading screens) may have left the
    // animated number sequence partially used.
    if order.0.len() < 9 {
        // Refresh it if that's the case.
        order.0 = generate_random_numbers();
    }
}

/// Spawn the 3x3 grid for the loading animation.
pub fn spawn_loading_grid(commands: &mut RelatedSpawnerCommands<'_, ChildOf>) {
    commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(300.0),
                display: Display::Grid,
                aspect_ratio: Some(1.0),
                grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                ..Default::default()
            },
            BackgroundColor(
                Srgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.5,
                }
                .into(),
            ),
        ))
        .with_children(|grid| {
            for i in 0..9 {
                let x = i / 3;
                let y = i % 3;
                grid.spawn((
                    LoadingCell {
                        x: x as u8,
                        y: y as u8,
                    },
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    BorderColor {
                        top: if i > 2 { Color::WHITE } else { Color::NONE },
                        bottom: if i < 6 { Color::WHITE } else { Color::NONE },
                        left: if y > 0 { Color::WHITE } else { Color::NONE },
                        right: if y < 2 { Color::WHITE } else { Color::NONE },
                    },
                ));
            }
        });
}

/// Populate the grid cells with numbers in a timed sequence.
pub fn populate_grid(
    mut commands: Commands,
    mut cells: Query<(Entity, &mut LoadingCell)>,
    mut animated_sequence: ResMut<LoadingAnimationOrder>,
    mut timer: ResMut<LoadingAnimationInterval>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Some((number, index)) = animated_sequence.0.pop() {
            let next_cell = LoadingCell {
                x: index / 3,
                y: index % 3,
            };

            for (entity, mut cell) in &mut cells {
                if *cell.as_ref() == next_cell {
                    let mut cmds = commands.entity(entity);
                    cmds.with_child((
                        Text::new(number.to_string()),
                        TextFont {
                            font_size: FontSize::Px(40.0),
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    cell.set_changed();
                }
            }
        } else {
            // If finished animating, then reset
            for (entity, mut cell) in &mut cells {
                // Clear cell text
                let mut cmds = commands.entity(entity);
                cmds.despawn_children();
                cell.set_changed();
            }
            // Reset random number sequence (and corresponding indexes)
            animated_sequence.0 = generate_random_numbers();
        }
    }
}
