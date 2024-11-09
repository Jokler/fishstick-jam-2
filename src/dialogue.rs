use bevy::prelude::*;
use bevy_yarnspinner::{
    events::DialogueCompleteEvent,
    prelude::{DialogueRunner, YarnFileSource, YarnProject, YarnSpinnerPlugin},
};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;

use crate::{
    game::{
        dino::SpawnDino,
        inventory::{Inventory, Item},
        level::Level,
        movement::ActionsFrozen,
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
            YarnFileSource::file("dialogue/dino.yarn"),
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
        .add_command("spawn_dino", spawn_dino);

    fn inventory_convert(
        In((from, to)): In<(String, String)>,
        mut dialogue_runner: Query<&mut DialogueRunner>,
        mut inventory: ResMut<Inventory>,
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
        mut dialogue_runner: Query<&mut DialogueRunner>,
        mut inventory: ResMut<Inventory>,
        mut level: ResMut<Level>,
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
    }

    fn spawn_dino(In(()): In<()>, mut commands: Commands) {
        commands.trigger(SpawnDino);
    }

    dialogue_runner.start_node("Intro");
    commands.spawn(dialogue_runner);
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
