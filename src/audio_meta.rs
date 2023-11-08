use id3::{Tag, TagLike};
use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{record, Record, Span, Value};
use std::{fs::File, path::PathBuf};

use crate::constants::ID3_HASHMAP;

pub fn parse_meta(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (_, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let tags = match Tag::read_from(file_value) {
        Ok(tags) => Some(tags),
        Err(_) => None,
    };
    let mut other = record! {};
    let (_, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let duration = match mp3_duration::from_file(&file_value) {
        Ok(duration) => duration,
        Err(err) => err.at_duration,
    };
    other.push(
        "duration",
        Value::duration(duration.as_nanos().try_into().unwrap(), call.head),
    );
    // let info;
    match tags {
        Some(tags) => {
            for (key, val) in ID3_HASHMAP.iter() {
                if let Some(result) = tags.get(val) {
                    insert_into_str(
                        &mut other,
                        key,
                        Some(result.content().to_string()),
                        call.head,
                    )
                }
            }
            // insert_into_str(&mut other, "artist", tags.artist(), call.head);
            // insert_into_str(&mut other, "title", tags.title(), call.head);
            // insert_into_str(&mut other, "genre", tags.genre(), call.head);
            // insert_into_str(&mut other, "album", tags.album(), call.head);
            // insert_into_str(&mut other, "album_artist", tags.album_artist(), call.head);

            insert_into_integer(&mut other, "track_no", tags.track(), call.head);
            insert_into_integer(&mut other, "total_tracks", tags.total_tracks(), call.head);
            insert_into_integer(&mut other, "disc_no", tags.disc(), call.head);
            insert_into_integer(&mut other, "total_discs", tags.total_discs(), call.head);

            // //TODO - need conversion
            // insert_into_date(&mut other, "date_recorded", tags.date_recorded(), call.head);
            // insert_into_date(&mut other, "date_released", tags.date_released(), call.head);
        }
        None => {}
    }

    Ok(Value::record(other, call.head))
    // Ok(Value::nothing(call.head))
}
pub fn audio_meta_set(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (_, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let key = match call.get_flag_value("key") {
        Some(value) => match value.as_string() {
            Ok(value) => value,
            Err(err) => {
                return Err(LabeledError {
                    label: "cannot get value of key".to_string(),
                    msg: err.to_string(),
                    span: Some(value.span()),
                })
            }
        },
        None => {
            return Err(LabeledError {
                label: "cannot get value of key".to_string(),
                msg: "set key using `-k` flag".to_string(),
                span: None,
            })
        }
    };
    let value = match call.get_flag_value("value") {
        Some(value) => match value.as_string() {
            Ok(value) => value,
            Err(err) => {
                return Err(LabeledError {
                    label: "cannot get value of value".to_string(),
                    msg: err.to_string(),
                    span: Some(value.span()),
                })
            }
        },
        None => {
            return Err(LabeledError {
                label: "cannot get value of `value`".to_string(),
                msg: "set value using `-v` flag".to_string(),
                span: None,
            })
        }
    };
    let tags = match Tag::read_from(file_value) {
        Ok(tags) => Some(tags),
        Err(_) => None,
    };

    if let Some(mut tags) = tags {
        tags.set_text(key, value);

        let (_, path) = match load_file_path(call) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let tt = tags.write_to_path(path, tags.version());
        if tt.is_err() {
            return Err(LabeledError {
                label: "error during writing".to_string(),
                msg: tt.err().unwrap().to_string(),
                span: None,
            });
        }
    }
    parse_meta(call)
}
fn insert_into_str(
    record: &mut Record,
    name: impl AsRef<str>,
    val: Option<impl AsRef<str>>,
    span: Span,
) {
    match val {
        Some(val) => record.push(name.as_ref(), Value::string(val.as_ref(), span)),
        None => {}
    }
}

// fn insert_into_date(record: &mut Record, name: &str, val: Option<Timestamp>, span: Span) {
//     match val {
//         Some(val) => record.push(name, Value::string(val.to_string(), span)),
//         None => {}
//     }
// }
fn insert_into_integer(record: &mut Record, name: &str, val: Option<u32>, span: Span) {
    match val {
        Some(val) => record.push(name, Value::int(val.into(), span)),
        None => {}
    }
}

fn load_file(
    call: &EvaluatedCall,
) -> Result<(nu_protocol::Span, File), Result<Value, LabeledError>> {
    let (span, path) = match load_file_path(call) {
        Ok(value) => value,
        Err(value) => return Err(value),
    };
    let file_value: File = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "file value error".to_string(),
                msg: err.to_string(),
                span: Some(span),
            }))
        }
    };
    Ok((span, file_value))
}
fn load_file_path(
    call: &EvaluatedCall,
) -> Result<(nu_protocol::Span, PathBuf), Result<Value, LabeledError>> {
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
    let mut loader = File::options();
    loader.write(true);
    let path_value: PathBuf = match file.as_path() {
        Ok(value) => value,
        Err(err) => {
            return Err(Err(LabeledError {
                label: "Frequency value must be of type Float (f32)".to_string(),
                msg: err.to_string(),
                span: Some(file_span),
            }))
        }
    };
    Ok((file_span, path_value))
}
