use std::time::Duration;

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Value};

use crate::{
    audio_meta::parse_meta,
    audio_player::play_audio,
    sound_make::{make_sound, sine_wave},
};

// use crate::make_sound;

#[derive(Default)]
pub struct Sound;

impl Plugin for Sound {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("sound make")
                .required(
                    "Frequency",
                    SyntaxShape::Float,
                    "Frequency of the sound to make",
                )
                .required("duration", SyntaxShape::Duration, "duration of the sound")
                .named(
                    "amplify",
                    SyntaxShape::Float,
                    "amplify the sound by given value",
                    Some('a'),
                )
                .category(Category::Experimental),
            PluginSignature::build("sound beep").category(Category::Experimental),
            PluginSignature::build("sound meta")
                .required("File Path", SyntaxShape::Filepath, "file to play")
                .category(Category::Experimental),
            PluginSignature::build("sound play")
                .required("File Path", SyntaxShape::Filepath, "file to play")
                .named(
                    "duration",
                    SyntaxShape::Duration,
                    "duration of file (mandatory for non-wave formats like mp3) (default 1 hour)",
                    Some('d'),
                )
                .category(Category::Experimental),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        // let path: Option<Spanned<String>> = call.opt(0)?;
        // return Ok(Value::test_string(name));
        // TODO provide help
        match name {
            "sound make" => make_sound(call),
            "sound beep" => {
                sine_wave(1000.0, Duration::from_millis(300), 1.0);
                return Ok(Value::nothing(call.head));
            }
            "sound play" => play_audio(call),
            "sound meta" => parse_meta(call),
            &_ => {
                return Err(LabeledError {
                    label: "Command not found".to_string(),
                    msg: "WIP".to_string(),
                    span: Some(call.head),
                })
            }
        }
    }
}
