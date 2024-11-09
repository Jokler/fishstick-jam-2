//! The screen state for the main gameplay.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::Music, screens::Screen};

use super::Area;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GameplayMusic>();
    app.add_systems(
        OnEnter(Area::Cave),
        (stop_ambience, play_cave_ambience).chain(),
    );
    app.add_systems(
        OnEnter(Area::Outside),
        (stop_ambience, play_outside_ambience).chain(),
    );
    app.add_systems(OnExit(Screen::Gameplay), stop_ambience);

    // TODO: Pause screen?
    // app.add_systems(
    //     Update,
    //     return_to_title_screen
    //         .run_if(in_state(Screen::Gameplay).and_then(input_just_pressed(KeyCode::Escape))),
    // );
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct GameplayMusic {
    #[dependency]
    cave_handle: Handle<AudioSource>,
    #[dependency]
    outside_handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            cave_handle: assets.load("audio/music/cave.ogg"),
            outside_handle: assets.load("audio/music/outside.ogg"),
            entity: None,
        }
    }
}

fn play_cave_ambience(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.cave_handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
                Name::from("Cave Ambience"),
            ))
            .id(),
    );
}

fn play_outside_ambience(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.outside_handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
                Name::from("Outside Ambience"),
            ))
            .id(),
    );
}

fn stop_ambience(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}
