//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
pub mod dino;
pub mod inventory;
pub mod level;
pub mod movement;
pub mod player;
mod wife;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        movement::plugin,
        player::plugin,
        level::plugin,
        inventory::plugin,
        wife::plugin,
        dino::plugin,
    ));
}
