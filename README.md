# nu_plugin_audio_hook

A [nushell](https://www.nushell.sh/) plugin to make and play sounds

## Examples

## Installing

* supported features:
  * full (will enable everything below)
  * flac (default)
  * vorbis (default)
  * wav (default)
  * minimp3
  * symphonia-all (will enable everything below)
    * symphonia-aac
    * symphonia-flac
    * symphonia-isomp4
    * symphonia-mp3 (default)
    * symphonia-vorbis
    * symphonia-wav

* via git

```bash
git clone https://github.com/FMotalleb/nu_plugin_audio_hook.git
cd nu_plugin_audio_hook
cargo build -r --features=all-decoders
register target/debug/nu_plugin_audio_hook 
```

* or using cargo

```bash
cargo install nu_plugin_audio_hook --features=all-decoders
register ~/.cargo/bin/nu_plugin_audio_hook
```
