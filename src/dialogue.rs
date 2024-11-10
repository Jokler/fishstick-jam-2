use bevy::prelude::*;
use bevy_yarnspinner::{
    events::DialogueCompleteEvent,
    prelude::{DialogueRunner, YarnFileSource, YarnProject, YarnSpinnerPlugin},
};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

use crate::{
    audio::SoundEffect,
    game::{
        dino::SpawnDino,
        inventory::{Inventory, Item},
        level::Level,
        movement::ActionsFrozen,
        player::{AutoRunner, Player, PlayerAssets},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::with_yarn_sources(vec![
            YarnFileSource::file("dialogue/intro.yarn"),
            YarnFileSource::file("dialogue/wife.yarn"),
            YarnFileSource::file("dialogue/knife.yarn"),
            YarnFileSource::file("dialogue/papyrus.yarn"),
            YarnFileSource::file("dialogue/papyrus_strips.yarn"),
            YarnFileSource::file("dialogue/woven_papyrus.yarn"),
            YarnFileSource::file("dialogue/paper.yarn"),
            YarnFileSource::file("dialogue/dino.yarn"),
            YarnFileSource::file("dialogue/banan.yarn"),
            YarnFileSource::file("dialogue/fire.yarn"),
        ]),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_dialogue_runner);
    app.add_systems(Update, unfreeze_after_dialog);
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    mut actions_frozen: ResMut<ActionsFrozen>,
) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command("inventory_convert", inventory_convert)
        .add_command("level_convert", level_convert)
        .add_command("drop", drop)
        .add_command("spawn_dino", spawn_dino)
        .add_command("player_run", player_run)
        .add_command("play_sound", play_sound)
        .add_command("end_game", end_game);

    fn inventory_convert(
        In((from, to)): In<(String, String)>,
        mut commands: Commands,
        mut dialogue_runner: Query<&mut DialogueRunner>,
        mut inventory: ResMut<Inventory>,
        player_assets: Res<PlayerAssets>,
    ) {
        let index = inventory
            .items
            .iter()
            .position(|x| *x.to_string() == from)
            .unwrap();
        inventory.items.remove(index);
        inventory.items.push(Item::from(to.as_str()));

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

    fn level_convert(In((from, to)): In<(String, String)>, mut level: ResMut<Level>) {
        let index = level
            .items
            .iter()
            .position(|x| *x.to_string() == from)
            .unwrap();
        level.items.remove(index);
        level.items.push(Item::from(to.as_str()));
    }

    fn drop(
        In(item): In<String>,
        mut commands: Commands,
        mut dialogue_runner: Query<&mut DialogueRunner>,
        mut inventory: ResMut<Inventory>,
        mut level: ResMut<Level>,
        player_assets: Res<PlayerAssets>,
    ) {
        let index = inventory
            .items
            .iter()
            .position(|x| *x.to_string() == item)
            .unwrap();
        let item = inventory.items.remove(index);
        level.items.push(item);

        let mut dialogue_runner = dialogue_runner
            .get_single_mut()
            .expect("only one dialogue runner");
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
    }

    fn player_run(
        In((direction, end_position)): In<(String, f32)>,
        mut commands: Commands,
        player: Query<Entity, With<Player>>,
    ) {
        let intent = match direction.as_str() {
            "left" => Vec2::new(-1.0, 0.0),
            "right" => Vec2::new(1.0, 0.0),
            _ => panic!("unknown direction {direction}"),
        };
        let entity = player.get_single().expect("exactly one player");
        commands.entity(entity).insert(AutoRunner {
            end_position,
            intent,
        });
    }

    fn spawn_dino(In(()): In<()>, mut commands: Commands) {
        commands.trigger(SpawnDino);
    }

    fn end_game(In(()): In<()>, mut next_state: ResMut<NextState<Screen>>) {
        next_state.set(Screen::End);
    }

    fn play_sound(In(name): In<String>, mut commands: Commands, player_assets: Res<PlayerAssets>) {
        let sound = match name.as_str() {
            "vine_boom" => player_assets.vine_boom.clone(),
            "uh_oh" => player_assets.uh_oh.clone(),
            "trophy_wife" => player_assets.trophy_wife.clone(),
            "wife_hm" => player_assets.wife_hm.clone(),
            _ => panic!("unknown sound {name}"),
        };
        commands.spawn((
            AudioBundle {
                source: sound,
                settings: PlaybackSettings::DESPAWN,
            },
            SoundEffect,
            Name::from(format!("{name} sound")),
        ));
    }

    dialogue_runner.start_node("Intro");
    commands.spawn((dialogue_runner, StateScoped(Screen::Gameplay)));
    actions_frozen.freeze();
}

fn unfreeze_after_dialog(
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
    mut freeze: ResMut<ActionsFrozen>,
) {
    for _event in dialogue_complete_event.read() {
        freeze.unfreeze();
    }
}
