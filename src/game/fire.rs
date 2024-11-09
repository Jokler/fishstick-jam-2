//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use std::time::Duration;

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{
    asset_tracking::LoadResource,
    game::animation::Animation,
    screens::{Area, Screen},
};

use super::animation::{AnimationData, AnimationState};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Fire>();
    app.load_resource::<FireAssets>();
    app.add_systems(OnEnter(Area::Cave), spawn_fire);
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
