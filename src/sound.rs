use std::time::Duration;

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{record, Category, PluginExample, PluginSignature, SyntaxShape, Value};

use crate::{
    audio_meta::{audio_meta_set, parse_meta, SoundMetaGetCmd, SoundMetaSetCmd},
    audio_player::{play_audio, SoundPlayCmd},
    constants::get_meta_records,
    sound_make::{make_sound, sine_wave, SoundMakeCmd},
};

// use crate::make_sound;

#[derive(Default)]
pub struct Sound;

impl Plugin for Sound {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(SoundPlayCmd {}),
            Box::new(SoundMakeCmd {}),
            Box::new(SoundMetaGetCmd {}),
            Box::new(SoundMetaSetCmd {}),
        ]
    }
}
