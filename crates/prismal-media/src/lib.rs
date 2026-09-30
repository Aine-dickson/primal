//! Prismal media export: a presentation as a video file or a raster image (PK-12.2, D-052).
//!
//! Frames are the SVG renderer's (`prismal-svg`), drawn from what the host interface gives
//! every host: the layout and a frame description per instant. A lesson is opened in the
//! `video` medium, so its explore beats play their fallbacks (PK-9.10) and elements without
//! one are reported (PK-12.3); a session is recorded from the start of its run. Frames are
//! deterministic (PK-8.7), so the same program exports the same video.
//!
//! Each frame is rasterized with resvg onto one canvas, the largest frame of the clip, so
//! that frames of different sizes (a caption appears, a panel grows) keep their views in
//! place. Encoding is left to an external encoder, ffmpeg, fed raw frames through a pipe:
//! video codecs stay outside Prismal. Without an encoder, the frames are written as a PNG
//! sequence with the captions and the command that encodes them.
//!
//! Narration has no audio yet (the language carries its text only), so a video has no
//! sound; captions are drawn into the frames, carried as a subtitle track, or both, and
//! always written beside the video as WebVTT (PK-11.3).

use prismal_host::Instance;
use prismal_svg::fmt;
use resvg::{tiny_skia, usvg};
use serde_json::Value as Json;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// ---------------------------------------------------------------- settings

/// Where narration captions go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captions {
    /// Drawn into the frames, as the frame description shows them.
    Burned,
    /// Only in a subtitle track, which a viewer can turn off.
    Track,
    /// Both.
    Both,
}

impl Captions {
    fn burned(self) -> bool {
        self != Captions::Track
    }
    fn track(self) -> bool {
        self != Captions::Burned
    }
}

/// How a presentation is exported.
#[derive(Clone, Debug)]
pub struct Settings {
    /// Frames per second.
    pub fps: f64,
    /// Pixels per SVG pixel.
    pub scale: f64,
    /// Where captions go.
    pub captions: Captions,
    /// The last instant exported, instead of the end of the lesson or the session.
    pub until: Option<f64>,
    /// How frames are drawn. The header line is off by default: it names the instant, which
    /// a video shows by playing.
    pub svg: prismal_svg::Options,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings { fps: 30.0, scale: 1.0, captions: Captions::Burned, until: None, svg: prismal_svg::Options { header: false, ..Default::default() } }
    }
}

// ---------------------------------------------------------------- clips

/// A caption cue, in presentation seconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Cue {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

/// A presentation drawn at every frame instant.
#[derive(Clone, Debug)]
pub struct Clip {
    pub presentation: String,
    pub fps: f64,
    /// One SVG document per frame, the `k`th at `k / fps` seconds.
    pub frames: Vec<String>,
    /// Narration captions (empty for a session).
    pub captions: Vec<Cue>,
    /// What the video medium could not show, and refusals and diagnostics of the lesson
    /// (PK-12.3): reported, never silently dropped.
    pub reports: Vec<String>,
    /// Background color of the theme, which fills the canvas around smaller frames.
    pub background: String,
}

/// The frame instants of a clip that ends at `end`: `0, 1/fps, ...` up to and including the
/// last instant not after `end`, so the final state is shown.
pub fn times(end: f64, fps: f64) -> Vec<f64> {
    let n = (end * fps + 1e-9).floor().max(0.0) as usize;
    (0..=n).map(|k| k as f64 / fps).collect()
}

/// Opens `presentation` for linear media and draws its frames.
pub fn clip(inst: &mut Instance, presentation: &str, s: &Settings) -> Result<Clip, String> {
    let layout = inst.open(presentation, true).map_err(|d| d.to_string())?;
    let lesson = layout["mode"] == "lesson";
    let end = s.until.unwrap_or_else(|| if lesson { &layout["lesson"]["end"] } else { &layout["session"]["end"] }.as_f64().unwrap_or(0.0));
    let frames = times(end, s.fps).into_iter().map(|t| draw(inst, &layout, t, 1.0 / s.fps, s)).collect();
    let l = &layout["lesson"];
    let list = |v: &Json| v.as_array().cloned().unwrap_or_default();
    let captions = list(&l["captions"])
        .iter()
        .map(|c| Cue { start: c["start"].as_f64().unwrap_or(0.0), end: c["end"].as_f64().unwrap_or(0.0), text: c["text"].as_str().unwrap_or("").to_string() })
        .filter(|c| c.start <= end)
        .collect();
    let mut reports: Vec<String> = list(&l["unsupported"]).iter().chain(&list(&l["diagnostics"])).map(|r| r.as_str().map(str::to_string).unwrap_or_else(|| r.to_string())).collect();
    reports.extend(list(&l["refusals"]).iter().map(|r| format!("refused at {} s: {} ({})", r["at"], r["input"].as_str().unwrap_or(""), r["reason"].as_str().unwrap_or(""))));
    Ok(Clip { presentation: layout["presentation"].as_str().unwrap_or(presentation).to_string(), fps: s.fps, frames, captions, reports, background: s.svg.theme.bg.to_string() })
}

