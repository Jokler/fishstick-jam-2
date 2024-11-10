//! The game's main screen states and transitions between them.

mod credits;
mod difficulty;
mod end;
mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use derive_more::derive::Display;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_sub_state::<Area>();
    app.enable_state_scoped_entities::<Screen>();
    app.enable_state_scoped_entities::<Area>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        difficulty::plugin,
        end::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum Screen {
    // TODO: Splash screen?
    Splash,
    #[default]
    Loading,
    Title,
    Difficulty,
    Credits,
    Gameplay,
    End,
}

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Copy, Default, Display)]
#[source(Screen = Screen::Gameplay)]
pub enum Area {
    Cave,
    #[default]
    Outside,
}
