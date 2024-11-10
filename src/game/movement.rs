//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    screens::{Area, Screen},
    AppSet,
};

use super::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.init_resource::<ActionsFrozen>();

    app.add_systems(
        Update,
        (apply_movement, clamp_player_x, change_level)
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(OnEnter(Screen::Difficulty), |mut commands: Commands| {
        commands.insert_resource(ActionsFrozen::default())
    });
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics
    /// engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}

fn change_level(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut next_state: ResMut<NextState<Area>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let half_width = window.size().x / 2.0 + 50.0;
    for mut transform in &mut player_query {
        if transform.translation.x > half_width {
            transform.translation.x = -half_width;
            next_state.set(Area::Outside);
        } else if transform.translation.x < -half_width {
            transform.translation.x = half_width;
            next_state.set(Area::Cave);
        }
    }
}

fn clamp_player_x(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    area: Res<State<Area>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let half_width = window.size().x / 2.0 - 50.0;
    for mut transform in &mut player_query {
        if transform.translation.x < -half_width && *area == Area::Cave {
            transform.translation.x = -half_width;
        } else if transform.translation.x > half_width && *area == Area::Outside {
            transform.translation.x = half_width;
        }
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ActionsFrozen {
    freeze_count: usize,
}
impl ActionsFrozen {
    pub fn freeze(&mut self) {
        self.freeze_count += 1;
    }
    pub fn unfreeze(&mut self) {
        self.freeze_count -= 1;
    }
    pub fn is_frozen(&self) -> bool {
        self.freeze_count > 0
    }
}
