use bevy::{
    color::palettes::css::BLACK,
    input::{
        common_conditions::input_just_pressed,
        keyboard::{Key, KeyboardInput},
    },
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::primitives::Aabb,
};
use bevy_yarnspinner::prelude::{DialogueRunner, YarnValue};
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
            update_paper_text,
            update_inventory.run_if(resource_changed::<Inventory>),
        )
            .run_if(in_state(Screen::Gameplay)),),
    );
    app.add_systems(
        Update,
        close_paper
            .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape))),
    );
    app.observe(open_paper);

    app.add_systems(OnEnter(Screen::Gameplay), |mut commands: Commands| {
        commands.insert_resource(Inventory::default())
    });
}

#[derive(Component, Reflect, Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Papyrus,
    Knife,
    PapyrusStrips,
    WovenPapyrus,
    Paper,
    WrittenPaper,
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
            "WrittenPaper" => Item::WrittenPaper,
            "Banana" => Item::Banana,
            "BurntBanana" => Item::BurntBanana,
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
    mut actions_frozen: ResMut<ActionsFrozen>,
) {
    if actions_frozen.is_frozen() {
        return;
    }
    for (player_aabb, player_transform) in &player {
        let player_aabb2d = Aabb2d::new(
            player_transform.translation.xy(),
            player_aabb.half_extents.xy() * player_transform.scale.xy(),
        );
        for (item_aabb, item_transform, item) in &items {
            let item_aabb2d = Aabb2d::new(
                item_transform.translation.xy(),
                item_aabb.half_extents.xy() * item_transform.scale.xy(),
            );
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

                let mut dialogue_runner = dialogue_runner
                    .get_single_mut()
                    .expect("only one dialogue runner");

                dialogue_runner
                    .variable_storage_mut()
                    .set(format!("$_has_{}", item), true.into())
                    .unwrap();

                if item == &Item::Paper {
                    dialogue_runner.start_node("CollectedPaper");
                    actions_frozen.freeze();
                }
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
    if let Some(ecommands) = entity.and_then(|e| commands.get_entity(e)) {
        ecommands.despawn_recursive();
    }
    *entity = Some(
        commands
            .inventory_root()
            .insert(StateScoped(Screen::Gameplay))
            .with_children(|children| {
                for item in &inventory.items {
                    let (image, height) = match item {
                        Item::Knife => (level_assets.knife.clone(), 15.0),
                        Item::Papyrus => (level_assets.papyrus.clone(), 60.0),
                        Item::PapyrusStrips => (level_assets.papyrus_strips.clone(), 50.0),
                        Item::WovenPapyrus => (level_assets.woven_papyrus.clone(), 50.0),
                        Item::Paper => (level_assets.paper.clone(), 50.0),
                        Item::WrittenPaper => (level_assets.written_paper.clone(), 50.0),
                        Item::Banana => (level_assets.banana.clone(), 44.0),
                        Item::BurntBanana => (level_assets.burnt_banana.clone(), 44.0),
                    };
                    children
                        .inventory_item(image, height)
                        .insert(*item)
                        .observe(interact_item);
                }
            })
            .id(),
    );
}

fn interact_item(
    trigger: Trigger<OnPress>,
    mut commands: Commands,
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

    if item == &Item::Paper {
        commands.trigger(OpenPaper);
    } else {
        dialogue_runner.start_node(item.to_string());
        actions_frozen.freeze();
    }
}

#[derive(Event, Debug)]
struct OpenPaper;

#[derive(Reflect, Component, Debug)]
struct Paper;

#[derive(Reflect, Component, Debug)]
struct PaperText;

fn open_paper(
    _: Trigger<OpenPaper>,
    mut commands: Commands,
    mut actions_frozen: ResMut<ActionsFrozen>,
    player_assets: Res<PlayerAssets>,
) {
    if actions_frozen.is_frozen() {
        return;
    }

    commands.spawn((
        Name::new("Paper"),
        Paper,
        SpriteBundle {
            texture: player_assets.paper_big.clone(),
            transform: Transform::from_scale(Vec2::splat(7.0).extend(1.0))
                .with_translation(Vec3::new(0.0, 0.0, 70.0)),
            ..Default::default()
        },
        StateScoped(Screen::Gameplay),
    ));
    commands.spawn((
        Name::new("Paper Text"),
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: player_assets.animal_font.clone(),
                    font_size: 80.0,
                    color: BLACK.into(),
                },
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 80.0)),
            ..default()
        },
        PaperText,
    ));
    actions_frozen.freeze();
}

fn update_paper_text(
    mut events: EventReader<KeyboardInput>,
    dialogue_runner: Query<&DialogueRunner>,
    mut text: Query<&mut Text, With<PaperText>>,
    player_assets: Res<PlayerAssets>,
) {
    let dialogue_runner = dialogue_runner
        .get_single()
        .expect("only one dialogue runner");

    let learned_pen = dialogue_runner
        .variable_storage()
        .get("$learned_pen")
        .unwrap_or(YarnValue::Boolean(false));
    if learned_pen != YarnValue::Boolean(true) {
        return;
    }

    let Ok(mut text) = text.get_single_mut() else {
        return;
    };
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if !event.state.is_pressed() {
            continue;
        }

        let max_width = 6;
        let max_height = 4;
        match &event.logical_key {
            Key::Character(character) => {
                if text.sections.last().unwrap().value.len() >= max_width
                    && text.sections.len() < max_height
                {
                    text.sections.last_mut().unwrap().value.push('\n');
                    text.sections.push(TextSection::new(
                        "",
                        TextStyle {
                            font: player_assets.animal_font.clone(),
                            font_size: 80.0,
                            color: BLACK.into(),
                        },
                    ));
                }
                if let Some(text) = text.sections.last_mut() {
                    if text.value.len() < max_width {
                        text.value.push_str(character);
                    }
                }
            }
            _ => continue,
        }
    }
}

fn close_paper(
    mut commands: Commands,
    paper: Query<Entity, With<Paper>>,
    paper_text: Query<(Entity, &Text), With<PaperText>>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut inventory: ResMut<Inventory>,
    player_assets: Res<PlayerAssets>,
) {
    for entity in &paper {
        commands.entity(entity).despawn_recursive();
        actions_frozen.unfreeze();
    }
    let mut written = false;
    for (entity, text) in &paper_text {
        if !text.sections[0].value.is_empty() {
            written = true;
        }
        commands.entity(entity).despawn_recursive();
    }
    if !written {
        return;
    }

    let from = Item::Paper;
    let to = Item::WrittenPaper;
    let index = inventory.items.iter().position(|x| *x == from).unwrap();
    inventory.items.remove(index);
    inventory.items.push(to);

    let mut dialogue_runner = dialogue_runner
        .get_single_mut()
        .expect("only one dialogue runner");
    let vars = dialogue_runner.variable_storage_mut();

    vars.set(format!("$_has_{}", from), false.into()).unwrap();
    vars.set(format!("$_has_{}", to), true.into()).unwrap();

    commands.spawn((
        AudioBundle {
            source: player_assets.item_pickup.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
        SoundEffect,
        Name::from("Convert sound"),
    ));
}
