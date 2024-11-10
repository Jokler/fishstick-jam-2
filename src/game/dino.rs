use std::time::Duration;

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};
use bevy_yarnspinner::prelude::DialogueRunner;

use crate::{asset_tracking::LoadResource, audio::SoundEffect, screens::Area};

use super::{
    inventory::{Inventory, Item},
    level::LevelAssets,
    movement::ActionsFrozen,
    player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DinoLeg>();
    app.load_resource::<DinoAssets>();

    app.add_systems(
        Update,
        (start_event, dino_stomp).run_if(in_state(Area::Outside)),
    );
    app.observe(spawn_dino);
}

fn start_event(
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    inventory: Res<Inventory>,
    player: Query<&Transform, With<Player>>,
) {
    if !inventory.items.contains(&Item::WovenPapyrus) {
        // inventory.items.push(Item::WovenPapyrus);
        return;
    }
    for transform in &player {
        if transform.translation.x < -440.0 {
            return;
        }
    }
    let mut dialogue_runner = dialogue_runner
        .get_single_mut()
        .expect("only one dialogue runner");
    dialogue_runner.start_node("Dino");
    actions_frozen.freeze();
}

#[derive(Event, Debug)]
pub struct SpawnDino;

fn spawn_dino(_: Trigger<SpawnDino>, mut commands: Commands, dino_assets: Res<DinoAssets>) {
    let tween = Tween::new(
        EaseFunction::ExponentialIn,
        Duration::from_millis(1500),
        TransformPositionLens {
            start: Vec3::new(200.0, 770.0, 1.0),
            end: Vec3::new(-200.0, 238.0, 0.0),
        },
    )
    .with_completed_event(69)
    .then(Tween::new(
        EaseFunction::ExponentialIn,
        Duration::from_millis(1500),
        TransformPositionLens {
            start: Vec3::new(-200.0, 238.0, 0.0),
            end: Vec3::new(-400.0, 770.0, 1.0),
        },
    ));

    commands.spawn((
        Name::new("Dino Leg"),
        DinoLeg,
        SpriteBundle {
            texture: dino_assets.dino_leg.clone(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0))
                .with_translation(Vec3::new(200.0, 770.0, 1.0)),
            ..Default::default()
        },
        Animator::new(tween),
        StateScoped(Area::Outside),
    ));
}

fn dino_stomp(
    mut reader: EventReader<TweenCompleted>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    for ev in reader.read() {
        if ev.user_data == 69 {
            commands.spawn((
                AudioBundle {
                    source: level_assets.dino_stomp.clone(),
                    settings: PlaybackSettings::DESPAWN,
                },
                SoundEffect,
                Name::from("Dino Stomp"),
            ));
        }
    }
}

#[derive(Component, Debug, Reflect)]
struct DinoLeg;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct DinoAssets {
    #[dependency]
    pub dino_leg: Handle<Image>,
    #[dependency]
    pub stomp: Handle<AudioSource>,
}

impl DinoAssets {
    pub const PATH_DINO_LEG: &'static str = "images/dino_leg.png";
    pub const PATH_STOMP: &'static str = "audio/sound_effects/step1.ogg";
}

impl FromWorld for DinoAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            dino_leg: assets.load_with_settings(
                DinoAssets::PATH_DINO_LEG,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            stomp: assets.load(DinoAssets::PATH_STOMP),
        }
    }
}
