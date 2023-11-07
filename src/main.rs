use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{Category, PluginSignature, Span, SyntaxShape, Value};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

use std::time::Duration;

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("sound make")
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
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let (frequency_value, duration_value, amplify_value) = match load_values(call) {
            Ok(value) => value,
            Err(value) => return value,
        };

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = SineWave::new(frequency_value)
            .take_duration(duration_value)
            .amplify(amplify_value);
        sink.append(source);
        sink.sleep_until_end();
        Ok(Value::nothing(call.head))
    }
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

fn main() {
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}
