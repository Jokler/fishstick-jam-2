//! Spawn the main level.

use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::{asset_tracking::LoadResource, screens::Area};

use super::{inventory::Item, wife::spawn_wife};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
    app.init_resource::<Level>();

    app.add_systems(
        OnEnter(Area::Cave),
        |mut commands: Commands, level: Res<Level>| {
            commands.add(|w: &mut World| {
                SpawnBackground { area: Area::Cave }.apply(w);

                w.run_system_once(spawn_wife)
            });
            if level.items.contains(&Item::Knife) {
                commands.add(|w: &mut World| {
                    SpawnItem {
                        item: Item::Knife,
                        transform: Transform::from_translation(Vec3::new(-200.0, -130.0, -25.0))
                            .with_scale(Vec3::splat(8.0)),
                    }
                    .apply(w);
                });
            }
        },
    );

    app.add_systems(
        OnEnter(Area::Outside),
        |mut commands: Commands, level: Res<Level>| {
            commands.add(|w: &mut World| {
                SpawnBackground {
                    area: Area::Outside,
                }
                .apply(w);
            });
            if level.items.contains(&Item::Papyrus) {
                commands.add(|w: &mut World| {
                    SpawnItem {
                        item: Item::Papyrus,
                        transform: Transform::from_translation(Vec3::new(0.0, -60.0, -25.0))
                            .with_scale(Vec3::splat(8.0)),
                    }
                    .apply(w);
                });
            }
            if level.items.contains(&Item::WovenPapyrus) {
                commands.add(|w: &mut World| {
                    SpawnItem {
                        item: Item::WovenPapyrus,
                        transform: Transform::from_translation(Vec3::new(-430.0, -130.0, -25.0))
                            .with_scale(Vec3::splat(8.0)),
                    }
                    .apply(w);
                });
            }
            if level.items.contains(&Item::Paper) {
                commands.add(|w: &mut World| {
                    SpawnItem {
                        item: Item::Papyrus,
                        transform: Transform::from_translation(Vec3::new(-430.0, -130.0, -25.0))
                            .with_scale(Vec3::splat(8.0)),
                    }
                    .apply(w);
                });
            }
        },
    );
    app.add_systems(
        Update,
        (|mut commands: Commands,
          level: Res<Level>,
          items: Query<(Entity, &Item), With<Sprite>>| {
            let spawned = items
                .iter()
                .find(|(_, i)| i == &&Item::WovenPapyrus)
                .map(|(e, _)| e);
            let should_have = level.items.contains(&Item::WovenPapyrus);
            dbg!(spawned, should_have);
            if spawned.is_none() && should_have {
                commands.add(|w: &mut World| {
                    SpawnItem {
                        item: Item::WovenPapyrus,
                        transform: Transform::from_translation(Vec3::new(-430.0, -130.0, -25.0))
                            .with_scale(Vec3::splat(8.0)),
                    }
                    .apply(w);
                });
            } else if !should_have {
                if let Some(entity) = spawned {
                    commands.entity(entity).despawn_recursive();
                }
            }

            let spawned = items
                .iter()
                .find(|(_, i)| i == &&Item::Paper)
                .map(|(e, _)| e);
            let should_have = level.items.contains(&Item::Paper);
            if spawned.is_none() && should_have {
                commands.add(|w: &mut World| {
                    SpawnItem {
                        item: Item::Paper,
                        transform: Transform::from_translation(Vec3::new(-430.0, -130.0, -25.0))
                            .with_scale(Vec3::splat(8.0)),
                    }
                    .apply(w);
                });
            } else if !should_have {
                if let Some(entity) = spawned {
                    commands.entity(entity).despawn_recursive();
                }
            }
        })
        .run_if(resource_changed::<Level>)
        .run_if(in_state(Area::Outside)),
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
            items: vec![Item::Papyrus, Item::Knife],
        }
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct LevelAssets {
    #[dependency]
    pub cave_background: Handle<Image>,
    #[dependency]
    pub outside_background: Handle<Image>,

    #[dependency]
    pub papyrus: Handle<Image>,
    #[dependency]
    pub knife: Handle<Image>,
    #[dependency]
    pub woven_papyrus: Handle<Image>,
    #[dependency]
    pub paper: Handle<Image>,
}

impl LevelAssets {
    pub const PATH_CAVE: &'static str = "images/cave.png";
    pub const PATH_OUTSIDE: &'static str = "images/outside.png";
    pub const PATH_PAPYRUS: &'static str = "images/papyrus.png";
    pub const PATH_KNIFE: &'static str = "images/knife.png";
    pub const PATH_WOVEN: &'static str = "images/woven.png";
    pub const PATH_PAPER: &'static str = "images/paper.png";
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            cave_background: assets.load(LevelAssets::PATH_CAVE),
            outside_background: assets.load(LevelAssets::PATH_OUTSIDE),
            papyrus: assets.load_with_settings(
                LevelAssets::PATH_PAPYRUS,
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

fn spawn_background(
    In(config): In<SpawnBackground>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        Name::new(config.area.to_string()),
        Background,
        SpriteBundle {
            texture: match config.area {
                Area::Cave => level_assets.cave_background.clone(),
                Area::Outside => level_assets.outside_background.clone(),
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -50.0)),
            ..Default::default()
        },
        StateScoped(config.area),
    ));
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
                _ => todo!("no sprite for {}", config.item.to_string()),
            },
            transform: config.transform,
            ..Default::default()
        },
        StateScoped(state.get().clone()),
    ));
}
