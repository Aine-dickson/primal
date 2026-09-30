//! Narration sound for video export (D-053).
//!
//! Sound is not part of a program. Every host that shows captions may voice them: the
//! engine names each caption cue after the beat that narrates it (`b3`, `b3.2`), and a
//! voice is a directory of recordings named by cue (`b3.wav`, `b3.mp3` ...), or a speech
//! synthesizer for cues without one. The timeline decides when a cue starts and how long it
//! lasts; a recording longer than its cue is reported, never allowed to change the timing.

use crate::Cue;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Extensions of recordings, in the order they are looked for.
pub const AUDIO_EXTENSIONS: [&str; 8] = ["wav", "mp3", "ogg", "opus", "m4a", "flac", "aac", "aiff"];

/// A speech synthesizer for cues without a recording.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Speech {
    Off,
    /// The system's synthesizer: Windows speech, `say` on macOS, `espeak-ng` elsewhere.
    System,
    /// A program and its arguments; `{out}` in an argument is replaced by the file to write
    /// (WAV), and the text is given on standard input.
    Command(Vec<String>),
}

/// The sound of an export.
#[derive(Clone, Debug)]
pub struct Sound {
    /// A directory of recordings named by cue.
    pub voice: Option<PathBuf>,
    /// The synthesizer for cues without a recording.
    pub speech: Speech,
    /// A music file, looped under the narration for the whole video.
    pub music: Option<PathBuf>,
    /// Volume of the music, 1 as recorded.
    pub music_volume: f64,
}

impl Default for Sound {
    fn default() -> Sound {
        Sound { voice: None, speech: Speech::Off, music: None, music_volume: 0.25 }
    }
}

impl Sound {
    pub fn is_silent(&self) -> bool {
        self.voice.is_none() && self.speech == Speech::Off && self.music.is_none()
    }
}

/// A narration file placed at a presentation instant.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub cue: String,
    pub file: PathBuf,
    pub start: f64,
}

/// The recording of `cue` in `dir`, if there is one.
pub fn recording(dir: &Path, cue: &str) -> Option<PathBuf> {
    AUDIO_EXTENSIONS.iter().map(|e| dir.join(format!("{cue}.{e}"))).find(|p| p.is_file())
}

/// Speaks `text` into a file named `out` without its extension, and returns the file written.
pub fn synthesize(speech: &Speech, text: &str, out: &Path) -> Result<PathBuf, String> {
    let wav = out.with_extension("wav");
    let (mut cmd, file) = match speech {
        Speech::Off => return Err("no synthesizer".into()),
        Speech::Command(argv) => {
            let (prog, args) = argv.split_first().ok_or("an empty synthesizer command")?;
            let mut c = Command::new(prog);
            c.args(args.iter().map(|a| a.replace("{out}", &wav.display().to_string())));
            (c, wav)
        }
        Speech::System if cfg!(windows) => {
            let mut c = Command::new("powershell");
            c.args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "[Console]::InputEncoding = [Text.Encoding]::UTF8; Add-Type -AssemblyName System.Speech; \
                 $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; $s.SetOutputToWaveFile($env:PRISMAL_SPEECH_OUT); \
                 $s.Speak([Console]::In.ReadToEnd()); $s.Dispose()",
            ]);
            c.env("PRISMAL_SPEECH_OUT", &wav);
            (c, wav)
        }
        Speech::System if cfg!(target_os = "macos") => {
            let aiff = out.with_extension("aiff");
            let mut c = Command::new("say");
            c.arg("-o").arg(&aiff);
            (c, aiff)
        }
        Speech::System => {
            let mut c = Command::new("espeak-ng");
            c.arg("--stdin").arg("-w").arg(&wav);
            (c, wav)
        }
    };
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::null()).spawn().map_err(|e| format!("cannot start the speech synthesizer ({e})"))?;
    child.stdin.take().expect("piped").write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() && file.is_file() {
        Ok(file)
    } else {
        Err(format!("the speech synthesizer failed ({status})"))
    }
}

/// Length of an audio file in seconds, read with ffprobe (`prober`).
pub fn duration(prober: &Path, file: &Path) -> Option<f64> {
    let o = Command::new(prober).args(["-v", "error", "-show_entries", "format=duration", "-of", "default=nw=1:nk=1"]).arg(file).output().ok()?;
    String::from_utf8_lossy(&o.stdout).trim().parse().ok()
}

/// The prober beside an encoder: `ffprobe` in the directory of `ffmpeg`.
pub fn prober(encoder: &Path) -> PathBuf {
    let name = encoder.file_name().map(|n| n.to_string_lossy().replace("ffmpeg", "ffprobe")).unwrap_or_else(|| "ffprobe".into());
    encoder.with_file_name(name)
}

/// The narration of `cues`: each cue's recording, or its synthesized speech, placed at its
/// start. Returns the tracks and reports: cues without a voice, and voices longer than their
/// cue. Synthesized speech is written to `work`.
pub fn narration(cues: &[Cue], sound: &Sound, work: &Path, prober: Option<&Path>) -> (Vec<Track>, Vec<String>) {
    let mut tracks = vec![];
    let mut reports = vec![];
    for c in cues {
        let file = match sound.voice.as_deref().and_then(|d| recording(d, &c.name)) {
            Some(f) => Some(f),
            None if sound.speech != Speech::Off => match synthesize(&sound.speech, &c.text, &work.join(&c.name)) {
                Ok(f) => Some(f),
                Err(e) => {
                    reports.push(format!("cue {}: {e}", c.name));
                    None
                }
            },
            None => {
                if sound.voice.is_some() {
                    reports.push(format!("cue {}: no recording ({}.wav, .mp3 ...); the cue is silent", c.name, c.name));
                }
                None
            }
        };
        let Some(file) = file else { continue };
        if let Some(len) = prober.and_then(|p| duration(p, &file)) {
            if len > c.end - c.start + 0.05 {
                reports.push(format!(
                    "cue {}: its voice lasts {:.2} s, its caption {} s; give the narration `for {:.1} s` or more, or it overlaps what follows",
                    c.name,
                    len,
                    prismal_svg::fmt(c.end - c.start),
                    (len * 10.0).ceil() / 10.0
                ));
            }
        }
        tracks.push(Track { cue: c.name.clone(), file, start: c.start });
    }
    (tracks, reports)
}

/// A recording script: every cue with its name, start, length and text.
pub fn script(presentation: &str, cues: &[Cue]) -> String {
    let mut out = format!(
        "# Narration of {presentation}\n# Record each cue into one directory as CUE.wav (or .mp3, .ogg, .m4a, .flac),\n# no longer than its length; lengthen a narration with `narrate \"...\" for d`.\n\n"
    );
    let w = cues.iter().map(|c| c.name.len()).max().unwrap_or(0);
    for c in cues {
        let m = (c.start / 60.0).floor();
        out.push_str(&format!("{:w$}  {:>2}:{:06.3}  {:>5} s  {}\n", c.name, m as u64, c.start - 60.0 * m, prismal_svg::fmt(c.end - c.start), c.text));
    }
    out
}
