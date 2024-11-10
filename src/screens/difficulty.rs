//! The difficulty screen that appears when clicking play.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Difficulty), spawn_difficulty_screen);
}

fn spawn_difficulty_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Difficulty))
        .with_children(|children| {
            children.button("Story").observe(enter_gameplay_screen);
            children.button("Medium").observe(enter_gameplay_screen);
            children.button("Brutal").observe(enter_gameplay_screen);

            children.label("");
            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
