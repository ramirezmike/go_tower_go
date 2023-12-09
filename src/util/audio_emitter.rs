use bevy::prelude::*;
use crate::{AppState, ingame::player, ingame::config, util::audio};
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::prelude::*;

pub struct AudioEmitterPlugin;

impl Plugin for AudioEmitterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpacialAudio { max_distance: config::AUDIO_DISTANCE });
        //app.add_systems(Update, emit_audios.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, Default)]
pub struct AudioEmitter {
    pub id: String,
    pub time_to_live: Option<Timer>,
    pub looped: bool,
    pub audio: Handle<AudioSource>,
}

fn emit_audios( 
    mut commands: Commands,
    mut audio: audio::GameAudio,
    mut emitters: Query<(Entity, &mut AudioEmitter, &Transform)>,
    players: Query<&Transform, With<player::Player>>,
    time: Res<Time>
) {
    for (entity, mut emitter, emitter_transform) in &mut emitters {
        if emitter.time_to_live.is_some() && emitter.time_to_live.as_mut().unwrap().tick(time.delta()).finished() {
            commands.entity(entity).remove::<AudioEmitter>();
        } else {
            for player in &players {
                let distance = player.translation.distance(emitter_transform.translation);
                if distance < config::AUDIO_DISTANCE {
                    let volume = 1. - (distance / config::AUDIO_DISTANCE);
                    audio.play_sfx_with_volume(&emitter.audio, &emitter.id, volume, emitter.looped);
                }
            }
        }
    }
}
