use std::time::Duration;

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{record, Category, PluginExample, PluginSignature, SyntaxShape, Value};

use crate::{
    audio_meta::{audio_meta_set, parse_meta},
    audio_player::play_audio,
    sound_make::{make_sound, sine_wave},
};

// use crate::make_sound;

#[derive(Default)]
pub struct Sound;

impl Plugin for Sound {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("sound")
            .usage("a nushell plugin that can make noise, read audio file's metadata and play audio files")
            .plugin_examples(
                vec![
                    PluginExample {
                        description: "create a simple noise frequency".to_string(),
                        example: "sound make 1000 200ms".to_string(),
                        result: None,
                    },
                    PluginExample {
                        description: "create a simple noise sequence".to_string(),
                        example: "[ 300.0, 500.0,  1000.0, 400.0, 600.0 ] | each { |it| sound make $it 150ms }".to_string(),
                        result: None,
                    },
                    PluginExample {
                        description: "play a sound and exits after 5min".to_string(),
                        example: "sound play audio.mp4 -d 5min".to_string(),
                        result: None,
                    },
                    PluginExample {
                        description: "play a sound for its duration".to_string(),
                        example: "sound meta audio.mp4 | sound play audio.mp3 -d $in.duration".to_string(),
                        result: None,
                    },  
                    PluginExample {
                        description: "set artist of `audio.mp3` to `new-artist`".to_string(),
                        example: "sound meta set audio.mp3 -k TPE1 -v new-artist".to_string(),
                        result: None,
                    },
                ]
            ),
            PluginSignature::build("sound make")
                .usage("creates a noise with given frequency and duration")
                .required("Frequency", SyntaxShape::Float, "Frequency of the noise")
                .required("duration", SyntaxShape::Duration, "duration of the noise")
                .named(
                    "amplify",
                    SyntaxShape::Float,
                    "amplify the sound by given value",
                    Some('a'),
                )
                .plugin_examples(
                    vec![
                        PluginExample {
                            description: "create a simple noise frequency".to_string(),
                            example: "sound make 1000 200ms".to_string(),
                            result: None,
                        },
                        PluginExample {
                            description: "create a simple noise sequence".to_string(),
                            example: "[ 300.0, 500.0,  1000.0, 400.0, 600.0 ] | each { |it| sound make $it 150ms }".to_string(),
                            result: None,
                        },
                    ]
                )
                .category(Category::Experimental),
            PluginSignature::build("sound beep")
                .usage("play a beep sound")
                .category(Category::Experimental)
                .usage("play a beep sound")
                .category(Category::Experimental),
            PluginSignature::build("sound meta set") 
                .usage("set a id3 frame on an audio file")
                .required("File Path", SyntaxShape::Filepath, "file to update")
                .required_named("key", SyntaxShape::String, "id3 key", Some('k'))
                .required_named("value", SyntaxShape::String, "id3 value", Some('v'))
                .category(Category::Experimental),
            PluginSignature::build("sound meta")
                .required("File Path", SyntaxShape::Filepath, "file to play")
                .usage("get duration and meta data of an audio file")
                .category(Category::Experimental),
            PluginSignature::build("sound play")
                .usage("play an audio file, by default supports flac,Wav,mp3 and ogg files, install plugin with `all-decoders` feature to include aac and mp4(audio)")
                .required("File Path", SyntaxShape::Filepath, "file to play")
                .named(
                    "duration",
                    SyntaxShape::Duration,
                    "duration of file (mandatory for non-wave formats like mp3) (default 1 hour)",
                    Some('d'),
                ).plugin_examples(
                    vec![
                        PluginExample {
                            description: "play a sound and exits after 5min".to_string(),
                            example: "sound play audio.mp4 -d 5min".to_string(),
                            result: None,
                        },
                        PluginExample {
                            description: "play a sound for its duration".to_string(),
                            example: "sound meta audio.mp4 | sound play audio.mp3 -d $in.duration".to_string(),
                            result: None,
                        },
                    ]
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
        match name {
            "sound make" => make_sound(call),
            "sound beep" => {
                sine_wave(1000.0, Duration::from_millis(300), 1.0);
                return Ok(Value::nothing(call.head));
            }
            "sound play" => play_audio(call),
            "sound meta" => parse_meta(call),
            "sound meta set" => audio_meta_set(call),
            &_ => Ok(Value::record(
                record! {
                    "sound make {frequency} {duration} -a {amplify}"=>Value::string("creates a noise with given frequency and duration", call.head),
                    "sound beep"=>Value::string("creates a beep sound (equivalent to `sound make 1000 300ms`)", call.head),
                    "sound play {file_path} -d {duration}"=>Value::string("creates a noise with given frequency and duration", call.head),
                    "sound meta {file_path}"=>Value::string("creates a noise with given frequency and duration", call.head)
                },
                call.head,
            )),
        }
    }
}
