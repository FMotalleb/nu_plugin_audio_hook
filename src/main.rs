use nu_plugin_audio_hook::Sound;

fn main() {
    nu_plugin::serve_plugin(&mut Sound {}, nu_plugin::MsgPackSerializer {})
}
