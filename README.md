# nu_plugin_audio_hook

A [nushell](https://www.nushell.sh/) plugin to make and play sounds

## Usage

* sound beep - play a beep sound
* sound make - creates a noise with given frequency and duration
* sound meta - get duration and meta data of an audio file
* sound play - play an audio file, by default supports flac,Wav,mp3 and ogg files, install plugin with `all-decoders` feature to include aac and mp* (audio)

## Examples

* to create a simple noise:

```bash
~> sound make 1000 200ms
```

* create a simple noise sequence

```bash
~> [ 300.0, 500.0,  1000.0, 400.0, 600.0 ] | each { |it| sound make $it 150ms }
```

* play first 3seconds of an audio

```bash
~> sound play audio.mp3 -d 3sec
```

* load metadata of an audio file

```bash
~> sound meta audio.mp4
╭──────────────┬────────────────────────────╮
│ duration     │ 4min 5sec 551ms 20µs 408ns │
│ artist       │ SINGER                     │
│ title        │ TITLE                      │
│ album        │ ALBUM                      │
│ album_artist │ SINGER                     │
│ track_no     │ 1                          │
│ total_tracks │ 1                          │
╰──────────────┴────────────────────────────╯
```

* to correctly play a mp3 audio file you need to first load its metadata and then use its duration to play it

```bash
~> sound meta audio.mp4 | sound play audio.mp3 -d $in.duration
```

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
