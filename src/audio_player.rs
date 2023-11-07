use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::Value;
use rodio::{source::Source, Decoder, OutputStream};

use std::{fs::File, io::BufReader, time::Duration};

pub fn play_audio(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (file_span, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(file_value);
    let source = Decoder::new(file).unwrap();
    let duration = source.total_duration();

    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());
    let opt_duration: Duration = match call.get_flag_value("duration") {
        Some(duration) => match duration.as_duration() {
            Ok(nanos) => Duration::from_nanos(nanos.try_into().unwrap()),
            Err(err) => {
                return Err(LabeledError {
                    label: "duration error".to_string(),
                    msg: err.to_string(),
                    span: Some(file_span),
                })
            }
        },
        None => match duration {
            Some(duration) => duration,
            None => {
                return Err(LabeledError {
                    label: "duration error".to_string(),
                    msg: "cannot get duration of audio file".to_string(),
                    span: Some(file_span),
                })
            }
        },
    };
    std::thread::sleep(opt_duration);
    Ok(Value::nothing(call.head))
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
