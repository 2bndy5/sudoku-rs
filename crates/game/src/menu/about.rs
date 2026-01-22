use super::{BackButton, ButtonBgBundle, ButtonTextBundle, MenuState, button_select};
use bevy::prelude::*;

#[derive(Component)]
struct OnAboutScreen;

pub struct AboutPlugin;

impl Plugin for AboutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::SettingsAbout), setup_about_info)
            .add_systems(
                Update,
                back_button_listen.run_if(in_state(MenuState::SettingsAbout)),
            )
            .add_systems(
                Update,
                button_select.run_if(in_state(MenuState::SettingsAbout)),
            );
    }
}

fn setup_about_info(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(MenuState::SettingsAbout),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        OnAboutScreen,
        children![
            (
                Text::new("About This Game"),
                TextFont {
                    font_size: 48.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::new(concat!(
                    "Version ",
                    env!("CARGO_PKG_VERSION"),
                    "\nDeveloped by Brendan Doherty"
                )),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ),
            (
                ButtonBgBundle::default(),
                BackButton,
                children![ButtonTextBundle::new("Back")],
            )
        ],
    ));
}

fn back_button_listen(
    mut interaction_query: Query<(&Interaction, &BackButton), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, _) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            menu_state.set(MenuState::Settings);
        }
    }
}
