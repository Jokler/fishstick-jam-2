//! A end screen that is shown after the game ends.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::Music, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::End), spawn_end_screen);

    app.load_resource::<EndMusic>();
    app.add_systems(OnEnter(Screen::End), play_end_music);
    app.add_systems(OnExit(Screen::End), stop_music);
}

fn spawn_end_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::End))
        .with_children(|children| {
            children.header("Mission passed!");
            children.big_label("Respect+");
            children.label("");

            children.button("Back").observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct EndMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for EndMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/sound_effects/end.ogg"),
            entity: None,
        }
    }
}

fn play_end_music(mut commands: Commands, mut music: ResMut<EndMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.music.clone(),
                    settings: PlaybackSettings::ONCE,
                },
                Music,
                Name::from("End Music"),
            ))
            .id(),
    );
}

fn stop_music(mut commands: Commands, mut music: ResMut<EndMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}
