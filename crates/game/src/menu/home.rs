use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use super::{
    AppState, ButtonBgBundle, ButtonTextBundle, MenuState, button_select, menu_screen_root,
};
use crate::board::{GameState, cache::AppCache};

/// Component to identify main menu's buttons.
#[derive(Component)]
pub struct MenuButton(MenuButtonType);

/// Different types of buttons in the main menu.
#[derive(Copy, Clone)]
pub enum MenuButtonType {
    NewGame,
    Continue,
    Settings,
    Exit,
}

impl MenuButtonType {
    fn as_str(&self) -> &str {
        match self {
            MenuButtonType::NewGame => "New Game",
            MenuButtonType::Continue => "Continue",
            MenuButtonType::Settings => "Settings",
            MenuButtonType::Exit => "Exit",
        }
    }

    pub fn spawn(self, node: &mut RelatedSpawnerCommands<'_, ChildOf>) {
        let label = self.as_str();
        node.spawn((ButtonBgBundle::default(), MenuButton(self)))
            .with_children(|b| {
                b.spawn(ButtonTextBundle::new(label));
            });
    }

    /// System to handle main menu button interactions
    fn action(
        mut interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut app_state: ResMut<NextState<AppState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
        cached_game: Res<AppCache>,
    ) {
        for (interaction, menu_button) in &mut interaction_query {
            if matches!(*interaction, Interaction::Pressed) {
                let button_type = menu_button.0;
                match button_type {
                    MenuButtonType::NewGame => {
                        menu_state.set(MenuState::NewGame);
                    }
                    MenuButtonType::Continue => {
                        if cached_game.unfinished_game.is_some() {
                            app_state.set(AppState::Game);
                            game_state.set(GameState::Loading);
                        }
                    }
                    MenuButtonType::Settings => {
                        menu_state.set(MenuState::Settings);
                    }
                    MenuButtonType::Exit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                }
            }
        }
    }
}

/// Plugin to manage the main menu.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(MenuState::Main), setup)
            .add_systems(
                Update,
                MenuButtonType::action.run_if(in_state(MenuState::Main)),
            )
            .add_systems(Update, button_select.run_if(in_state(MenuState::Main)));
    }
}

/// Draw the main menu
fn setup(mut commands: Commands, cached_game: Res<AppCache>) {
    // Root node: full-screen center
    commands
        .spawn((DespawnOnExit(MenuState::Main), menu_screen_root()))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|menu| {
                    MenuButtonType::NewGame.spawn(menu);
                    if cached_game.unfinished_game.is_some() {
                        MenuButtonType::Continue.spawn(menu);
                    }
                    MenuButtonType::Settings.spawn(menu);
                    MenuButtonType::Exit.spawn(menu);
                });
        });
}
