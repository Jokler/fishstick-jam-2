//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::{input_just_pressed, input_toggle_active},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screens::{Area, Screen};

const INSPECTOR_TOGGLE_KEY: KeyCode = KeyCode::Backquote;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, log_transitions::<Area>);

    app.add_plugins((
        DebugUiPlugin,
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, INSPECTOR_TOGGLE_KEY)),
    ));
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(UI_TOGGLE_KEY)),
    );
}

const UI_TOGGLE_KEY: KeyCode = KeyCode::KeyU;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
