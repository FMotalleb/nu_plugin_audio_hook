use id3::{Tag, TagLike};
use nu_plugin::{self, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{record, Category, LabeledError, Record, Signature, Span, SyntaxShape, Value};
use std::{fs::File, path::PathBuf};

use crate::{
    constants::{get_meta_records, ID3_HASHMAP},
    Sound,
};
pub struct SoundMetaSetCmd;
impl SimplePluginCommand for SoundMetaSetCmd {
    type Plugin = Sound;

    fn name(&self) -> &str {
        "sound meta set"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::new("sound meta set")
            .required("File Path", SyntaxShape::Filepath, "file to update")
            .required_named("key", SyntaxShape::String, "id3 key", Some('k'))
            .required_named("value", SyntaxShape::String, "id3 value", Some('v'))
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "set a id3 frame on an audio file"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        audio_meta_set(call)
    }
}

pub struct SoundMetaGetCmd;
impl SimplePluginCommand for SoundMetaGetCmd {
    type Plugin = Sound;

    fn name(&self) -> &str {
        "sound meta"
    }

    fn signature(&self) -> Signature {
        Signature::new("sound meta")
            .switch("all", "List all possible frame names", Some('a'))
            .optional("File Path", SyntaxShape::Filepath, "file to play")
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "get duration and meta data of an audio file"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        if let Ok(true) = call.has_flag("all") {
            return Ok(get_meta_records(call.head));
        }
        parse_meta(call)
    }
}

fn parse_meta(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (_, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let tags = match Tag::read_from2(file_value) {
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
    let duration_nanos = duration.as_nanos().try_into().map_err(|e| {
        LabeledError::new(format!("Failed to convert duration: {e}"))
            .with_label("duration conversion error", call.head)
    })?;
    other.push("duration", Value::duration(duration_nanos, call.head));
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

fn audio_meta_set(call: &EvaluatedCall) -> Result<Value, LabeledError> {
    let (_, file_value) = match load_file(call) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let key = match call.get_flag_value("key") {
        Some(Value::String { val, .. }) => val,
        _ => {
            return Err(LabeledError::new("set key using `-k` flag".to_string())
                .with_label("cannot get value of key", call.head));
        }
    };
    let value = match call.get_flag_value("value") {
        Some(Value::String { val, .. }) => val,
        _ => {
            return Err(LabeledError::new("set value using `-v` flag".to_string())
                .with_label("cannot get value of value", call.head));
        }
    };
    let tags = match Tag::read_from2(file_value) {
        Ok(tags) => Some(tags),
        Err(_) => None,
    };

    if let Some(mut tags) = tags {
        tags.set_text(key, value);

        let (_, path) = match load_file_path(call) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let tr = tags.write_to_path(path, tags.version());
        tr.map_err(|e| {
            LabeledError::new(e.to_string()).with_label("error during writing", call.head)
        })?
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
            return Err(Err(
                LabeledError::new(err.to_string()).with_label("file value error", span)
            ))
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
            return Err(Err(LabeledError::new(err.to_string())
                .with_label("Frequency value not found", call.head)))
        }
    };
    let file_span = file.span();
    let mut loader = File::options();
    loader.write(true);
    let path_value: PathBuf = match file {
        Value::String { val, .. } => PathBuf::from(val),
        _ => return Err(Err(LabeledError::new("cannot get file path".to_string()))),
    };
    Ok((file_span, path_value))
}
