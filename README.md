# AudLyrics
A tool to display lyrics playing by Audacious, in the form of KDE plasmoid.


## Requirements

- Audacious is installed, and the bundled `audtool` cli-interface is in `$PATH`.
- Lyrics files have the same name as the corresponding music file, e.g. `Rick Astley - Never Gonna Give You Up.lrc` and `Rick Astley - Never Gonna Give You Up.mp3` and are placed in the same directory.


## Structure

It consists of two parts

The first part is a server written in Rust, it

- uses `audtool` distributed along with Audacious to acquire the playback information;
- locates and parses the lyrics file;
- serves the now-playing line of lyrics at `127.0.0.1:30123/lyrics`;
- launches Audacious at start-up and exits with Audacious.

The second part is a plasmoid written in QML, it checks for lyrics at an interval of 300 milliseconds and displays accordingly.


## Build Requirement
Have Rust toolchain installed.


## Build and Install

### Manual

1. Build the rust executable in directory `server` using `cargo build --release`.
2. Copy the produced executable `server/release/audlyrics` to anywhere in `$PATH`.
3. use command
```bash
plasmapkg2 -t Plasma/Applet -i plasmoid
```
to install the plasmoid.

### Autamatic

There is a shell script to automate the procedure. Simply execute `arch-install.sh`.

Note that as its name indicates, the script **only supports** Arch based distributions.


## Notes

To fit my own needs, AudLyrics has two non-standard behavior

- Two neighbored lines with the same timestamp are displayed at the same time.
- Any lyrics ended with line "" are ignored.

