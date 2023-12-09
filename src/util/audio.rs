use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioPlugin, AudioSource};
use bevy_kira_audio::prelude::*;
use std::marker::PhantomData;

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SoundChannel>()
            .add_plugins(AudioPlugin);
    }
}

#[derive(Resource)]
pub struct MusicChannel;
#[derive(Resource)]
pub struct SoundChannel;

#[derive(SystemParam)]
pub struct GameAudio<'w, 's> {
    music_channel: Res<'w, AudioChannel<MusicChannel>>,
    sound_channel: Res<'w, AudioChannel<SoundChannel>>,
    dynamic_channel: ResMut<'w, DynamicAudioChannels>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> GameAudio<'w, 's> {
    pub fn play_bgm(&mut self, handle: &Handle<AudioSource>) {
        self.music_channel.stop();
        #[cfg(not(feature = "no_music"))]
        {
            self.music_channel.set_volume(0.5);
            self.music_channel.play(handle.clone()).looped();
        }
    }

    pub fn stop_bgm(&mut self) {
        self.music_channel.stop();
    }

    pub fn play_sfx(&mut self, handle: &Handle<AudioSource>) {
        self.sound_channel.set_volume(0.5);
        self.sound_channel.play(handle.clone());
    }

    pub fn play_sfx_with_volume(&mut self, handle: &Handle<AudioSource>, channel: &str, volume: f32, looped: bool) {
        let channel =
            if !self.dynamic_channel.is_channel(channel) {
                println!("CREATING");
                 self.dynamic_channel.create_channel(channel)
            } else {
                self.dynamic_channel.channel(channel)
            };

        if looped {
            if !channel.is_playing_sound() {
                println!("er");
                channel.play(handle.clone()).looped();
            }
        } else {
            println!("uh");
            channel.play(handle.clone());
        }
        channel.set_volume(volume as f64);
    }
}
