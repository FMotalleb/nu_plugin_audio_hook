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
│  2 │ taggingtime             │ TDTG       │
│  3 │ itunescompilationflag   │ TCMP       │
│  4 │ time                    │ TIME       │
│  5 │ wwwpayment              │ WPAY       │
│  6 │ originalreleasetime     │ TDOR       │
│  7 │ encodedby               │ TENC       │
│  8 │ wwwcopyright            │ WCOP       │
│  9 │ fileowner               │ TOWN       │
│ 10 │ version                 │ TIT3       │
│ 11 │ arranger                │ TPE4       │
│ 12 │ albumsortorder          │ TSOA       │
│ 13 │ copyright               │ TCOP       │
│ 14 │ wwwradio                │ WORS       │
│ 15 │ album                   │ TALB       │
│ 16 │ conductor               │ TPE3       │
│ 17 │ originalalbum           │ TOAL       │
│ 18 │ encodingtime            │ TDEN       │
│ 19 │ grouping                │ TIT1       │
│ 20 │ radioowner              │ TRSO       │
│ 21 │ bpm                     │ TBPM       │
│ 22 │ audiosize               │ TSIZ       │
│ 23 │ originalyear            │ TORY       │
│ 24 │ initialkey              │ TKEY       │
│ 25 │ language                │ TLAN       │
│ 26 │ radiostationname        │ TRSN       │
│ 27 │ wwwpublisher            │ WPUB       │
│ 28 │ setsubtitle             │ TSST       │
│ 29 │ organization            │ TPUB       │
│ 30 │ composer                │ TCOM       │
│ 31 │ producednotice          │ TPRO       │
│ 32 │ wwwfileinfo             │ WOAF       │
│ 33 │ year                    │ TDRC       │
│ 34 │ encodingsettings        │ TSSE       │
│ 35 │ itunesalbumsortorder    │ TSO2       │
│ 36 │ date                    │ TDAT       │
│ 37 │ mediatype               │ TMED       │
│ 38 │ albumartist             │ TPE2       │
│ 39 │ genre                   │ TCON       │
│ 40 │ wwwcommercialinfo       │ WCOM       │
│ 41 │ artist                  │ TPE1       │
│ 42 │ isrc                    │ TSRC       │
│ 43 │ itunescomposersortorder │ TSOC       │
│ 44 │ originalartist          │ TOPE       │
│ 45 │ performersortorder      │ TSOP       │
│ 46 │ wwwsource               │ WOAS       │
│ 47 │ lyricist                │ TEXT       │
│ 48 │ author                  │ TOLY       │
│ 49 │ audiodelay              │ TDLY       │
│ 50 │ recordingdates          │ TRDA       │
│ 51 │ releasetime             │ TDRL       │
│ 52 │ filetype                │ TFLT       │
│ 53 │ title                   │ TIT2       │
│ 54 │ filename                │ TOFN       │
│ 55 │ titlesortorder          │ TSOT       │
│ 56 │ wwwartist               │ WOAR       │
│ 57 │ discnumber              │ TPOS       │
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
