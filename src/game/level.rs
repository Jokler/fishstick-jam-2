//! Spawn the main level.

use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{
    asset_tracking::LoadResource,
    screens::{Area, Screen},
};

use super::{inventory::Item, wife::spawn_wife};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
    app.init_resource::<Level>();

    app.add_systems(OnEnter(Screen::Gameplay), |mut commands: Commands| {
        commands.insert_resource(Level::default())
    });

    app.add_systems(
        OnEnter(Area::Cave),
        |mut commands: Commands, level: Res<Level>, items: Query<(Entity, &Item), With<Sprite>>| {
            commands.add(|w: &mut World| {
                SpawnBackground { area: Area::Cave }.apply(w);
                w.run_system_once(spawn_wife);
            });

            for (item, translation, rotation) in [
                (
                    Item::Knife,
                    Vec3::new(295.0, -130.0, -25.0),
                    180.0f32.to_radians(),
                ),
                (Item::BurntBanana, Vec3::new(0.0, -130.0, 55.0), 0.0),
            ] {
                let spawned = items.iter().find(|(_, i)| i == &&item).map(|(e, _)| e);
                let should_have = level.items.contains(&item);
                if spawned.is_none() && should_have {
                    commands.add(move |w: &mut World| {
                        SpawnItem {
                            item,
                            transform: Transform::from_translation(translation)
                                .with_scale(Vec3::splat(8.0))
                                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation)),
                        }
                        .apply(w);
                    });
                } else if !should_have {
                    if let Some(entity) = spawned {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        },
    );

    app.add_systems(
        Update,
        (|mut commands: Commands,
          level: Res<Level>,
          items: Query<(Entity, &Item), With<Sprite>>| {
            for (item, translation, rotation) in [
                (
                    Item::Knife,
                    Vec3::new(295.0, -130.0, -25.0),
                    180.0f32.to_radians(),
                ),
                (Item::BurntBanana, Vec3::new(0.0, -130.0, 55.0), 0.0),
            ] {
                let spawned = items.iter().find(|(_, i)| i == &&item).map(|(e, _)| e);
                let should_have = level.items.contains(&item);
                if spawned.is_none() && should_have {
                    commands.add(move |w: &mut World| {
                        SpawnItem {
                            item,
                            transform: Transform::from_translation(translation)
                                .with_scale(Vec3::splat(8.0))
                                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation)),
                        }
                        .apply(w);
                    });
                } else if !should_have {
                    if let Some(entity) = spawned {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        })
        .run_if(resource_changed::<Level>)
        .run_if(in_state(Area::Cave)),
    );

    app.add_systems(OnEnter(Area::Outside), |mut commands: Commands| {
        commands.add(|w: &mut World| {
            SpawnBackground {
                area: Area::Outside,
            }
            .apply(w);
        });
    });

    app.add_systems(
        Update,
        (|mut commands: Commands,
          level: Res<Level>,
          items: Query<(Entity, &Item), With<Sprite>>| {
            for (item, translation) in [
                (Item::WovenPapyrus, Vec3::new(-300.0, -130.0, -25.0)),
                (Item::Paper, Vec3::new(-300.0, -130.0, -25.0)),
                (Item::Papyrus, Vec3::new(170.0, -62.0, -25.0)),
                (Item::Banana, Vec3::new(440.0, -130.0, -25.0)),
            ] {
                let spawned = items.iter().find(|(_, i)| i == &&item).map(|(e, _)| e);
                let should_have = level.items.contains(&item);
                if spawned.is_none() && should_have {
                    commands.add(move |w: &mut World| {
                        SpawnItem {
                            item,
                            transform: Transform::from_translation(translation)
                                .with_scale(Vec3::splat(8.0)),
                        }
                        .apply(w);
                    });
                } else if !should_have {
                    if let Some(entity) = spawned {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        })
        .run_if(resource_changed::<Level>)
        .run_if(in_state(Area::Outside)),
    );

    app.add_systems(
        OnEnter(Area::Outside),
        |mut commands: Commands, level: Res<Level>, items: Query<(Entity, &Item), With<Sprite>>| {
            for (item, translation) in [
                (Item::WovenPapyrus, Vec3::new(-300.0, -130.0, -25.0)),
                (Item::Paper, Vec3::new(-300.0, -130.0, -25.0)),
                (Item::Papyrus, Vec3::new(170.0, -62.0, -25.0)),
                (Item::Banana, Vec3::new(440.0, -130.0, -25.0)),
            ] {
                let spawned = items.iter().find(|(_, i)| i == &&item).map(|(e, _)| e);
                let should_have = level.items.contains(&item);
                if spawned.is_none() && should_have {
                    commands.add(move |w: &mut World| {
                        SpawnItem {
                            item,
                            transform: Transform::from_translation(translation)
                                .with_scale(Vec3::splat(8.0)),
                        }
                        .apply(w);
                    });
                } else if !should_have {
                    if let Some(entity) = spawned {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        },
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Level {
    pub items: Vec<Item>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            items: vec![Item::Papyrus, Item::Knife, Item::Banana],
        }
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct LevelAssets {
    #[dependency]
    pub cave_background: Handle<Image>,
    #[dependency]
    pub cave_ground: Handle<Image>,
    #[dependency]
    pub outside_background: Handle<Image>,
    #[dependency]
    pub outside_ground: Handle<Image>,
    #[dependency]
    pub palm_tree: Handle<Image>,

    #[dependency]
    pub knife: Handle<Image>,
    #[dependency]
    pub papyrus: Handle<Image>,
    #[dependency]
    pub papyrus_strips: Handle<Image>,
    #[dependency]
    pub woven_papyrus: Handle<Image>,
    #[dependency]
    pub paper: Handle<Image>,
    #[dependency]
    pub written_paper: Handle<Image>,
    #[dependency]
    pub banana: Handle<Image>,
    #[dependency]
    pub burnt_banana: Handle<Image>,

    #[dependency]
    pub dino_stomp: Handle<AudioSource>,
}

impl LevelAssets {
    pub const PATH_CAVE_BACKGROUND: &'static str = "images/cave.png";
    pub const PATH_CAVE_GROUND: &'static str = "images/cave_ground.png";
    pub const PATH_OUTSIDE_BACKGROUND: &'static str = "images/outside.png";
    pub const PATH_OUTSIDE_GROUND: &'static str = "images/outside_ground.png";
    pub const PATH_PALM_TREE: &'static str = "images/palm_tree.png";
    pub const PATH_PAPYRUS: &'static str = "images/papyrus.png";
    pub const PATH_KNIFE: &'static str = "images/knife.png";
    pub const PATH_WOVEN: &'static str = "images/papyrus_woven.png";
    pub const PATH_PAPER: &'static str = "images/paper.png";
    pub const PATH_WRITTEN_PAPER: &'static str = "images/paper_written.png";
    pub const PATH_STRIPS: &'static str = "images/papyrus_strips.png";
    pub const PATH_BANANA: &'static str = "images/banan.png";
    pub const PATH_BURNT_BANANA: &'static str = "images/banan_burnt.png";
    pub const PATH_DINO_STOMP: &'static str = "audio/sound_effects/stomp.ogg";
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            cave_background: assets.load_with_settings(
                LevelAssets::PATH_CAVE_BACKGROUND,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            cave_ground: assets.load_with_settings(
                LevelAssets::PATH_CAVE_GROUND,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            outside_background: assets.load_with_settings(
                LevelAssets::PATH_OUTSIDE_BACKGROUND,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            outside_ground: assets.load_with_settings(
                LevelAssets::PATH_OUTSIDE_GROUND,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            palm_tree: assets.load_with_settings(
                LevelAssets::PATH_PALM_TREE,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            papyrus: assets.load_with_settings(
                LevelAssets::PATH_PAPYRUS,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            papyrus_strips: assets.load_with_settings(
                LevelAssets::PATH_STRIPS,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            knife: assets.load_with_settings(
                LevelAssets::PATH_KNIFE,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            woven_papyrus: assets.load_with_settings(
                LevelAssets::PATH_WOVEN,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            paper: assets.load_with_settings(
                LevelAssets::PATH_PAPER,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            written_paper: assets.load_with_settings(
                LevelAssets::PATH_WRITTEN_PAPER,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            banana: assets.load_with_settings(
                LevelAssets::PATH_BANANA,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            burnt_banana: assets.load_with_settings(
                LevelAssets::PATH_BURNT_BANANA,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            dino_stomp: assets.load(LevelAssets::PATH_DINO_STOMP),
        }
    }
}

#[derive(Debug)]
pub struct SpawnBackground {
    area: Area,
}

impl Command for SpawnBackground {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_background);
    }
}

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

fn spawn_background(
    In(config): In<SpawnBackground>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        Name::new(format!("{} Background", config.area)),
        Background,
        SpriteBundle {
            texture: match config.area {
                Area::Cave => level_assets.cave_background.clone(),
                Area::Outside => level_assets.outside_background.clone(),
            },
            transform: match config.area {
                Area::Cave => Transform::from_translation(Vec3::new(0.0, 200.0, -50.0))
                    .with_scale(Vec3::splat(0.8)),
                Area::Outside => Transform::from_translation(Vec3::new(0.0, 160.0, -50.0))
                    .with_scale(Vec3::splat(2.0)),
            },
            ..Default::default()
        },
        StateScoped(config.area),
    ));

    commands.spawn((
        Name::new(format!("{} Ground", config.area)),
        Ground,
        SpriteBundle {
            texture: match config.area {
                Area::Cave => level_assets.cave_ground.clone(),
                Area::Outside => level_assets.outside_ground.clone(),
            },
            transform: match config.area {
                Area::Cave => Transform::from_translation(Vec3::new(0.0, -264.0, 40.0))
                    .with_scale(Vec3::splat(1.0)),
                Area::Outside => Transform::from_translation(Vec3::new(0.0, -255.0, 50.0))
                    .with_scale(Vec3::splat(1.0)),
            },
            ..Default::default()
        },
        StateScoped(config.area),
    ));

    if config.area == Area::Outside {
        commands.spawn((
            Name::new("Palm Tree"),
            SpriteBundle {
                texture: level_assets.palm_tree.clone(),
                transform: Transform::from_translation(Vec3::new(480.0, 95.0, -40.0))
                    .with_scale(Vec3::splat(8.0)),
                ..Default::default()
            },
            StateScoped(config.area),
        ));
    }
}

#[derive(Debug)]
pub struct SpawnItem {
    item: Item,
    transform: Transform,
}

impl Command for SpawnItem {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_item);
    }
}

fn spawn_item(
    In(config): In<SpawnItem>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    state: Res<State<Area>>,
) {
    commands.spawn((
        Name::new(config.item.to_string()),
        config.item,
        SpriteBundle {
            texture: match config.item {
                Item::Papyrus => level_assets.papyrus.clone(),
                Item::Knife => level_assets.knife.clone(),
                Item::WovenPapyrus => level_assets.woven_papyrus.clone(),
                Item::Paper => level_assets.paper.clone(),
                Item::Banana => level_assets.banana.clone(),
                Item::BurntBanana => level_assets.burnt_banana.clone(),
                _ => todo!("no sprite for {}", config.item.to_string()),
            },
            transform: config.transform,
            ..Default::default()
        },
        StateScoped(*state.get()),
    ));
}
