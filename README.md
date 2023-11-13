# nu_plugin_audio_hook

A [nushell](https://www.nushell.sh/) plugin to make and play sounds

## Usage

* sound beep - play a beep sound
* sound make - creates a noise with given frequency and duration
* sound meta - get duration and meta data of an audio file
* sound meta set - set id3 frame on the audio file (more about [id3 frames](https://docs.puddletag.net/source/id3.html))
* sound play - play an audio file, by default supports flac,Wav,mp3 and ogg files, install plugin with `all-decoders` feature to include aac and mp4 (audio)

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

* to change an id3 frame you can use this command (more about [id3 frames](https://docs.puddletag.net/source/id3.html))

```bash
~> sound meta set audio.mp3 -k TPE1 -v new-artist
╭──────────────┬────────────────────────────╮
│ duration     │ 4min 5sec 551ms 20µs 408ns │
│ artist       │ new-artist                 │
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

* to get all possible id3 frame names

```bash
~> sound meta list
╭────┬─────────────────────────┬────────────╮
│  # │       normalized        │ frame_name │
├────┼─────────────────────────┼────────────┤
│  0 │ audiolength             │ TLEN       │
│  1 │ mood                    │ TMOO       │
...
│ 58 │ track                   │ TRCK       │
├────┼─────────────────────────┼────────────┤
│  # │       normalized        │ frame_name │
╰────┴─────────────────────────┴────────────╯
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

* using [nupm](https://github.com/nushell/nupm)

```bash
git clone https://github.com/FMotalleb/nu_plugin_audio_hook.git
nupm install --path nu_plugin_audio_hook -f
```

* or compile manually

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
