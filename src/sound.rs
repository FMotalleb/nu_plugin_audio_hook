use std::time::Duration;

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Value};

use crate::sound_make::{make_sound, sine_wave};

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
