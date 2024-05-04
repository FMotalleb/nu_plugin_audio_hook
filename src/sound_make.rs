use nu_plugin::{self, EvaluatedCall, LabeledError, PluginCommand, SimplePluginCommand};
use nu_protocol::{Signature, Span, Value};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

use std::time::Duration;

use crate::Sound;

pub struct SoundMakeCmd;

impl SimplePluginCommand for SoundMakeCmd {
    type Plugin = Sound;

    fn name(&self) -> &str {
        "sound make"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new("sound make")
                .required("Frequency", SyntaxShape::Float, "Frequency of the noise")
                .required("duration", SyntaxShape::Duration, "duration of the noise")
                .named(
                    "amplify",
                    SyntaxShape::Float,
                    "amplify the sound by given value",
                    Some('a'),
                ).named(
                    "beep",
                    SyntaxShape::Float,
                    "just a beep sound",
                    Some('b'),
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
                .category(Category::Experimental)
    }

    fn usage(&self) -> &str {
        "creates a noise with given frequency and duration"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        if let Ok(true) = call.has_flag("beep") {
            sine_wave(1000.0, Duration::from_millis(300), 1.0);
            return Ok(Value::nothing(call.head));
        }
        make_sound(call)
    }
}

fn make_sound(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (frequency_value, duration_value, amplify_value) = match load_values(call) {
        Ok(value) => value,
        Err(value) => return value,
    };

    sine_wave(frequency_value, duration_value, amplify_value);
    Ok(Value::nothing(call.head))
}

fn sine_wave(frequency_value: f32, duration_value: Duration, amplify_value: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(frequency_value)
        .take_duration(duration_value)
        .amplify(amplify_value);
    sink.append(source);
    sink.sleep_until_end();
}

fn load_values(call: &EvaluatedCall) -> Result<(f32, Duration, f32), Result<Value, LabeledError>> {
    let frequency: Value = match call.req(0) {
        Ok(value) => value,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value not found".to_string(),
                msg: err.to_string(),
                span: Some(call.head),
            }))
        }
    };
    let frequency_value: f32 = match frequency.as_f64() {
        Ok(value) => value as f32,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value must be of type Float (f32)".to_string(),
                msg: err.to_string(),
                span: Some(frequency.span()),
            }))
        }
    };
    let duration: Value = match call.req(1) {
        Ok(value) => value,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Duration value not found".to_string(),
                msg: err.to_string(),
                span: Some(call.head),
            }))
        }
    };
    let duration_value: Duration = match duration.as_duration() {
        Ok(value) => Duration::from_nanos(value.try_into().unwrap_or(0)),
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value must be of type Float (f32)".to_string(),
                msg: err.to_string(),
                span: Some(frequency.span()),
            }))
        }
    };
    let amplify: Value = match call.get_flag("amplify") {
        Ok(value) => match value {
            Some(value) => value,
            None => Value::float(1.0, Span::unknown()),
        },
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Duration value not found".to_string(),
                msg: err.to_string(),
                span: Some(call.head),
            }))
        }
    };
    let amplify_value: f32 = match amplify.as_float() {
        Ok(value) => value as f32,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value must be of type Float (f32)".to_string(),
                msg: err.to_string(),
                span: Some(frequency.span()),
            }))
        }
    };
    Ok((frequency_value, duration_value, amplify_value))
}
