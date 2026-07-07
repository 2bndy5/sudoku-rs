use bevy::prelude::*;
use bevy::{ecs::relationship::RelatedSpawnerCommands, window::WindowResized};

use crate::board::cache::AppCache;

use super::GameState;
use super::scene::ResponsiveLayout;

const LANDSCAPE_THRESHOLD: f32 = 1.2; // aspect ratio threshold (width/height)

#[derive(Component)]
pub struct InputPanel;

#[derive(Component)]
pub struct NumberGrid;

#[derive(Component)]
pub struct ActionButtonsContainer;

/// Marker for input buttons
#[derive(Component)]
#[allow(dead_code)]
pub struct InputButton(pub InputAction);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputAction {
    Number(u8),
    Undo,
    Erase,
    PencilMode,
    Hint,
    Check,
}

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_responsive_layout.run_if(in_state(GameState::Playing)),
        );
    }
}

/// Spawn the input controls panel
pub fn spawn_input_panel(
    parent: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>,
    max_cells: u8,
) {
    parent
        .spawn((
            InputPanel,
            Node {
                flex_direction: FlexDirection::Row, // Default to vertical for landscape
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                column_gap: Val::Px(15.0),
                ..Default::default()
            },
        ))
        .with_children(|panel| {
            // Number grid
            spawn_number_grid(panel, max_cells);
            // Action buttons
            spawn_action_buttons(panel);
        });
}

/// Spawn the number grid (responsive grid layout)
fn spawn_number_grid(
    panel: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>,
    max_cells: u8,
) {
    panel
        .spawn((
            NumberGrid,
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::max_content(2),
                grid_template_rows: RepeatedGridTrack::max_content(
                    (max_cells as f32 / 2.0).ceil() as u16
                ),
                ..Default::default()
            },
        ))
        .with_children(|grid| {
            let (start, end) = if max_cells <= 9 {
                (1, max_cells + 1)
            } else {
                (0, max_cells)
            };
            for num in start..end {
                spawn_button(grid, &format!("{}", num), InputAction::Number(num));
            }
        });
}

/// Spawn the action buttons container (responsive flex layout)
fn spawn_action_buttons(panel: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>) {
    panel
        .spawn((
            ActionButtonsContainer,
            Node {
                flex_direction: FlexDirection::Column, // Default to landscape
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(5.0),
                row_gap: Val::Px(5.0),
                ..Default::default()
            },
        ))
        .with_children(|container| {
            spawn_button(container, "Undo", InputAction::Undo);
            spawn_button(container, "Erase", InputAction::Erase);
            spawn_button(container, "Pencil", InputAction::PencilMode);
            spawn_button(container, "Hint", InputAction::Hint);
            spawn_button(container, "Check", InputAction::Check);
        });
}

/// Spawn a single input button
fn spawn_button(
    parent: &mut RelatedSpawnerCommands<'_, bevy::prelude::ChildOf>,
    label: &str,
    action: InputAction,
) {
    parent
        .spawn((
            InputButton(action),
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..Default::default()
            },
            BorderColor::all(Color::srgb(0.6, 0.8, 1.0)),
            BackgroundColor(Color::srgb(0.2, 0.4, 0.6)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Update the responsive layout based on window size
#[allow(
    clippy::type_complexity,
    reason = "Need to query several nodes for layout updates"
)]
fn update_responsive_layout(
    mut window_event: MessageReader<WindowResized>,
    mut queries: ParamSet<(
        Query<&mut Node, With<ResponsiveLayout>>,
        Query<&mut Node, With<InputPanel>>,
        Query<&mut Node, With<NumberGrid>>,
        Query<&mut Node, With<ActionButtonsContainer>>,
    )>,
    cache: Res<AppCache>,
) {
    let max_cells = match cache.unfinished_game.as_ref() {
        Some(cached_puzzle) => cached_puzzle.puzzle.size.max_cells(),
        None => return, // No game in progress, skip layout update
    };
    for event in window_event.read() {
        let aspect_ratio = event.width / event.height;
        let is_landscape = aspect_ratio > LANDSCAPE_THRESHOLD;

        // Update game container layout
        if let Ok(mut container_node) = queries.p0().single_mut() {
            if is_landscape {
                // Landscape: board and inputs side-by-side (row)
                container_node.flex_direction = FlexDirection::Row;
            } else {
                // Portrait: board above inputs (column)
                container_node.flex_direction = FlexDirection::Column;
            }
        }

        // Update input panel layout
        if let Ok(mut panel_node) = queries.p1().single_mut() {
            if is_landscape {
                // Landscape: number grid and action buttons side-by-side (row)
                panel_node.flex_direction = FlexDirection::Row;
            } else {
                // Portrait: number grid above action buttons (column)
                panel_node.flex_direction = FlexDirection::Column;
            }
        }

        // Update number grid layout
        if let Ok(mut grid_node) = queries.p2().single_mut() {
            let max_track_cells = (max_cells as f32 / 2.0).ceil() as u16;
            if is_landscape {
                // Landscape: 2 columns, 5 rows
                grid_node.grid_template_columns = RepeatedGridTrack::max_content(2);
                grid_node.grid_template_rows = RepeatedGridTrack::max_content(max_track_cells);
            } else {
                // Portrait: 5 columns, 2 rows
                grid_node.grid_template_columns = RepeatedGridTrack::max_content(max_track_cells);
                grid_node.grid_template_rows = RepeatedGridTrack::max_content(2);
            }
        }

        // Update action buttons layout
        if let Ok(mut action_node) = queries.p3().single_mut() {
            if is_landscape {
                // Landscape: buttons in a column
                action_node.flex_direction = FlexDirection::Column;
            } else {
                // Portrait: buttons in a row
                action_node.flex_direction = FlexDirection::Row;
            }
        }
    }
}
