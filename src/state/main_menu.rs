use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{
    audio::{AudioAssets, MusicChannel, SoundChannel, VolumeSettings},
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
            .add_system(cleanup.in_schedule(OnExit(super::State::MainMenu)));
    }
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenuRoot)
        .with_children(|parent| {
            parent
                // Title wrapper
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Unnamed Tower Defense Game",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 60.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });

            add_button(parent, "Start", MenuButton::Start, game_assets.font.clone());
            add_button(parent, "Sound", MenuButton::Sound, game_assets.font.clone());
            add_button(parent, "Music", MenuButton::Music, game_assets.font.clone());
        });
}

fn add_button(parent: &mut ChildBuilder, text: &str, button: MenuButton, font: Handle<Font>) {
    let button_style = Style {
        size: Size::new(Val::Px(195.0), Val::Px(65.0)),
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

fn cleanup(mut commands: Commands, root: Query<Entity, With<MainMenuRoot>>) {
    for entity in root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
