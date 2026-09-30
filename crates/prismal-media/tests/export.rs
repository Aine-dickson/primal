//! Media export (D-052): frame instants, captions, canvases, the encoder's arguments, and
//! whole exports of the reference programs' lessons and sessions. The encoder itself is run
//! when one is found (`PRISMAL_FFMPEG` or `ffmpeg` on the path); otherwise that test says so
//! and passes.

use prismal_host::{Content, Document, Instance};
use prismal_media::*;
use std::path::{Path, PathBuf};

fn instance(src: String) -> Instance {
    let doc = Document::load(Content::Text(src)).expect("compiles");
    Instance::new(doc.program().expect("checks"))
}

fn example(key: &str) -> Instance {
    instance(prismal_web::examples::all().into_iter().find(|e| e.key == key).unwrap_or_else(|| panic!("no example {key}")).source)
}

fn settings(fps: f64, captions: Captions) -> Settings {
    Settings { fps, captions, ..Default::default() }
}

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("media").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn frame_instants_include_the_end() {
    assert_eq!(times(1.0, 4.0), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    // 0.3 * 10 is 2.9999999999999996 in floating point: the frame at 0.3 s is kept.
    assert_eq!(times(0.3, 10.0).len(), 4);
    assert_eq!(times(0.0, 30.0), vec![0.0]);
}

#[test]
fn captions_are_webvtt() {
    let cues = [Cue { start: 0.0, end: 4.0, text: "A ball --> falls.".into() }, Cue { start: 3661.5, end: 3663.25, text: "Later.".into() }];
    assert_eq!(webvtt(&cues), "WEBVTT\n\n1\n00:00:00.000 --> 00:00:04.000\nA ball -> falls.\n\n2\n01:01:01.500 --> 01:01:03.250\nLater.\n");
}

#[test]
fn canvas_is_the_largest_frame_in_even_pixels() {
    assert_eq!(canvas([(100.2, 50.0), (99.0, 51.5)], 1.0), (102, 52));
    assert_eq!(canvas([(100.0, 50.0)], 2.0), (200, 100));
    assert_eq!(canvas([(100.5, 50.0)], 1.0), (102, 50));
}

#[test]
fn encoder_arguments() {
    let a = encoder_args(Container::Mp4, Path::new("out.mp4"), (640, 360), 30.0, Some(Path::new("out.vtt")), "Lesson");
    let joined = a.join(" ");
    assert!(joined.starts_with("-hide_banner -loglevel error -y -f rawvideo -pix_fmt rgba -s 640x360 -framerate 30 -i - -i out.vtt -map 0:v -map 1:s"), "{joined}");
    assert!(joined.contains("-c:v libx264") && joined.contains("-pix_fmt yuv420p") && joined.contains("-c:s mov_text"), "{joined}");
    assert!(joined.ends_with("-metadata title=Lesson out.mp4"), "{joined}");

    let w = encoder_args(Container::Webm, Path::new("o.webm"), (2, 2), 24.0, Some(Path::new("o.vtt")), "t").join(" ");
    assert!(w.contains("libvpx-vp9") && w.contains("-c:s webvtt"), "{w}");

    // A GIF has no subtitle track: captions are only drawn and written beside it.
    let g = encoder_args(Container::Gif, Path::new("o.gif"), (2, 2), 10.0, Some(Path::new("o.vtt")), "t").join(" ");
    assert!(!g.contains("o.vtt") && g.contains("palettegen"), "{g}");

    assert_eq!(Container::of(Path::new("a/b.MP4")), Some(Container::Mp4));
    assert_eq!(Container::of(Path::new("frames")), None);
    assert_eq!(sidecar(Path::new("a/b.mp4")), PathBuf::from("a/b.vtt"));
}

/// RP-08 in the video medium: its explore beat plays the fallback, so nothing is reported;
/// captions come from the narration; every frame is drawn on one canvas.
#[test]
fn lesson_clip() {
    let mut inst = example("rp08");
    let fps = 2.0;
    let burned = clip(&mut inst, "ProjectileLesson", &settings(fps, Captions::Burned)).unwrap();
    let layout = inst.layout();
    assert_eq!(layout["lesson"]["medium"], "video");
    let end = layout["lesson"]["end"].as_f64().unwrap();
    assert_eq!(burned.frames.len(), times(end, fps).len());
    assert!(burned.reports.is_empty(), "{:?}", burned.reports);
    assert_eq!(burned.captions[0], Cue { start: 0.0, end: 4.0, text: "A ball is launched at 45 degrees.".into() });
    assert!(burned.frames[2].contains("A ball is launched at 45 degrees."));

    // With captions in a track only, no frame draws them.
    let track = clip(&mut example("rp08"), "ProjectileLesson", &settings(fps, Captions::Track)).unwrap();
    assert_eq!(track.captions, burned.captions);
    assert!(track.frames.iter().all(|f| !f.contains("A ball is launched")));

    // Frames are deterministic (PK-8.7): a second export is identical.
    let again = clip(&mut example("rp08"), "ProjectileLesson", &settings(fps, Captions::Burned)).unwrap();
    assert_eq!(again.frames, burned.frames);

    // Frames differ in size (a caption adds a line); all are drawn on the largest.
    let raster = Raster::new(&[]);
    let sizes: Vec<(f64, f64)> = burned.frames.iter().map(|f| raster.size(f).unwrap()).collect();
    assert!(sizes.iter().any(|s| s != &sizes[0]), "frames of one size: the canvas is not exercised");
    let (w, h) = canvas(sizes.iter().copied(), 1.0);
    assert_eq!((w % 2, h % 2), (0, 0));
    for k in [0, burned.frames.len() / 2, burned.frames.len() - 1] {
        let pm = raster.draw(&burned.frames[k], 1.0, (w, h), &burned.background).unwrap();
        assert_eq!((pm.width(), pm.height()), (w, h));
        // Opaque everywhere, so raw frames need no alpha.
        assert!(pm.data().chunks(4).all(|p| p[3] == 255));
        // Something is drawn besides the background.
        assert!(pm.data().chunks(4).any(|p| p[..3] != [0x1d, 0x1f, 0x24] && p[..3] != [0xf6, 0xf6, 0xf3]));
    }
}

/// A lesson whose explore beat has no fallback cannot be shown in linear media: the export
/// reports it (PK-12.3) instead of dropping it silently.
#[test]
fn missing_fallback_is_reported() {
    let src = r#"
space Plane = euclidean(2)
model Fall in Plane {
  param { h: Length = 10 m in [1 m, 50 m] }
  state { pos: Point = origin + (0 m, h) }
  flow { der(pos) = (0 m/s, -1 m/s) }
}
presentation Choose for Fall {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) { marker(pos) as ball }
  timeline {
    scene only {
      beat watch  { run rate 1; wait 1 s }
      beat choose { hold; explore limit 5 s { slider(h, range: [1 m, 50 m]) } }
    }
  }
}
"#;
    let c = clip(&mut instance(src.into()), "Choose", &settings(4.0, Captions::Burned)).unwrap();
    assert!(c.reports.iter().any(|r| r.contains("choose") && r.contains("PK-12.3")), "{:?}", c.reports);
}

