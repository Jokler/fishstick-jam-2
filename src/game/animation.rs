use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    audio::SoundEffect,
    game::{movement::MovementController, player::PlayerAssets},
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<Animation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSet::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .run_if(resource_exists::<PlayerAssets>)
                .in_set(AppSet::Update),
        ),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut Animation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        let dx = controller.intent.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if controller.intent == Vec2::ZERO {
            AnimationState::Idling
        } else {
            AnimationState::Walking
        };
        animation.update_state(animation_state);
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut Animation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&Animation, &mut TextureAtlas)>) {
    for (animation, mut atlas) in &mut query {
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut step_query: Query<&Animation>,
) {
    for animation in &mut step_query {
        if animation.state() == AnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::thread_rng();
            let random_step = player_assets.steps.choose(rng).unwrap();
            commands.spawn((
                AudioBundle {
                    source: random_step.clone(),
                    settings: PlaybackSettings::DESPAWN,
                },
                SoundEffect,
                Name::from("Step Sound"),
            ));
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Animation {
    timer: Timer,
    frame: usize,
    current: usize,
    animations: Vec<AnimationData>,
}

#[derive(Reflect)]
pub struct AnimationData {
    pub frames: usize,
    pub interval: Duration,
    pub state: AnimationState,
    pub atlas_index: usize,
}

#[derive(Debug, Reflect, PartialEq, Clone, Copy)]
pub enum AnimationState {
    Idling,
    Walking,
}

impl Animation {
    pub fn new(animations: Vec<AnimationData>) -> Self {
        Self {
            timer: Timer::new(animations[0].interval, TimerMode::Repeating),
            frame: 0,
            current: 0,
            animations,
        }
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1) % self.animations[self.current].frames;
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: AnimationState) {
        if self.state() != state {
            self.current = self
                .animations
                .iter()
                .position(|a| a.state == state)
                .unwrap();

            let data = &self.animations[self.current];

            self.timer = Timer::new(data.interval, TimerMode::Repeating);
            self.frame = 0;
            self.update_timer(self.timer.remaining());
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    pub fn state(&self) -> AnimationState {
        self.animations[self.current].state
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        self.animations[self.current].atlas_index + self.frame
    }
}