/// The SVG document of the frame at instant `t` of an open presentation whose layout is
/// `layout`: presentation time in a lesson, simulation time in a session.
pub fn draw(inst: &mut Instance, layout: &Json, t: f64, dt: f64, s: &Settings) -> String {
    let mut frame = if layout["mode"] == "lesson" {
        inst.frame(t, dt)
    } else {
        inst.seek(t);
        inst.frame(0.0, dt)
    };
    if !s.captions.burned() {
        frame["captions"] = Json::Array(vec![]);
    }
    prismal_svg::render(layout, &frame, &s.svg)
}

/// The frame at instant `t` of `presentation` in linear media, as a PNG image.
pub fn still(inst: &mut Instance, raster: &Raster, presentation: &str, t: f64, s: &Settings) -> Result<Vec<u8>, String> {
    let layout = inst.open(presentation, true).map_err(|d| d.to_string())?;
    png(raster, &draw(inst, &layout, t, 0.0, s), s.scale, s.svg.theme.bg)
}

// ---------------------------------------------------------------- captions

/// Captions as a WebVTT document.
pub fn webvtt(cues: &[Cue]) -> String {
    let mut out = String::from("WEBVTT\n");
    for (k, c) in cues.iter().enumerate() {
        out.push_str(&format!("\n{}\n{} --> {}\n{}\n", k + 1, stamp(c.start), stamp(c.end), c.text.replace("-->", "->")));
    }
    out
}

/// `hh:mm:ss.mmm`.
fn stamp(t: f64) -> String {
    let ms = (t.max(0.0) * 1000.0).round() as u64;
    format!("{:02}:{:02}:{:02}.{:03}", ms / 3_600_000, ms / 60_000 % 60, ms / 1000 % 60, ms % 1000)
}

// ---------------------------------------------------------------- rasterizing

/// Draws SVG documents as pixels, with the fonts of the system and of any directories given.
pub struct Raster {
    opts: usvg::Options<'static>,
}

impl Raster {
    pub fn new(font_dirs: &[PathBuf]) -> Raster {
        let mut opts = usvg::Options::default();
        let db = opts.fontdb_mut();
        db.load_system_fonts();
        for d in font_dirs {
            db.load_fonts_dir(d);
        }
        // Generic families resolve to the first installed candidate, so text is drawn on
        // any system that has one of them.
        let has = |db: &usvg::fontdb::Database, name: &str| db.faces().any(|f| f.families.iter().any(|(n, _)| n == name));
        let pick = |db: &usvg::fontdb::Database, names: &[&str]| names.iter().find(|n| has(db, n)).map(|n| n.to_string());
        if let Some(n) = pick(db, &["Segoe UI", "Arial", "Helvetica", "DejaVu Sans", "Liberation Sans", "Noto Sans"]) {
            db.set_sans_serif_family(n);
        }
        if let Some(n) = pick(db, &["Times New Roman", "STIX Two Text", "Liberation Serif", "DejaVu Serif", "Noto Serif"]) {
            db.set_serif_family(n);
        }
        if let Some(n) = pick(db, &["Cascadia Mono", "Consolas", "DejaVu Sans Mono", "Liberation Mono", "Courier New"]) {
            db.set_monospace_family(n);
        }
        Raster { opts }
    }

    fn tree(&self, svg: &str) -> Result<usvg::Tree, String> {
        usvg::Tree::from_str(svg, &self.opts).map_err(|e| e.to_string())
    }

