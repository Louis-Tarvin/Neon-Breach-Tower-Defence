use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/bgm-main.ogg")]
    pub bgm_main: Handle<AudioSource>,
    #[asset(path = "audio/bgm-drums.ogg")]
    pub bgm_drums: Handle<AudioSource>,
    #[asset(path = "audio/blip1.wav")]
    pub blip1: Handle<AudioSource>,
    #[asset(path = "audio/blip2.wav")]
    pub blip2: Handle<AudioSource>,
    #[asset(path = "audio/end.wav")]
    pub end: Handle<AudioSource>,
    #[asset(path = "audio/place.wav")]
    pub place: Handle<AudioSource>,
    #[asset(path = "audio/kill.wav")]
    pub kill: Handle<AudioSource>,
    #[asset(path = "audio/overheat-start.wav")]
    pub overheat_start: Handle<AudioSource>,
    #[asset(path = "audio/overheat-end.wav")]
    pub overheat_end: Handle<AudioSource>,
    #[asset(path = "audio/explosion.wav")]
    pub explosion: Handle<AudioSource>,
    #[asset(path = "audio/turret-shoot.wav")]
    pub turret_shoot: Handle<AudioSource>,
    #[asset(path = "audio/sniper-shoot.wav")]
    pub sniper_shoot: Handle<AudioSource>,
    #[asset(path = "audio/missile-shoot.wav")]
    pub missile_shoot: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct VolumeSettings {
    pub sfx_vol: f64,
    pub music_vol: f64,
}
impl Default for VolumeSettings {
    fn default() -> Self {
        Self {
            music_vol: 1.0,
            sfx_vol: 1.0,
        }
    }
}

impl VolumeSettings {
    pub fn toggle_sfx_vol(&mut self) {
        self.sfx_vol -= 0.1;
        if self.sfx_vol < 0.0 {
            self.sfx_vol = 1.0;
        }
    }
    pub fn toggle_music_vol(&mut self) {
        self.music_vol -= 0.1;
        if self.music_vol < 0.0 {
            self.music_vol = 1.0;
        }
    }
}

#[derive(Component, Resource, Default, Clone)]
pub struct MusicChannel;
#[derive(Component, Resource, Default, Clone)]
pub struct DrumsChannel;
#[derive(Component, Resource, Default, Clone)]
pub struct SoundChannel;
