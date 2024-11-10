//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use std::time::Duration;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::{
        primitives::Aabb,
        texture::{ImageLoaderSettings, ImageSampler},
    },
};
use bevy_yarnspinner::prelude::DialogueRunner;

use crate::{
    asset_tracking::LoadResource, audio::SoundEffect, game::animation::Animation, screens::Area,
};

use super::{
    animation::{AnimationData, AnimationState},
    inventory::{Inventory, Item},
    level::Level,
    movement::ActionsFrozen,
    player::{Player, PlayerAssets},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Fire>();
    app.load_resource::<FireAssets>();
    app.add_systems(OnEnter(Area::Cave), spawn_fire);
    app.add_systems(Update, place_banana.run_if(in_state(Area::Cave)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Fire;

fn spawn_fire(
    mut commands: Commands,
    fire_assets: Res<FireAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(25, 29),
        3,
        1,
        Some(UVec2::splat(2)),
        Some(UVec2::splat(1)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let idle = AnimationData {
        frames: 3,
        interval: Duration::from_millis(200),
        state: AnimationState::Idling,
        atlas_index: 0,
    };
    let animation = Animation::new(vec![idle]);

    commands.spawn((
        Name::new("Fire"),
        Fire,
        SpriteBundle {
            texture: fire_assets.fire.clone(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0))
                .with_translation(Vec3::new(-80.0, -110.0, 50.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation.get_atlas_index(),
        },
        animation,
        StateScoped(Area::Cave),
    ));
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct FireAssets {
    #[dependency]
    pub fire: Handle<Image>,
}

impl FireAssets {
    pub const PATH_FIRE: &'static str = "images/campfire.png";
}

impl FromWorld for FireAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            fire: assets.load_with_settings(
                FireAssets::PATH_FIRE,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

fn place_banana(
    mut commands: Commands,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut inventory: ResMut<Inventory>,
    mut level: ResMut<Level>,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<(&Aabb, &Transform), With<Player>>,
    fire: Query<(&Aabb, &Transform), With<Fire>>,
    player_assets: Res<PlayerAssets>,
    mut actions_frozen: ResMut<ActionsFrozen>,
) {
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }
    if actions_frozen.is_frozen() {
        return;
    }
    for (player_aabb, player_transform) in &player {
        let player_aabb2d = Aabb2d::new(
            player_transform.translation.xy(),
            player_aabb.half_extents.xy() * player_transform.scale.xy(),
        );
        for (fire_aabb, fire_transform) in &fire {
            let fire_aabb2d = Aabb2d::new(
                fire_transform.translation.xy(),
                fire_aabb.half_extents.xy() * fire_transform.scale.xy(),
            );
            if player_aabb2d.intersects(&fire_aabb2d) {
                let mut dialogue_runner = dialogue_runner
                    .get_single_mut()
                    .expect("only one dialogue runner");

                let item = Item::Banana;
                let Some(index) = inventory.items.iter().position(|x| *x == item) else {
                    if !level.items.contains(&Item::BurntBanana) {
                        dialogue_runner.start_node("Fire");
                        actions_frozen.freeze();
                    }
                    return;
                };
                inventory.items.remove(index);
                level.items.push(Item::BurntBanana);

                let vars = dialogue_runner.variable_storage_mut();

                vars.set(format!("$_has_{}", item), false.into()).unwrap();

                commands.spawn((
                    AudioBundle {
                        source: player_assets.item_pickup.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    },
                    SoundEffect,
                    Name::from("Drop sound"),
                ));
                dialogue_runner.start_node("DroppedBanana");
                actions_frozen.freeze();
            }
        }
    }
}
