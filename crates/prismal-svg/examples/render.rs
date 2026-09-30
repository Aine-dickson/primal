//! Draws frames of a presentation as SVG documents.
//!
//! `cargo run -p prismal-svg --example render -- PROGRAM PRESENTATION [--at T]... [--fps N]
//! [--video] [--dark] [--out DIR]`
//!
//! PROGRAM is a source file (working syntax, or a Markdown document whose `text` and `cases`
//! blocks form the program) or the key of an embedded example (`rp01` to `rp08`, `g7-wheel`
//! ...). `--at T` draws the frame at time T (presentation time in a lesson, simulation time in
//! a session), and may be repeated; `--fps N` draws every frame from the start to the end at
//! N frames per second, an image sequence for video. Without either, the first frame is
//! drawn. Files are written to DIR (default `target/svg`).

use prismal_host::{Content, Document, Instance};
use prismal_svg::{render, Options, Theme};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let pos: Vec<&String> = {
        let mut out = vec![];
        let mut i = 0;
        while i < args.len() {
            if matches!(args[i].as_str(), "--at" | "--fps" | "--out") {
                i += 2;
                continue;
            }
            if !args[i].starts_with("--") {
                out.push(&args[i]);
            }
            i += 1;
        }
        out
    };
    if pos.len() < 2 {
        eprintln!("usage: render PROGRAM PRESENTATION [--at T]... [--fps N] [--video] [--dark] [--out DIR]");
        std::process::exit(2);
    }
    let value = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1));
    let ats: Vec<f64> = args.windows(2).filter(|w| w[0] == "--at").map(|w| w[1].parse().expect("--at takes seconds")).collect();
    let fps: Option<f64> = value("--fps").map(|v| v.parse().expect("--fps takes a number"));
    let out = PathBuf::from(value("--out").map(String::as_str).unwrap_or("target/svg"));
    let mut opts = Options::default();
    if args.iter().any(|a| a == "--dark") {
        opts.theme = Theme::DARK;
    }

    let src = match std::fs::read_to_string(pos[0]) {
        Ok(s) => s,
        Err(_) => match prismal_web::examples::all().into_iter().find(|e| &e.key == pos[0]) {
            Some(e) => e.source,
            None => fail(&format!("{} is neither a file nor an example", pos[0])),
        },
    };
    let doc = Document::load(Content::Text(src)).unwrap_or_else(|ds| fail(&format!("{ds:?}")));
    let prog = doc.program().unwrap_or_else(|| fail(&format!("{:?}", doc.diagnostics())));
    let mut inst = Instance::new(prog);
    let layout = inst.open(pos[1], args.iter().any(|a| a == "--video")).unwrap_or_else(|d| fail(&d.to_string()));
    let lesson = layout["mode"] == "lesson";
    let end = if lesson { layout["lesson"]["end"].as_f64() } else { layout["session"]["end"].as_f64() }.unwrap_or(0.0);

    let times: Vec<f64> = match fps {
        Some(r) => (0..=(end * r).floor() as usize).map(|k| k as f64 / r).collect(),
        None if ats.is_empty() => vec![0.0],
        None => ats.clone(),
    };
    std::fs::create_dir_all(&out).expect("output directory");
    let dt = fps.map(|r| 1.0 / r).unwrap_or(0.0);
    for (k, &t) in times.iter().enumerate() {
        let frame = if lesson {
            inst.frame(t, dt)
        } else {
            inst.seek(t);
            inst.frame(0.0, dt)
        };
        let name = match fps {
            Some(_) => format!("{}-{k:05}.svg", pos[1]),
            None => format!("{}-{t}s.svg", pos[1]),
        };
        let path = out.join(name);
        std::fs::write(&path, render(&layout, &frame, &opts)).expect("write");
        if fps.is_none() || k + 1 == times.len() {
            println!("{}", path.display());
        }
    }
    if fps.is_some() {
        println!("{} frames", times.len());
    }
}

fn fail(m: &str) -> ! {
    eprintln!("{m}");
    std::process::exit(1)
}
