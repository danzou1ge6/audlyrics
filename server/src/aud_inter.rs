use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::Error as IOError;
use std::env;

fn home() -> String {
    if let Ok(h) = env::var("HOME") {
        h
    } else {
        panic!("Can't determine HOME of current user");
    }
}

#[derive(Debug)]
pub enum Error {
    BadExitCode(Option<i32>),
    IOError(IOError),
    NonUTF8Stdout
}

impl From<IOError> for Error {
    fn from(value: IOError) -> Self { Self::IOError(value) }
}

pub fn running() -> bool {
    Command::new("audtool")
        .arg("current-song")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Error {e} encountered running audtool. Is audacious installed?"))
        .unwrap()
        .wait_with_output()
        .unwrap()
        .status
        .success()
}

pub fn audtool_command(args: Vec<&'static str>) -> Result<String, Error> {
    let mut cmd = Command::new("audtool");
    cmd.stdout(Stdio::piped());
    cmd.args(args);
    
    let output = cmd.spawn()
        .map_err(|e| format!("Error {e} encountered running audtool. Is audacious installed?"))
        .unwrap()
        .wait_with_output()?;
    
    if !output.status.success() {
        return Err(Error::BadExitCode(output.status.code()));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|_| Error::NonUTF8Stdout)?;
    
    Ok(stdout)
}

pub fn current_song() -> Result<String, Error> {
    audtool_command(vec!["current-song"])
        .map(|x| x.trim().to_string())
}

pub fn current_song_lyric_path() -> Result<PathBuf, Error> {
    let name = audtool_command(vec!["current-song-tuple-data", "file-name"])?;
    let dir = audtool_command(vec!["current-song-tuple-data", "file-path"])?;

    Ok(format!("{}/{}.lrc", dir.trim(), name.trim()).replace('~', &home()).into())
}

pub fn playback_milliseconds() -> Result<i32, Error> {
    match audtool_command(vec!["current-song-output-length-frames"]) {
        Ok(s) => Ok(s.trim().parse().unwrap()),
        Err(e) => Err(e)
    }
}

pub fn playing() -> Result<bool, Error> {
    let output = audtool_command(vec!["playback-status"])?;
    Ok(output.trim() == "playing")
}




