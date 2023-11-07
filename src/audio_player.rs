use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::Value;
use rodio::{source::Source, Decoder, OutputStream};

use std::{fs::File, io::BufReader, time::Duration};

pub fn play_audio(call: &EvaluatedCall) -> Result<Value, LabeledError> {
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
            None => return Err(LabeledError {
                label: "duration error".to_string(),
                msg:
                    "cannot get duration of audio file please provide a limited duration using (-d)"
                        .to_string(),
                span: Some(file_span),
            }),
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
