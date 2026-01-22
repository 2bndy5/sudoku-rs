use super::AppState;
use bevy::prelude::*;

use crate::board::loading::{LoadingAnimationInterval, populate_grid, spawn_loading_grid};

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_splash)
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
            .add_systems(Update, populate_grid.run_if(in_state(AppState::Splash)));
    }
}

fn setup_splash(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            DespawnOnExit(AppState::Splash),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
            ZIndex(1),
        ))
        .with_children(spawn_loading_grid)
        .with_children(|root| {
            root.spawn((
                Text::new("Sudoku"),
                TextFont {
                    font_size: 48.0,
                    ..Default::default()
                },
            ));
            root.spawn((
                Text::new("An open source project"),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
            ));
            root.spawn((
                Text::new("written in Rust"),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
            ));
        });
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    commands.insert_resource(LoadingAnimationInterval(Timer::from_seconds(
        0.2,
        TimerMode::Repeating,
    )));
}

fn countdown(
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
    mut state: ResMut<NextState<AppState>>,
) {
    if timer.tick(time.delta()).is_finished() {
        state.set(AppState::Menu);
    }
}
