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
    game::{animation::Animation, movement::MovementController},
    screens::Screen,
    AppSet,
};

use super::{
    animation::{AnimationData, AnimationState},
    movement::ActionsFrozen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.load_resource::<PlayerAssets>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (record_player_directional_input, auto_run)
            .chain()
            .in_set(AppSet::RecordInput),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(16, 23),
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
    let walk = AnimationData {
        frames: 2,
        interval: Duration::from_millis(100),
        state: AnimationState::Walking,
        atlas_index: 2,
    };
    let player_animation = Animation::new(vec![idle, walk]);

    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: player_assets.caveman.clone(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0))
                .with_translation(Vec3::new(-330.0, -70.0, 0.0)),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController {
            max_speed: 300.0,
            ..default()
        },
        player_animation,
        StateScoped(Screen::Gameplay),
    ));

    commands.spawn((
        Name::new("Healthbar"),
        SpriteBundle {
            texture: player_assets.healthbar.clone(),
            transform: Transform::from_scale(Vec2::splat(4.0).extend(1.0))
                .with_translation(Vec3::new(-515.0, -320.0, 60.0)),
            ..Default::default()
        },
        StateScoped(Screen::Gameplay),
    ));
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut MovementController, With<Player>>,
    actions_frozen: Res<ActionsFrozen>,
) {
    if actions_frozen.is_frozen() {
        for mut controller in &mut controllers {
            controller.intent = Vec2::ZERO;
        }
        return;
    }
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controllers {
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    #[dependency]
    pub caveman: Handle<Image>,
    #[dependency]
    pub healthbar: Handle<Image>,
    #[dependency]
    pub paper_big: Handle<Image>,

    #[dependency]
    pub item_pickup: Handle<AudioSource>,
    #[dependency]
    pub vine_boom: Handle<AudioSource>,
    #[dependency]
    pub uh_oh: Handle<AudioSource>,
    #[dependency]
    pub trophy_wife: Handle<AudioSource>,
    #[dependency]
    pub wife_hm: Handle<AudioSource>,
    #[dependency]
    pub run_outside: Handle<AudioSource>,
    #[dependency]
    pub run_cave: Handle<AudioSource>,

    #[dependency]
    pub animal_font: Handle<Font>,
}

impl PlayerAssets {
    pub const PATH_CAVEMAN: &'static str = "images/caveman.png";
    pub const PATH_HEALTHBAR: &'static str = "images/health_bar.png";
    pub const PATH_PAPER_BIG: &'static str = "images/paper_big.png";
    pub const PATH_ITEM_PICKUP: &'static str = "audio/sound_effects/item_pickup.ogg";
    pub const PATH_VINE_BOOM: &'static str = "audio/sound_effects/vine_boom.ogg";
    pub const PATH_UH_OH: &'static str = "audio/sound_effects/uh_oh.ogg";
    pub const PATH_TROPHY_WIFE: &'static str = "audio/sound_effects/trophy_wife.ogg";
    pub const PATH_WIFE_HM: &'static str = "audio/sound_effects/wife_hm.ogg";
    pub const PATH_RUN_OUTSIDE: &'static str = "audio/sound_effects/run_outside.ogg";
    pub const PATH_RUN_CAVE: &'static str = "audio/sound_effects/run_cave.ogg";
    pub const PATH_ANIMAL_FONT: &'static str = "fonts/Animal-Alphabet-Regular.ttf";
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            caveman: assets.load_with_settings(
                PlayerAssets::PATH_CAVEMAN,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            healthbar: assets.load_with_settings(
                PlayerAssets::PATH_HEALTHBAR,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            paper_big: assets.load_with_settings(
                PlayerAssets::PATH_PAPER_BIG,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            item_pickup: assets.load(PlayerAssets::PATH_ITEM_PICKUP),
            vine_boom: assets.load(PlayerAssets::PATH_VINE_BOOM),
            uh_oh: assets.load(PlayerAssets::PATH_UH_OH),
            trophy_wife: assets.load(PlayerAssets::PATH_TROPHY_WIFE),
            wife_hm: assets.load(PlayerAssets::PATH_WIFE_HM),

            run_outside: assets.load(PlayerAssets::PATH_RUN_OUTSIDE),
            run_cave: assets.load(PlayerAssets::PATH_RUN_CAVE),
            animal_font: assets.load(PlayerAssets::PATH_ANIMAL_FONT),
        }
    }
}

#[derive(Component, Debug)]
pub struct AutoRunner {
    pub end_position: f32,
    pub intent: Vec2,
}

fn auto_run(
    mut commands: Commands,
    mut controllers: Query<(
        Entity,
        &Transform,
        &mut Sprite,
        &mut MovementController,
        &AutoRunner,
    )>,
) {
    for (entity, transform, mut sprite, mut controller, runner) in &mut controllers {
        let go_left = runner.intent.x < 0.0;
        if go_left && transform.translation.x < runner.end_position
            || !go_left && transform.translation.x > runner.end_position
        {
            controller.intent = Vec2::ZERO;
            commands.entity(entity).remove::<AutoRunner>();
            sprite.flip_x = false;

            continue;
        }
        controller.intent = runner.intent.normalize_or_zero();
    }
}