    /// Width and height of a document, in SVG pixels.
    pub fn size(&self, svg: &str) -> Result<(f64, f64), String> {
        // The renderer writes both on the root element; other documents are parsed.
        let root = svg.split_once("<svg").and_then(|(_, r)| r.split_once('>')).map(|(r, _)| r).unwrap_or("");
        let attr = |name: &str| root.split_once(&format!(" {name}=\"")).and_then(|(_, r)| r.split_once('"')).and_then(|(v, _)| v.parse::<f64>().ok());
        if let (Some(w), Some(h)) = (attr("width"), attr("height")) {
            return Ok((w, h));
        }
        let s = self.tree(svg)?.size();
        Ok((s.width() as f64, s.height() as f64))
    }

    /// Draws `svg` at `scale` onto a `w` by `h` canvas filled with `background`, from its
    /// top left corner. Returns opaque RGBA pixels, row by row.
    pub fn draw(&self, svg: &str, scale: f64, (w, h): (u32, u32), background: &str) -> Result<tiny_skia::Pixmap, String> {
        let tree = self.tree(svg)?;
        let mut pm = tiny_skia::Pixmap::new(w, h).ok_or("empty canvas")?;
        pm.fill(color(background));
        resvg::render(&tree, tiny_skia::Transform::from_scale(scale as f32, scale as f32), &mut pm.as_mut());
        Ok(pm)
    }
}

fn color(hex: &str) -> tiny_skia::Color {
    let v = u32::from_str_radix(hex.trim_start_matches('#'), 16).unwrap_or(0xffffff);
    tiny_skia::Color::from_rgba8((v >> 16) as u8, (v >> 8) as u8, v as u8, 255)
}

/// The canvas of a clip: its largest width and height at `scale`, rounded up to even numbers
/// of pixels, which encoders of subsampled color need.
pub fn canvas(sizes: impl IntoIterator<Item = (f64, f64)>, scale: f64) -> (u32, u32) {
    let (w, h) = sizes.into_iter().fold((0.0f64, 0.0f64), |(a, b), (w, h)| (a.max(w), b.max(h)));
    let even = |x: f64| {
        let p = (x * scale).ceil().max(2.0) as u32;
        p + p % 2
    };
    (even(w), even(h))
}

fn clip_canvas(raster: &Raster, clip: &Clip, scale: f64) -> Result<(u32, u32), String> {
    let sizes = clip.frames.iter().map(|f| raster.size(f)).collect::<Result<Vec<_>, _>>()?;
    Ok(canvas(sizes, scale))
}

/// One frame as a PNG image.
pub fn png(raster: &Raster, svg: &str, scale: f64, background: &str) -> Result<Vec<u8>, String> {
    let size = canvas([raster.size(svg)?], scale);
    raster.draw(svg, scale, size, background)?.encode_png().map_err(|e| e.to_string())
}

// ---------------------------------------------------------------- encoding

/// Video containers, chosen by the extension of the output file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Container {
    Mp4,
    Mov,
    Mkv,
    Webm,
    Gif,
}

impl Container {
    pub fn of(path: &Path) -> Option<Container> {
        match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
            "mp4" | "m4v" => Some(Container::Mp4),
            "mov" => Some(Container::Mov),
            "mkv" => Some(Container::Mkv),
            "webm" => Some(Container::Webm),
            "gif" => Some(Container::Gif),
            _ => None,
        }
    }

    /// Whether the container carries a subtitle track.
    pub fn subtitles(self) -> bool {
        self != Container::Gif
    }
}

