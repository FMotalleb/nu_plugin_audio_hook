use nu_plugin::Plugin;

use crate::{
    audio_meta::{SoundMetaGetCmd, SoundMetaSetCmd},
    audio_player::SoundPlayCmd,
    sound_make::{SoundBeepCmd, SoundMakeCmd},
};

// use crate::make_sound;

#[derive(Default)]
pub struct Sound;

impl Plugin for Sound {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(SoundPlayCmd {}),
            Box::new(SoundMakeCmd {}),
            Box::new(SoundBeepCmd {}),
            Box::new(SoundMetaGetCmd {}),
            Box::new(SoundMetaSetCmd {}),
        ]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}
