use bevy::{
    app::PluginGroupBuilder,
    color::palettes::{basic::*, css::DARK_CYAN},
    prelude::*,
};

mod home;
use home::MainMenuPlugin;
mod splash;
use splash::SplashPlugin;
mod setting;
use setting::SettingsPlugin;
mod about;
use about::AboutPlugin;
mod new_game;
use new_game::NewGamePlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    NewGame,
    Settings,
    SettingsTheme,
    SettingsHints,
    SettingsControls,
    SettingsAbout,
    Hidden,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

pub struct MenuPlugins;

impl PluginGroup for MenuPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SplashPlugin)
            .add(MainMenuPlugin)
            .add(SettingsPlugin)
            .add(AboutPlugin)
            .add(NewGamePlugin)
    }
}

#[derive(Bundle)]
pub struct ButtonBgBundle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
    pub button: Button,
}

impl Default for ButtonBgBundle {
    fn default() -> Self {
        Self {
            node: Node {
                max_width: Val::Px(350.0),
                min_width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(6.0)),
                margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::left(Val::Px(15.0)).with_right(Val::Px(15.0)),
                ..Default::default()
            },
            border_color: BorderColor::all(DARK_CYAN),
            background_color: BackgroundColor(Color::BLACK),
            button: Button,
        }
    }
}

#[derive(Bundle)]
pub struct ButtonTextBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl ButtonTextBundle {
    pub fn new(label: &str) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont {
                font_size: 28.0,
                ..Default::default()
            },
            color: TextColor(Color::WHITE),
        }
    }
}

#[derive(Component)]
struct BackButton;

/// Draws a canvas that occupies the entire screen for menu UIs.
pub fn menu_screen_root() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}

/// Function to change the color of a selected button.
pub fn button_select(
    mut interaction_query: Query<(&Interaction, &mut BorderColor), Changed<Interaction>>,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                *border_color = BorderColor::all(GREEN);
            }
            Interaction::None => {
                *border_color = BorderColor::all(DARK_CYAN);
            }
        }
    }
}