/// The encoder's arguments: raw RGBA frames of `size` on standard input at `fps`, and a
/// WebVTT subtitle file when given, written to `out`.
pub fn encoder_args(container: Container, out: &Path, (w, h): (u32, u32), fps: f64, subtitles: Option<&Path>, title: &str) -> Vec<String> {
    let mut a: Vec<String> = ["-hide_banner", "-loglevel", "error", "-y", "-f", "rawvideo", "-pix_fmt", "rgba", "-s"].map(String::from).to_vec();
    a.push(format!("{w}x{h}"));
    a.extend(["-framerate".into(), fmt(fps), "-i".into(), "-".into()]);
    let subs = subtitles.filter(|_| container.subtitles());
    if let Some(s) = subs {
        a.extend(["-i".into(), s.display().to_string(), "-map".into(), "0:v".into(), "-map".into(), "1:s".into()]);
    }
    let video: &[&str] = match container {
        Container::Mp4 | Container::Mov | Container::Mkv => &["-c:v", "libx264", "-preset", "medium", "-crf", "18", "-pix_fmt", "yuv420p"],
        Container::Webm => &["-c:v", "libvpx-vp9", "-crf", "32", "-b:v", "0", "-pix_fmt", "yuv420p"],
        Container::Gif => &["-filter_complex", "split[a][b];[a]palettegen[p];[b][p]paletteuse"],
    };
    a.extend(video.iter().map(|s| s.to_string()));
    if matches!(container, Container::Mp4 | Container::Mov) {
        a.extend(["-movflags".into(), "+faststart".into()]);
    }
    if subs.is_some() {
        let codec = match container {
            Container::Mp4 | Container::Mov => "mov_text",
            Container::Webm => "webvtt",
            _ => "srt",
        };
        a.extend(["-c:s".into(), codec.into(), "-metadata:s:s:0".into(), "title=Captions".into()]);
    }
    a.extend(["-metadata".into(), format!("title={title}"), out.display().to_string()]);
    a
}

/// The encoder: `PRISMAL_FFMPEG` if set, otherwise `ffmpeg` on the path.
pub fn default_encoder() -> PathBuf {
    std::env::var_os("PRISMAL_FFMPEG").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("ffmpeg"))
}

/// The WebVTT file written beside `out`: `out` with the extension `vtt`.
pub fn sidecar(out: &Path) -> PathBuf {
    out.with_extension("vtt")
}

/// Encodes `clip` as the video file `out` with `encoder`. Captions, when the clip has any,
/// are written beside it as WebVTT, and carried as a subtitle track when the settings ask
/// for one and the container has tracks.
pub fn encode(raster: &Raster, clip: &Clip, s: &Settings, encoder: &Path, out: &Path) -> Result<(), String> {
    let container = Container::of(out).ok_or_else(|| format!("{}: unknown video format (mp4, mov, mkv, webm or gif)", out.display()))?;
    let size = clip_canvas(raster, clip, s.scale)?;
    let vtt = sidecar(out);
    if !clip.captions.is_empty() {
        std::fs::write(&vtt, webvtt(&clip.captions)).map_err(|e| format!("{}: {e}", vtt.display()))?;
    }
    let subs = (s.captions.track() && !clip.captions.is_empty()).then_some(vtt.as_path());
    let args = encoder_args(container, out, size, clip.fps, subs, &clip.presentation);
    let mut child = Command::new(encoder)
        .args(&args)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot start the encoder `{}` ({e}); install ffmpeg, set PRISMAL_FFMPEG, or export to a directory to write the frames", encoder.display()))?;
    let mut stdin = child.stdin.take().expect("piped");
    for f in &clip.frames {
        let pm = raster.draw(f, s.scale, size, &clip.background)?;
        if stdin.write_all(pm.data()).is_err() {
            break; // the encoder stopped; its status says why
        }
    }
    drop(stdin);
    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("the encoder failed ({status})"))
    }
}

/// Writes `clip` to the directory `dir` as `frame-00000.png ...`, `captions.vtt` when it has
/// captions, and `encode.txt`, the encoder command that makes a video of them.
pub fn write_frames(raster: &Raster, clip: &Clip, s: &Settings, dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let size = clip_canvas(raster, clip, s.scale)?;
    for (k, f) in clip.frames.iter().enumerate() {
        let path = dir.join(format!("frame-{k:05}.png"));
        let bytes = raster.draw(f, s.scale, size, &clip.background)?.encode_png().map_err(|e| e.to_string())?;
        std::fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    let mut cmd = format!("ffmpeg -framerate {} -i frame-%05d.png", fmt(clip.fps));
    if !clip.captions.is_empty() {
        std::fs::write(dir.join("captions.vtt"), webvtt(&clip.captions)).map_err(|e| e.to_string())?;
        if s.captions.track() {
            cmd.push_str(" -i captions.vtt -c:s mov_text");
        }
    }
    cmd.push_str(" -c:v libx264 -pix_fmt yuv420p -crf 18 video.mp4\n");
    std::fs::write(dir.join("encode.txt"), cmd).map_err(|e| e.to_string())
}
