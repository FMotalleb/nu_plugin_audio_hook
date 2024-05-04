use nu_plugin::{self, EvaluatedCall, LabeledError, SimplePluginCommand};
use nu_protocol::{Signature, Value};
use rodio::{source::Source, Decoder, OutputStream};

use std::{fs::File, io::BufReader, time::Duration};

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
            .plugin_examples(vec![
                PluginExample {
                    description: "play a sound and exits after 5min".to_string(),
                    example: "sound play audio.mp4 -d 5min".to_string(),
                    result: None,
                },
                PluginExample {
                    description: "play a sound for its duration".to_string(),
                    example: "sound meta audio.mp4 | sound play audio.mp3 -d $in.duration"
                        .to_string(),
                    result: None,
                },
            ])
            .category(Category::Experimental)
    }

    fn usage(&self) -> &str {
        "play an audio file, by default supports flac,Wav,mp3 and ogg files, install plugin with `all-decoders` feature to include aac and mp4(audio)"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        play_audio(call)
    }
}

fn play_audio(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (file_span, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok(value) => value,
        Err(err) => {
            return Err(LabeledError {
                label: "audio stream exception".to_string(),
                msg: err.to_string(),
                span: Some(call.head),
            })
        }
    };
    let file = BufReader::new(file_value);

    let source = match Decoder::new(file) {
        Ok(value) => value,
        Err(err) => {
            return Err(LabeledError {
                label: "audio decoder exception".to_string(),
                msg: err.to_string(),
                span: Some(file_span),
            })
        }
    };

    let duration = source.total_duration();

    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => {}
        Err(err) => {
            return Err(LabeledError {
                label: "audio player exception".to_string(),
                msg: err.to_string(),
                span: Some(file_span),
            })
        }
    }

    let sleep_duration: Duration = match load_duration_from(call, "duration") {
        Some(duration) => duration,
        None => match duration {
            Some(duration) => duration,
            None => Duration::from_secs(3600),
        },
    };

    std::thread::sleep(sleep_duration);
    Ok(Value::nothing(call.head))
}
fn load_duration_from(call: &EvaluatedCall, name: &str) -> Option<Duration> {
    match call.get_flag_value(name) {
        Some(duration) => match duration.as_duration() {
            Ok(nanos) => Some(Duration::from_nanos(match nanos.try_into() {
                Ok(nanos) => nanos,
                Err(_) => return None,
            })),
            Err(_) => None,
        },
        None => None,
    }
}
fn load_file(
    call: &EvaluatedCall,
) -> Result<(nu_protocol::Span, File), Result<Value, LabeledError>> {
    let file: Value = match call.req(0) {
        Ok(value) => value,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value not found".to_string(),
                msg: err.to_string(),
                span: Some(call.head),
            }))
        }
    };
    let file_span = file.span();
    let file_value: File = match file.as_path() {
        Ok(value) => match File::open(value) {
            Ok(file) => file,
            Err(err) => {
                return Err(Err(LabeledError {
                    label: "file value error".to_string(),
                    msg: err.to_string(),
                    span: Some(file_span),
                }))
            }
        },
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value must be of type Float (f32)".to_string(),
                msg: err.to_string(),
                span: Some(file_span),
            }))
        }
    };
    Ok((file_span, file_value))
}
