//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::spawn::level::SpawnLevel;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing)
        .add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
}

// TODO: Reset camera transform here.
fn exit_playing() {}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
