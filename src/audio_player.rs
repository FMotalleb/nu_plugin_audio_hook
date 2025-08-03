use chrono::Utc;
use nu_plugin::{self, EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, SyntaxShape, Value};
use rodio::{source::Source, Decoder, OutputStream};

use std::{fs::File, io::BufReader, time::Duration};

use crate::Sound;

pub struct SoundPlayCmd;
impl SimplePluginCommand for SoundPlayCmd {
    type Plugin = Sound;

    fn name(&self) -> &str {
        "sound play"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new("sound play")
            .required("File Path", SyntaxShape::Filepath, "file to play")
            .named(
                "duration",
                SyntaxShape::Duration,
                "duration of file (mandatory for non-wave formats like mp3) (default 1 hour)",
                Some('d'),
            )
            .category(Category::Experimental)
    }
    fn examples(&self) -> Vec<Example<'_>> {
        vec![
            Example {
                description: "play a sound and exits after 5min",
                example: "sound play audio.mp4 -d 5min",
                result: None,
            },
            Example {
                description: "play a sound for its duration",
                example: "sound meta audio.mp4 | sound play audio.mp3 -d $in.duration",
                result: None,
            },
        ]
    }
    fn description(&self) -> &str {
        "play an audio file, by default supports flac,Wav,mp3 and ogg files, install plugin with `all-decoders` feature to include aac and mp4(audio)"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        play_audio(engine, call)
    }
}

fn play_audio(engine: &EngineInterface, call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (file_span, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(value) => value,
        Err(err) => {
            return Err(
                LabeledError::new(err.to_string()).with_label("audio stream exception", call.head)
            )
        }
    };
    let file = BufReader::new(file_value);

    let source = match Decoder::new(file) {
        Ok(value) => value,
        Err(err) => {
            return Err(
                LabeledError::new(err.to_string()).with_label("audio decoder exception", file_span)
            )
        }
    };

    let duration = source.total_duration();

    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => {}
        Err(err) => {
            return Err(
                LabeledError::new(err.to_string()).with_label("audio player exception", file_span)
            )
        }
    }

    let sleep_duration: Duration = match load_duration_from(call, "duration") {
        Some(duration) => duration,
        None => match duration {
            Some(duration) => duration,
            None => Duration::from_secs(3600),
        },
    };

    let sleep_until = Utc::now() + sleep_duration;

    // We check for OS signals
    while engine.signals().check(&call.head).map(|_| true)? && Utc::now() < sleep_until {
        // We yield to the OS until necessary
        std::thread::yield_now();
    }

    Ok(Value::nothing(call.head))
}
fn load_duration_from(call: &EvaluatedCall, name: &str) -> Option<Duration> {
    match call.get_flag_value(name) {
        Some(Value::Duration { val, .. }) => {
            Some(Duration::from_nanos(u64::from_ne_bytes(val.to_ne_bytes())))
        }
        _ => None,
    }
}
fn load_file(
    call: &EvaluatedCall,
) -> Result<(nu_protocol::Span, File), Result<Value, LabeledError>> {
    let file: Value = match call.req(0) {
        Ok(value) => value,
        Err(err) => {
            return Err(Err(LabeledError::new(err.to_string())
                .with_label("Frequency value not found", call.head)))
        }
    };
    let file_span = file.span();
    let file_value: File = match file {
        Value::String { val, .. } => match File::open(val) {
            Ok(file) => file,
            Err(err) => {
                return Err(Err(LabeledError::new(err.to_string())
                    .with_label("error trying to open the file", file_span)))
            }
        },
        _ => return Err(Err(LabeledError::new("cannot access file path"))),
    };
    Ok((file_span, file_value))
}
