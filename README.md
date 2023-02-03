# AudLyrics
A tool to display lyrics playing by Audacious, in the form of KDE plasmoid.

## Requirements

- Audacious is installed, and the bundled `audtool` cli-interface is in `$PATH`.
- Lyrics files have the same name as the corresponding music file, e.g. `Rick Astley - Never Gonna Give You Up.lrc` and `Rick Astley - Never Gonna Give You Up.mp3` and are placed in the same directory.

## Build Requirement
Have Rust toolchain installed.

## Build

1. Build the rust executable using `cargo build --release`
