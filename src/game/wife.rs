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

use super::{
    animation::{Animation, AnimationData, AnimationState},
    movement::ActionsFrozen,
    player::Player,
};
use crate::{
    asset_tracking::LoadResource,
    screens::{Area, Screen},
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<WifeAssets>();

    app.add_systems(Update, talk.run_if(in_state(Screen::Gameplay)));
    app.observe(start_wife_dialogue);
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wife;

pub fn spawn_wife(
    mut commands: Commands,
    player_assets: Res<WifeAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(15, 21),
        4,
        1,
        Some(UVec2::splat(2)),
        Some(UVec2::splat(1)),
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let idle = AnimationData {
        frames: 2,
        interval: Duration::from_millis(200),
        state: AnimationState::Idling,
        atlas_index: 0,
    };
    let player_animation = Animation::new(vec![idle]);

    commands.spawn((
        Name::new("Wife"),
        Wife,
        SpriteBundle {
            texture: player_assets.wife.clone(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0))
                .with_translation(Vec3::new(-400.0, -78.0, 0.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        StateScoped(Area::Cave),
    ));
}

fn talk(
    // mut gizmos: Gizmos,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<(&Aabb, &Transform), With<Player>>,
    wife: Query<(&Aabb, &Transform), With<Wife>>,
    actions_frozen: Res<ActionsFrozen>,
) {
    if actions_frozen.is_frozen() {
        return;
    }
    for (player_aabb, player_transform) in &player {
        let player_aabb2d = Aabb2d::new(
            player_transform.translation.xy(),
            player_aabb.half_extents.xy() * player_transform.scale.xy(),
        );
        for (item_aabb, item_transform) in &wife {
            let item_aabb2d = Aabb2d::new(
                item_transform.translation.xy(),
                item_aabb.half_extents.xy() * item_transform.scale.xy(),
            );
            // gizmos.rect_2d(item_aabb2d.center(), 0., item_aabb2d.half_size() * 2., BLUE);
            if player_aabb2d.intersects(&item_aabb2d) {
                if !input.just_pressed(KeyCode::KeyE) {
                    return;
                }
                commands.trigger(DialogueStart);
            }
        }
    }
}

#[derive(Debug, Event)]
struct DialogueStart;

fn start_wife_dialogue(
    _trigger: Trigger<DialogueStart>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
) {
    let mut dialogue_runner = dialogue_runner
        .get_single_mut()
        .expect("only one dialogue runner");
    dialogue_runner.start_node("Wife");
    actions_frozen.freeze();
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct WifeAssets {
    #[dependency]
    pub wife: Handle<Image>,
}

impl WifeAssets {
    pub const PATH_WIFE: &'static str = "images/wife.png";
}

impl FromWorld for WifeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            wife: assets.load_with_settings(
                WifeAssets::PATH_WIFE,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