/// A session is recorded from the start of its run: frames follow simulation time.
#[test]
fn session_clip() {
    let mut inst = example("rp04");
    let name = inst.program().doc.presentations[0].name.clone();
    let s = Settings { until: Some(1.0), ..settings(4.0, Captions::Burned) };
    let c = clip(&mut inst, &name, &s).unwrap();
    assert_eq!(c.frames.len(), 5);
    assert!(c.captions.is_empty());
    assert_ne!(c.frames[0], c.frames[4], "the pendulum does not move");
}

#[test]
fn still_and_frame_directory() {
    let raster = Raster::new(&[]);
    let png = still(&mut example("rp08"), &raster, "ProjectileLesson", 5.0, &Settings { scale: 2.0, ..Default::default() }).unwrap();
    assert_eq!(&png[1..4], b"PNG");

    let dir = scratch("frames");
    let s = Settings { until: Some(2.0), ..settings(2.0, Captions::Track) };
    let c = clip(&mut example("rp08"), "ProjectileLesson", &s).unwrap();
    write_frames(&raster, &c, &s, &dir).unwrap();
    assert!(dir.join("frame-00004.png").exists() && !dir.join("frame-00005.png").exists());
    assert!(std::fs::read_to_string(dir.join("captions.vtt")).unwrap().contains("A ball is launched"));
    assert!(std::fs::read_to_string(dir.join("encode.txt")).unwrap().contains("-i captions.vtt"));
}

#[test]
fn a_missing_encoder_is_named() {
    let c = clip(&mut example("rp08"), "ProjectileLesson", &Settings { until: Some(0.5), ..settings(2.0, Captions::Burned) }).unwrap();
    let out = scratch("missing").join("x.mp4");
    let e = encode(&Raster::new(&[]), &c, &Settings::default(), Path::new("no-such-encoder-prismal"), &out).unwrap_err();
    assert!(e.contains("cannot start the encoder `no-such-encoder-prismal`"), "{e}");
}

/// The whole path through the encoder, when one is installed.
#[test]
fn encodes_a_video() {
    let encoder = default_encoder();
    if std::process::Command::new(&encoder).arg("-version").output().map(|o| !o.status.success()).unwrap_or(true) {
        eprintln!("no encoder at {}; set PRISMAL_FFMPEG to run this test", encoder.display());
        return;
    }
    let dir = scratch("video");
    let raster = Raster::new(&[]);
    let s = Settings { until: Some(3.0), ..settings(5.0, Captions::Both) };
    let c = clip(&mut example("rp08"), "ProjectileLesson", &s).unwrap();
    for name in ["lesson.mp4", "lesson.webm", "lesson.gif"] {
        let out = dir.join(name);
        encode(&raster, &c, &s, &encoder, &out).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(std::fs::metadata(&out).unwrap().len() > 1000, "{name} is empty");
    }
    assert!(std::fs::read_to_string(dir.join("lesson.vtt")).unwrap().starts_with("WEBVTT"));

    // The encoder's own reader agrees on the frames and the caption track.
    let probe = encoder.with_file_name(encoder.file_name().unwrap().to_string_lossy().replace("ffmpeg", "ffprobe"));
    let Ok(o) = std::process::Command::new(&probe)
        .args(["-v", "error", "-count_frames", "-show_entries", "stream=codec_type,nb_read_frames,width,height", "-of", "csv=p=0"])
        .arg(dir.join("lesson.mp4"))
        .output()
    else {
        return;
    };
    let text = String::from_utf8_lossy(&o.stdout);
    let video = text.lines().find(|l| l.starts_with("video")).unwrap_or_else(|| panic!("no video stream: {text}"));
    assert!(video.ends_with(&format!(",{}", c.frames.len())), "{video}");
    assert!(text.lines().any(|l| l.starts_with("subtitle")), "no caption track: {text}");
}
