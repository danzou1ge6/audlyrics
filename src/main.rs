use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::server::conn::AddrStream;

use std::convert::Infallible;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use tokio::process::Command;

use audlyric_server::lyrics::{Lyrics, Line, ParseLyricError};
use audlyric_server::aud_inter as aud;

extern crate derive_more;
use derive_more::From;

#[derive(Debug)]
struct State {
    running: bool,
    playing: bool,
    song: String,
    playback_ms: i32,
    lyrics: Option<Lyrics>,
}

#[derive(Debug, From)]
enum Error {
    AudError(aud::Error),
    LrcParseError(ParseLyricError),
    IoError(std::io::Error),
    LrcNotUTF8(std::string::FromUtf8Error),
}

impl State {
    fn update(&mut self) -> Result<(), Error> {
        self.running = aud::running();

        if !self.running {
            return Ok(());
        }

        self.playing = aud::playing()?;
        self.playback_ms = aud::playback_milliseconds()?;
            
        let song = aud::current_song()?;
        if song == self.song {
            return Ok(());
        }
        self.song = song;

        return self.update_lyrics();
    }

    fn current_line(&self) -> String {
        if !self.playing {
            return String::from("Paused");
        }
        match &self.lyrics {
            None => String::new(),
            Some(lrc) => {
                
                for (i, Line { t, s: _ }) in lrc.lines.iter().enumerate() {
                    if *t > self.playback_ms {
                        if i == 0 {
                            return self.song.clone();
                        } else {
                            return lrc.lines[i - 1].s.clone();
                        }
                    }
                }

                if let Some(last) = lrc.lines.last() {
                    last.s.clone()
                } else {
                    String::new() 
                }

            }
        }
    }

    fn update_lyrics(&mut self) -> Result<(), Error> {
        let lrc_path = aud::current_song_lyric_path()?;
        let mut lrc_f = File::open(lrc_path)?;
        let mut lrc_buf = Vec::new();
        lrc_f.read_to_end(&mut lrc_buf)?;
        let lrc_buf = String::from_utf8(lrc_buf)?;

        match Lyrics::parse(&lrc_buf) {
            Ok(lrc) => self.lyrics = Some(lrc),
            Err(e) => {
                self.lyrics = None;
                return Err(e.into())
            }
        }
        return Ok(())
    }

    fn new() -> Self {
        Self {
            running: false,
            playing: false,
            playback_ms: 0,
            song: String::new(),
            lyrics: None
        }
    }
}

async fn handle(
    state: Arc<Mutex<State>>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {

    let mut resp = Response::new(Body::empty());
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/lyric") => {

            let mut state = state.lock().unwrap();

            match state.update() {
                Err(Error::AudError(e)) => {
                    panic!("{:?}", e);
                },
                Err(e) => {
                    eprintln!("{}: {:?}", &state.song, e);
                },
                Ok(()) => ()
            }

            *resp.body_mut() = Body::from(state.current_line());

        },
        _ => {
            *resp.status_mut() = StatusCode::NOT_FOUND; 
        }
    }

    Ok::<_, Infallible>(resp)
}

#[tokio::main]
async fn main() {

    let mut child = Command::new("audacious").spawn()
        .expect("Cannot start audacious. Is it Installed?");
    

    let state = Arc::new(Mutex::new(State::new()));

    let make_service = make_service_fn(move |_: &AddrStream| {
        let state = state.clone();
        
        let service = service_fn(
        move |req: Request<Body>| {
            handle(state.clone(), req)
        });

        async move {
            Ok::<_, Infallible>(service)
        }
    });

    let addr = ([127, 0, 0, 1], 30123).into();
    let server = Server::bind(&addr).serve(make_service);

    let grace = server.with_graceful_shutdown(async move {
        match child.wait().await {
            Ok(code) => println!("Audacious returned with {}", code),
            Err(e) => println!("Audacious returned with IoError {:?}", e)
        };
    });

    if let Err(e) = grace.await {
        eprintln!("Server error: {}", e);
    }

}