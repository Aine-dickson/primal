//! Exports a presentation as a video file, a PNG image sequence, or a PNG still (D-052).
//!
//! `prismal-media PROGRAM PRESENTATION OUT [--fps N] [--scale S] [--at T] [--until T]
//! [--captions burned|track|both] [--voice DIR] [--speech system|COMMAND] [--music FILE]
//! [--music-volume V] [--dark] [--header] [--fonts DIR]... [--encoder PATH]`
//!
//! PROGRAM is a source file (working syntax, or a Markdown document whose `text` and `cases`
//! blocks form the program) or the key of an embedded example (`rp01` to `rp08`,
//! `g8-freefall` ...). OUT decides what is written:
//!
//! - `name.mp4`, `.mov`, `.mkv`, `.webm` or `.gif`: a video, encoded by ffmpeg (`--encoder`,
//!   or `PRISMAL_FFMPEG`, or `ffmpeg` on the path), with the captions beside it as
//!   `name.vtt` and the events it shows as `name.descriptions.vtt`;
//! - `name.png`: the frame at `--at T` (default 0);
//! - `name.txt`: the recording script, every narration cue with its name, start, length and
//!   text;
//! - anything else: a directory of `frame-00000.png ...`, `captions.vtt`,
//!   `descriptions.vtt` and `encode.txt`,
//!   for encoding elsewhere.
//!
//! A lesson plays in the video medium: explore beats play their fallbacks (PK-9.10), and
//! what cannot be shown is reported on standard error (PK-12.3). A session is recorded from
//! the start of its run to its end, or to `--until`.
//!
//! Sound (D-053): `--voice DIR` voices each cue with the recording named after it
//! (`b3.wav`); `--speech system` synthesizes cues without one with the system's voice, and
//! `--speech "COMMAND ARGS {out}"` with any program that reads text on standard input and
//! writes a WAV file to `{out}`; `--music FILE` loops music under the narration.

use prismal_host::{Content, Document, Instance};
use prismal_media::{clip, encode, still, voice, write_frames, Captions, Container, Raster, Settings, Speech};
use std::path::PathBuf;

const USAGE: &str = "usage: prismal-media PROGRAM PRESENTATION OUT [--fps N] [--scale S] [--at T] [--until T] [--captions burned|track|both] [--voice DIR] [--speech system|COMMAND] [--music FILE] [--music-volume V] [--dark] [--header] [--fonts DIR]... [--encoder PATH]";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let valued = ["--fps", "--scale", "--at", "--until", "--captions", "--fonts", "--encoder", "--voice", "--speech", "--music", "--music-volume"];
    let mut pos = vec![];
    let mut i = 0;
    while i < args.len() {
        if valued.contains(&args[i].as_str()) {
            i += 2;
            continue;
        }
        if args[i].starts_with("--") && !matches!(args[i].as_str(), "--dark" | "--header") {
            fail(&format!("unknown option {}\n{USAGE}", args[i]));
        }
        if !args[i].starts_with("--") {
            pos.push(args[i].clone());
        }
        i += 1;
    }
    if pos.len() != 3 {
        fail(USAGE);
    }
    let value = |flag: &str| args.iter().position(|a| a == flag).map(|i| args.get(i + 1).unwrap_or_else(|| fail(&format!("{flag} takes a value"))));
    let number = |flag: &str| value(flag).map(|v| v.parse::<f64>().unwrap_or_else(|_| fail(&format!("{flag} takes a number"))));

    let mut s = Settings::default();
    s.fps = number("--fps").unwrap_or(s.fps);
    s.scale = number("--scale").unwrap_or(s.scale);
    s.until = number("--until");
    if !(s.fps > 0.0 && s.scale > 0.0) {
        fail("--fps and --scale must be positive");
    }
    s.captions = match value("--captions").map(String::as_str) {
        None | Some("burned") => Captions::Burned,
        Some("track") => Captions::Track,
        Some("both") => Captions::Both,
        Some(o) => fail(&format!("--captions takes burned, track or both, not {o}")),
    };
    if args.iter().any(|a| a == "--dark") {
        s.svg.theme = prismal_svg::Theme::DARK;
    }
    s.svg.header = args.iter().any(|a| a == "--header");
    s.sound.voice = value("--voice").map(PathBuf::from);
    s.sound.speech = match value("--speech").map(String::as_str) {
        None => Speech::Off,
        Some("system") => Speech::System,
        Some(cmd) => Speech::Command(cmd.split_whitespace().map(String::from).collect()),
    };
    s.sound.music = value("--music").map(PathBuf::from);
    s.sound.music_volume = number("--music-volume").unwrap_or(s.sound.music_volume);
    if let Some(d) = &s.sound.voice {
        if !d.is_dir() {
            fail(&format!("--voice {}: not a directory", d.display()));
        }
    }
    let fonts: Vec<PathBuf> = args.windows(2).filter(|w| w[0] == "--fonts").map(|w| PathBuf::from(&w[1])).collect();
    let encoder = value("--encoder").map(PathBuf::from).unwrap_or_else(prismal_media::default_encoder);

    let src = match std::fs::read_to_string(&pos[0]) {
        Ok(s) => s,
        Err(_) => match prismal_web::examples::all().into_iter().find(|e| e.key == pos[0]) {
            Some(e) => e.source,
            None => fail(&format!("{} is neither a file nor an example", pos[0])),
        },
    };
    let doc = Document::load(Content::Text(src)).unwrap_or_else(|ds| fail(&format!("{ds:?}")));
    let prog = doc.program().unwrap_or_else(|| fail(&format!("{:?}", doc.diagnostics())));
    let mut inst = Instance::new(prog);
    let raster = Raster::new(&fonts);
    let out = PathBuf::from(&pos[2]);

    if out.extension().is_some_and(|e| e.eq_ignore_ascii_case("png")) {
        let bytes = still(&mut inst, &raster, &pos[1], number("--at").unwrap_or(0.0), &s).unwrap_or_else(|e| fail(&e));
        std::fs::write(&out, bytes).unwrap_or_else(|e| fail(&format!("{}: {e}", out.display())));
        println!("{}", out.display());
        return;
    }
    if out.extension().is_some_and(|e| e.eq_ignore_ascii_case("txt")) {
        let c = clip(&mut inst, &pos[1], &Settings { fps: 1.0, ..s.clone() }).unwrap_or_else(|e| fail(&e));
        std::fs::write(&out, voice::script(&c.presentation, &c.captions)).unwrap_or_else(|e| fail(&format!("{}: {e}", out.display())));
        println!("{}: {} cues", out.display(), c.captions.len());
        return;
    }
    let c = clip(&mut inst, &pos[1], &s).unwrap_or_else(|e| fail(&e));
    for r in &c.reports {
        eprintln!("note: {r}");
    }
    let container = Container::of(&out);
    if container == Some(Container::Gif) && s.captions == Captions::Track {
        eprintln!("note: a GIF has no subtitle track; captions are only in {}", prismal_media::sidecar(&out).display());
    }
    let notes = match container {
        Some(_) => encode(&raster, &c, &s, &encoder, &out),
        None => write_frames(&raster, &c, &s, &out).map(|_| vec![]),
    }
    .unwrap_or_else(|e| fail(&e));
    for n in notes {
        eprintln!("note: {n}");
    }
    println!("{}: {} frames at {} per second, {:.2} s", out.display(), c.frames.len(), prismal_svg::fmt(c.fps), c.frames.len() as f64 / c.fps);
}

fn fail(m: &str) -> ! {
    eprintln!("{m}");
    std::process::exit(1)
}
