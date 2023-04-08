use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioTween};

use crate::{
    audio::{AudioAssets, DrumsChannel, MusicChannel, SoundChannel, VolumeSettings},
    ui::constants::*,
};

use super::{loading::GameAssets, State};

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
enum MenuButton {
    Start,
    Sound,
    Music,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(super::State::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(super::State::MainMenu)))
            .add_system(update_button_volume_text.in_set(OnUpdate(super::State::MainMenu)))
            .add_system(cleanup.in_schedule(OnExit(super::State::MainMenu)));
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    audio_assets: Res<AudioAssets>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    drums_channel: Res<AudioChannel<DrumsChannel>>,
) {
    music_channel
        .play(audio_assets.bgm_main.clone())
        .looped()
        .fade_in(AudioTween::linear(Duration::from_secs(3)));
    drums_channel
        .play(audio_assets.bgm_drums.clone())
        .looped()
        .with_volume(0.0);
    commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(100.0)),
                max_size: Size::new(Val::Px(1920.0), Val::Px(1080.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                aspect_ratio: Some(1.778),
                ..Default::default()
            },
            image: UiImage {
                texture: game_assets.titlecard.clone(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenuRoot)
        .with_children(|parent| {
            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(300.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
            add_button(parent, "Start", MenuButton::Start, game_assets.font.clone());
            add_button(parent, "Sound", MenuButton::Sound, game_assets.font.clone());
            add_button(parent, "Music", MenuButton::Music, game_assets.font.clone());
        });
}

fn add_button(parent: &mut ChildBuilder, text: &str, button: MenuButton, font: Handle<Font>) {
    let button_style = Style {
        size: Size::new(Val::Px(210.0), Val::Px(65.0)),
        // center button
        margin: UiRect::all(Val::Px(20.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..Default::default()
    };
    parent
        .spawn(ButtonBundle {
            style: button_style,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: BUTTON_TEXT_COLOR,
                },
            ));
        })
        .insert(button);
}

fn button_system(
    mut interaction_query: Query<
        (&MenuButton, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<State>>,
    mut volume_settings: ResMut<VolumeSettings>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    sound_channel: Res<AudioChannel<SoundChannel>>,
    audio_assets: Res<AudioAssets>,
) {
    for (button, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = BUTTON_BACKGROUND_COLOR_PRESSED.into();
                match button {
                    MenuButton::Start => next_state.set(State::Game),
                    MenuButton::Sound => {
                        volume_settings.toggle_sfx_vol();
                        sound_channel.set_volume(volume_settings.sfx_vol);
                        sound_channel.play(audio_assets.blip2.clone());
                    }
                    MenuButton::Music => {
                        volume_settings.toggle_music_vol();
                        music_channel.set_volume(volume_settings.music_vol);
                        sound_channel.play(audio_assets.blip2.clone());
                    }
                }
            }
            Interaction::Hovered => {
                sound_channel.play(audio_assets.blip1.clone());
                *color = BUTTON_BACKGROUND_COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = BUTTON_BACKGROUND_COLOR.into();
            }
        }
    }
}

fn update_button_volume_text(
    query: Query<(&MenuButton, &Children)>,
    mut text_query: Query<&mut Text>,
    volume_settings: Res<VolumeSettings>,
) {
    if !volume_settings.is_changed() {
        return;
    }
    for (button, children) in query.iter() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match button {
            MenuButton::Sound => {
                text.sections[0].value =
                    format!(" Sound: {}% ", (volume_settings.sfx_vol * 100.0).round());
            }
            MenuButton::Music => {
                text.sections[0].value =
                    format!(" Music: {}% ", (volume_settings.music_vol * 100.0).round());
            }
            _ => {}
        }
    }
}

fn cleanup(mut commands: Commands, root: Query<Entity, With<MainMenuRoot>>) {
    for entity in root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
