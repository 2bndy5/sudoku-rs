use crate::menu::{ButtonBgBundle, button_select, menu_screen_root};

use super::MenuState;
use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

#[derive(Component)]
struct OnSettingsMenu;

#[derive(Component)]
struct OnThemeSettings;

#[derive(Component)]
struct OnHintsSettings;

#[derive(Component)]
struct OnControlsSettings;

#[derive(Component)]
pub struct SettingsButton(SettingsButtonType);

#[derive(Copy, Clone)]
pub enum SettingsButtonType {
    Theme,
    Hints,
    Controls,
    About,
    Back,
}

impl SettingsButtonType {
    fn as_str(&self) -> &str {
        match self {
            SettingsButtonType::Theme => "Theme",
            SettingsButtonType::Hints => "Hints",
            SettingsButtonType::Controls => "Controls",
            SettingsButtonType::About => "About",
            SettingsButtonType::Back => "Back",
        }
    }

    fn spawn(self, node: &mut RelatedSpawnerCommands<'_, ChildOf>) {
        let label = self.as_str();
        node.spawn((ButtonBgBundle::default(), SettingsButton(self)))
            .with_children(|b| {
                b.spawn(super::ButtonTextBundle::new(label));
            });
    }

    /// System to handle menu button interactions
    fn action(
        mut interaction_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ) {
        for (interaction, menu_button) in &mut interaction_query {
            if matches!(*interaction, Interaction::Pressed) {
                let button_type = &menu_button.0;
                match *button_type {
                    SettingsButtonType::Theme => {
                        menu_state.set(MenuState::SettingsTheme);
                    }
                    SettingsButtonType::Hints => {
                        menu_state.set(MenuState::SettingsHints);
                    }
                    SettingsButtonType::Controls => {
                        menu_state.set(MenuState::SettingsControls);
                    }
                    SettingsButtonType::About => {
                        menu_state.set(MenuState::SettingsAbout);
                    }
                    SettingsButtonType::Back => {
                        menu_state.set(MenuState::Main);
                    }
                }
            }
        }
    }
}

/// Plugin to manage the settings' root menu.
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // Add systems and resources related to settings here
        app.add_systems(OnEnter(MenuState::Settings), setup_settings_menu)
            .add_systems(OnEnter(MenuState::SettingsTheme), setup_theme_settings)
            .add_systems(OnEnter(MenuState::SettingsHints), setup_hints_settings)
            .add_systems(
                OnEnter(MenuState::SettingsControls),
                setup_controls_settings,
            )
            .add_systems(Update, button_select.run_if(in_state(MenuState::Settings)))
            .add_systems(
                Update,
                button_select.run_if(in_state(MenuState::SettingsTheme)),
            )
            .add_systems(
                Update,
                button_select.run_if(in_state(MenuState::SettingsHints)),
            )
            .add_systems(
                Update,
                button_select.run_if(in_state(MenuState::SettingsControls)),
            )
            .add_systems(
                Update,
                SettingsButtonType::action.run_if(in_state(MenuState::Settings)),
            );
    }
}

/// Setup UI for main settings menu
fn setup_settings_menu(mut commands: Commands) {
    commands
        .spawn((
            OnSettingsMenu,
            DespawnOnExit(MenuState::Settings),
            menu_screen_root(),
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },))
                .with_children(|menu| {
                    menu.spawn((
                        Text::new("Settings Menu"),
                        TextFont {
                            font_size: 36.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    SettingsButtonType::Theme.spawn(menu);
                    SettingsButtonType::Hints.spawn(menu);
                    SettingsButtonType::Controls.spawn(menu);
                    SettingsButtonType::About.spawn(menu);
                    SettingsButtonType::Back.spawn(menu);
                });
        });
}

/// Setup UI for theme settings
fn setup_theme_settings(mut commands: Commands) {
    commands.spawn((
        OnThemeSettings,
        DespawnOnExit(MenuState::SettingsTheme),
        menu_screen_root(),
        children![Text::new("Theme Settings")],
    ));
}

/// Setup UI for hints settings
fn setup_hints_settings(mut commands: Commands) {
    commands.spawn((
        OnHintsSettings,
        DespawnOnExit(MenuState::SettingsHints),
        menu_screen_root(),
        children![Text::new("Hints Settings")],
    ));
}

/// Setup UI for controls settings
fn setup_controls_settings(mut commands: Commands) {
    commands.spawn((
        OnControlsSettings,
        DespawnOnExit(MenuState::SettingsControls),
        menu_screen_root(),
        children![Text::new("Controls Settings")],
    ));
}
