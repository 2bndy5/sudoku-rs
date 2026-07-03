use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use sudoku_gen::Difficulty;

use super::{
    AppState, BackButton, ButtonBgBundle, ButtonTextBundle, MenuState, button_select,
    menu_screen_root,
};
use crate::board::GameState;

#[derive(Component)]
pub struct DifficultySelector(DifficultyOptions);

#[derive(Debug, Clone, Copy)]
enum DifficultyOptions {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl DifficultyOptions {
    fn as_str(&self) -> &str {
        match self {
            DifficultyOptions::Easy => "Easy",
            DifficultyOptions::Medium => "Medium",
            DifficultyOptions::Hard => "Hard",
            DifficultyOptions::Expert => "Expert",
        }
    }

    fn spawn(self, node: &mut RelatedSpawnerCommands<'_, ChildOf>) {
        let label = self.as_str();
        node.spawn((ButtonBgBundle::default(), DifficultySelector(self)))
            .with_children(|b| {
                b.spawn(ButtonTextBundle::new(label));
            });
    }
}

impl From<DifficultyOptions> for Difficulty {
    fn from(option: DifficultyOptions) -> Self {
        match option {
            DifficultyOptions::Easy => Difficulty::Easy,
            DifficultyOptions::Medium => Difficulty::Medium,
            DifficultyOptions::Hard => Difficulty::Hard,
            DifficultyOptions::Expert => Difficulty::Expert,
        }
    }
}

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::NewGame), setup_new_game_menu)
            .add_systems(Update, button_select.run_if(in_state(MenuState::NewGame)))
            .add_systems(
                Update,
                back_button_listen.run_if(in_state(MenuState::NewGame)),
            )
            .add_systems(Update, start_game.run_if(in_state(MenuState::NewGame)));
    }
}

fn setup_new_game_menu(mut commands: Commands) {
    // Setup the new game menu UI here
    commands
        .spawn((DespawnOnExit(MenuState::NewGame), menu_screen_root()))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(Color::NONE),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Select Difficulty"),
                    TextFont {
                        font_size: FontSize::Px(40.0),
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                ));
                DifficultyOptions::Easy.spawn(parent);
                DifficultyOptions::Medium.spawn(parent);
                DifficultyOptions::Hard.spawn(parent);
                DifficultyOptions::Expert.spawn(parent);
                parent
                    .spawn((ButtonBgBundle::default(), BackButton))
                    .with_child(ButtonTextBundle::new("Back"));
            });
        });
}

fn back_button_listen(
    mut interaction_query: Query<(&Interaction, &BackButton), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, _) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            menu_state.set(MenuState::Main);
        }
    }
}

fn start_game(
    mut interaction_query: Query<(&Interaction, &DifficultySelector), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, difficulty_selector) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            let _difficulty: Difficulty = difficulty_selector.0.into();
            // Here you would typically set up the game with the selected difficulty
            // and transition to the game state.
            menu_state.set(MenuState::Hidden);
            app_state.set(AppState::Game);
            game_state.set(GameState::Loading);
        }
    }
}
