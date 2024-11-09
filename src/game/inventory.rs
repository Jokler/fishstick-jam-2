use bevy::{
    color::palettes::css::{GREEN, RED},
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
    render::primitives::Aabb,
};
use bevy_yarnspinner::prelude::{DialogueRunner, YarnProject};
use derive_more::derive::Display;

use super::{level::Level, movement::ActionsFrozen, player::Player};
use crate::{screens::Screen, theme::prelude::*};

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
    mut gizmos: Gizmos,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    player: Query<(&Aabb, &Transform), With<Player>>,
    items: Query<(Entity, &Aabb, &Transform, &Item)>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut inventory: ResMut<Inventory>,
    mut level: ResMut<Level>,
) {
    for (player_aabb, player_transform) in &player {
        let player_aabb2d = Aabb2d::new(
            player_transform.translation.xy(),
            player_aabb.half_extents.xy() * player_transform.scale.xy(),
        );
        gizmos.rect_2d(
            player_aabb2d.center(),
            0.,
            player_aabb2d.half_size() * 2.,
            GREEN,
        );
        for (entity, item_aabb, item_transform, item) in &items {
            let item_aabb2d = Aabb2d::new(
                item_transform.translation.xy(),
                item_aabb.half_extents.xy() * item_transform.scale.xy(),
            );
            gizmos.rect_2d(item_aabb2d.center(), 0., item_aabb2d.half_size() * 2., RED);
            if player_aabb2d.intersects(&item_aabb2d) {
                if !input.just_pressed(KeyCode::KeyE) {
                    return;
                }
                inventory.items.push(*item);

                let index = level.items.iter().position(|x| x == item).unwrap();
                level.items.remove(index);
                commands.entity(entity).despawn_recursive();

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
                    children
                        .button(item.to_string())
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
    let item = items
        .get(trigger.entity())
        .expect("item was inserted on button");
    let mut dialogue_runner = dialogue_runner
        .get_single_mut()
        .expect("only one dialogue runner");

    dialogue_runner.start_node(item.to_string());
    actions_frozen.freeze();
}
