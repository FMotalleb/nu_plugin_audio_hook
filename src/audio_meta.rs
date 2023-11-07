use id3::{Tag, TagLike, Timestamp};
use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{record, Record, Span, Value};
use std::{fs::File, time::Duration};

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
        Err(_) => Duration::from_secs(0),
    };
    other.push(
        "duration",
        Value::duration(duration.as_nanos().try_into().unwrap(), call.head),
    );
    // let info;
    match tags {
        Some(tags) => {
            insert_into_str(&mut other, "artist", tags.artist(), call.head);
            insert_into_str(&mut other, "title", tags.title(), call.head);
            insert_into_str(&mut other, "genre", tags.genre(), call.head);
            insert_into_str(&mut other, "album", tags.album(), call.head);
            insert_into_str(&mut other, "album_artist", tags.album_artist(), call.head);

            insert_into_integer(&mut other, "track_no", tags.track(), call.head);
            insert_into_integer(&mut other, "total_tracks", tags.total_tracks(), call.head);
            insert_into_integer(&mut other, "disc_no", tags.disc(), call.head);
            insert_into_integer(&mut other, "total_discs", tags.total_discs(), call.head);

            //TODO - need conversion
            insert_into_date(&mut other, "date_recorded", tags.date_recorded(), call.head);
            insert_into_date(&mut other, "date_released", tags.date_released(), call.head);
        }
        None => {}
    }

    Ok(Value::record(other, call.head))
    // Ok(Value::nothing(call.head))
}
fn insert_into_str(record: &mut Record, name: &str, val: Option<&str>, span: Span) {
    match val {
        Some(val) => record.push(name, Value::string(val, span)),
        None => {}
    }
}
fn insert_into_date(record: &mut Record, name: &str, val: Option<Timestamp>, span: Span) {
    match val {
        Some(val) => record.push(name, Value::string(val.to_string(), span)),
        None => {}
    }
}
fn insert_into_integer(record: &mut Record, name: &str, val: Option<u32>, span: Span) {
    match val {
        Some(val) => record.push(name, Value::int(val.into(), span)),
        None => {}
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
