use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::primitives::Aabb,
};
use bevy_yarnspinner::prelude::DialogueRunner;
use derive_more::derive::Display;

use super::{
    level::{Level, LevelAssets},
    movement::ActionsFrozen,
    player::{Player, PlayerAssets},
};
use crate::{audio::SoundEffect, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Inventory>();
    app.init_resource::<Inventory>();

    app.add_systems(
        Update,
        ((
            pick_up,
            update_inventory.run_if(resource_changed::<Inventory>),
        )
            .run_if(in_state(Screen::Gameplay)),),
    );
}

#[derive(Component, Reflect, Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Papyrus,
    Knife,
    PapyrusStrips,
    WovenPapyrus,
    Paper,
    Banana,
    BurntBanana,
}

impl From<&str> for Item {
    fn from(name: &str) -> Self {
        match name {
            "Papyrus" => Item::Papyrus,
            "Knife" => Item::Knife,
            "PapyrusStrips" => Item::PapyrusStrips,
            "WovenPapyrus" => Item::WovenPapyrus,
            "Paper" => Item::Paper,
            _ => panic!("unknown item {}", name),
        }
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Inventory {
    pub items: Vec<Item>,
}

fn pick_up(
    // mut gizmos: Gizmos,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<(&Aabb, &Transform), With<Player>>,
    items: Query<(&Aabb, &Transform, &Item)>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut inventory: ResMut<Inventory>,
    mut level: ResMut<Level>,
    player_assets: Res<PlayerAssets>,
) {
    for (player_aabb, player_transform) in &player {
        let player_aabb2d = Aabb2d::new(
            player_transform.translation.xy(),
            player_aabb.half_extents.xy() * player_transform.scale.xy(),
        );
        // gizmos.rect_2d(
        //     player_aabb2d.center(),
        //     0.,
        //     player_aabb2d.half_size() * 2.,
        //     GREEN,
        // );
        for (item_aabb, item_transform, item) in &items {
            let item_aabb2d = Aabb2d::new(
                item_transform.translation.xy(),
                item_aabb.half_extents.xy() * item_transform.scale.xy(),
            );
            // gizmos.rect_2d(item_aabb2d.center(), 0., item_aabb2d.half_size() * 2., RED);
            if player_aabb2d.intersects(&item_aabb2d) {
                if !input.just_pressed(KeyCode::KeyE) {
                    return;
                }
                inventory.items.push(*item);

                if let Some(index) = level.items.iter().position(|x| x == item) {
                    level.items.remove(index);
                }

                commands.spawn((
                    AudioBundle {
                        source: player_assets.item_pickup.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    },
                    SoundEffect,
                    Name::from("Pickup sound"),
                ));

                dialogue_runner
                    .get_single_mut()
                    .expect("only one dialogue runner")
                    .variable_storage_mut()
                    .set(format!("$_has_{}", item), true.into())
                    .unwrap();
            }
        }
    }
}

fn update_inventory(
    mut commands: Commands,
    mut entity: Local<Option<Entity>>,
    inventory: Res<Inventory>,
    level_assets: Res<LevelAssets>,
) {
    if let Some(entity) = *entity {
        commands.entity(entity).despawn_recursive();
    }
    *entity = Some(
        commands
            .inventory_root()
            .insert(StateScoped(Screen::Gameplay))
            .with_children(|children| {
                for item in &inventory.items {
                    let image = match item {
                        Item::Knife => level_assets.knife.clone(),
                        Item::Papyrus => level_assets.papyrus.clone(),
                        Item::PapyrusStrips => level_assets.papyrus_strips.clone(),
                        Item::WovenPapyrus => level_assets.woven_papyrus.clone(),
                        Item::Paper => level_assets.paper.clone(),
                        Item::Banana => level_assets.banana.clone(),
                        Item::BurntBanana => level_assets.burnt_banana.clone(),
                    };
                    children
                        .inventory_item(image)
                        .insert(*item)
                        .observe(interact_item);
                }
            })
            .id(),
    );
}

fn interact_item(
    trigger: Trigger<OnPress>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    items: Query<&Item>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
) {
    if actions_frozen.is_frozen() {
        return;
    }
    let item = items
        .get(trigger.entity())
        .expect("item was inserted on button");
    let mut dialogue_runner = dialogue_runner
        .get_single_mut()
        .expect("only one dialogue runner");

    dialogue_runner.start_node(item.to_string());
    actions_frozen.freeze();
}
