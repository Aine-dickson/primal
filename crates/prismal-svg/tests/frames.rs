//! The SVG renderer draws every presentation of the reference programs and of the guide:
//! each document is well formed, carries every representation of its frame with its text
//! alternative (PK-11.1), places marks at the frame's view coordinates, draws formulas from
//! their layouts (D-046), and follows the animations of a lesson frame by frame (D-042).
//!
//! The comparison with the web player's drawing of the same frames runs in a browser:
//! `node crates/prismal-svg/compare.mjs`.

use prismal_host::{Content, Document, Instance};
use prismal_svg::{all_reps, render, Options, Theme};
use serde_json::Value as Json;
use std::rc::Rc;

fn open(key: &str, pres: &str) -> (Instance, Json) {
    let ex = prismal_web::examples::all().into_iter().find(|e| e.key == key).unwrap_or_else(|| panic!("no example {key}"));
    let doc = Document::load(Content::Text(ex.source)).expect("compiles");
    let mut inst = Instance::new(doc.program().expect("checks"));
    let layout = inst.open(pres, false).unwrap_or_else(|d| panic!("{d}"));
    (inst, layout)
}

fn frame_at(inst: &mut Instance, layout: &Json, t: f64) -> Json {
    if layout["mode"] == "lesson" {
        inst.frame(t, 0.0)
    } else {
        inst.seek(t);
        inst.frame(0.0, 0.0)
    }
}

/// Every element is closed, in order.
fn well_formed(svg: &str) -> Result<(), String> {
    let mut stack: Vec<&str> = vec![];
    let mut rest = svg;
    while let Some(i) = rest.find('<') {
        rest = &rest[i + 1..];
        let end = rest.find('>').ok_or("unclosed tag")?;
        let tag = &rest[..end];
        rest = &rest[end + 1..];
        if let Some(name) = tag.strip_prefix('/') {
            match stack.pop() {
                Some(open) if open == name => {}
                other => return Err(format!("</{name}> closes {other:?}")),
            }
        } else if !tag.ends_with('/') {
            stack.push(tag.split(|c: char| c.is_whitespace()).next().unwrap());
        }
    }
    if stack.is_empty() {
        Ok(())
    } else {
        Err(format!("unclosed {stack:?}"))
    }
}

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn n(x: f64) -> String {
    let s = format!("{x:.3}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.into() }
}

/// Checks one drawn frame against its description.
fn check(what: &str, layout: &Json, frame: &Json, svg: &str) {
    well_formed(svg).unwrap_or_else(|e| panic!("{what}: {e}"));
    for r in all_reps(frame) {
        let (shape, id, text) = (r["shape"].as_str().unwrap(), r["id"].as_str().unwrap(), r["text"].as_str().unwrap());
        if shape == "axes" || shape == "grid" {
            continue;
        }
        let at = svg.find(&format!("data-rep=\"{}\"", esc(id))).unwrap_or_else(|| panic!("{what}: {id} not drawn"));
        let title = format!("<title>{}</title>", esc(text));
        assert!(svg[at..].starts_with(&format!("data-rep=\"{}\"", esc(id))) && svg[at..].contains(&title), "{what}: {id} lacks its text alternative");
        assert!(svg.contains(&esc(text)), "{what}: description of {id}");
    }
    // Markers of spatial views are drawn at the frame's view coordinates.
    for v in frame["views"].as_array().unwrap() {
        let spatial = layout["views"].as_array().unwrap().iter().any(|l| l["id"] == v["id"] && l["kind"] == "spatial");
        if !spatial {
            continue;
        }
        for r in v["reps"].as_array().unwrap() {
            if r["shape"] == "point" {
                let (x, y) = (r["at"][0].as_f64().unwrap(), r["at"][1].as_f64().unwrap());
                assert!(svg.contains(&format!("cx=\"{}\" cy=\"{}\"", n(x), n(y))), "{what}: {} at ({x}, {y})", r["id"]);
            }
        }
    }
    // Formulas: one text element per text run of their layouts.
    let runs: usize = all_reps(frame)
        .iter()
        .filter(|r| r["layout"].is_object())
        .map(|r| r["layout"]["items"].as_array().unwrap().iter().filter(|i| i["item"] == "text").count())
        .sum();
    assert_eq!(svg.matches("font-family=\"'Times New Roman'").count(), runs, "{what}: formula text runs");
}

#[test]
fn every_presentation_draws() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/svg-tests");
    std::fs::create_dir_all(&dir).unwrap();
    let opts = Options::default();
    let mut drawn = 0;
    for ex in prismal_web::examples::all() {
        let doc = Document::load(Content::Text(ex.source.clone())).expect("compiles");
        let prog = doc.program().expect("checks");
        for p in doc.catalogue()["presentations"].as_array().unwrap() {
            if p["kind"] == "observations" {
                continue;
            }
            let name = p["name"].as_str().unwrap();
            let mut inst = Instance::new(Rc::clone(&prog));
            let layout = inst.open(name, false).unwrap_or_else(|d| panic!("{d}"));
            let end = if layout["mode"] == "lesson" { layout["lesson"]["end"].as_f64() } else { layout["session"]["end"].as_f64() }.unwrap_or(0.0);
            for (k, t) in [0.0, end / 3.0, end].into_iter().enumerate() {
                let frame = frame_at(&mut inst, &layout, t);
                let svg = render(&layout, &frame, &opts);
                check(&format!("{} {name} at {t}", ex.key), &layout, &frame, &svg);
                std::fs::write(dir.join(format!("{}-{name}-{k}.svg", ex.key)), &svg).unwrap();
                drawn += 1;
            }
        }
    }
    assert!(drawn >= 60, "{drawn} frames drawn");
}

/// The header, title and description name the instant; controls, buttons and tables are
/// drawn in panels; a caption is drawn under the views.
#[test]
fn panels_and_captions() {
    let (mut inst, layout) = open("g8-freefall", "DropLab");
    let svg = render(&layout, &frame_at(&mut inst, &layout, 0.5), &Options::default());
    assert!(svg.contains("<title id=\"title\">DropLab: session run, simulation time 0.5 s</title>"));
    assert!(svg.contains(">DropLab, session run, t = 0.5 s</text>"));
    assert!(svg.contains(">Drop again</text>"), "button label");
    assert!(svg.contains(">3 rows</text>"), "table");
    assert!(svg.contains(">10 m</text>"), "slider value in its display unit");

    let (mut inst, layout) = open("rp08", "ProjectileLesson");
    let frame = frame_at(&mut inst, &layout, 20.0);
    let caption = frame["captions"][0].as_str().expect("a caption at 20 s");
    let svg = render(&layout, &frame, &Options::default());
    assert!(svg.contains(&format!(">{}</text>", esc(caption))));
    assert!(svg.contains("presentation time 20 s"));
    let dark = render(&layout, &frame, &Options { theme: Theme::DARK, header: false, ..Options::default() });
    assert!(dark.contains(&format!("fill=\"{}\"", Theme::DARK.bg)) && !dark.contains("presentation time"));
}

fn attr_values(svg: &str, attr: &str) -> Vec<f64> {
    svg.match_indices(&format!(" {attr}=\"")).map(|(i, m)| svg[i + m.len()..].split('"').next().unwrap().parse().unwrap()).collect()
}

fn view_box(svg: &str) -> Vec<f64> {
    let i = svg.find("data-view=").expect("a view");
    let start = svg[..i].rfind("viewBox=\"").unwrap() + 9;
    svg[start..].split('"').next().unwrap().split(' ').map(|x| x.parse().unwrap()).collect()
}

/// An image sequence of DropMovie at 10 frames per second follows its animations (D-042):
/// the ball fades in, the ground is drawn, the camera closes in and pulls back, the ball
/// fades out.
#[test]
fn animation_sequence() {
    let (mut inst, layout) = open("g8-freefall", "DropMovie");
    let end = layout["lesson"]["end"].as_f64().unwrap();
    let opts = Options::default();
    // Every tenth of a second, and the last instant.
    let times: Vec<f64> = (0..=(end * 10.0) as usize).map(|k| k as f64 / 10.0).chain([end]).collect();
    let frames: Vec<String> = times.iter().map(|&t| render(&layout, &inst.frame(t, 0.1), &opts)).collect();
    let opacities: Vec<f64> = frames.iter().flat_map(|s| attr_values(s, "opacity")).filter(|o| *o > 0.0 && *o < 1.0).collect();
    assert!(opacities.len() >= 10, "fades: {opacities:?}");
    let offsets: Vec<f64> = frames.iter().flat_map(|s| attr_values(s, "stroke-dashoffset")).collect();
    assert!(offsets.iter().any(|d| *d > 0.5) && offsets.iter().any(|d| *d < 0.2), "the ground is drawn: {offsets:?}");
    let widths: Vec<f64> = frames.iter().map(|s| view_box(s)[2]).collect();
    let (min, first, last) = (widths.iter().cloned().fold(f64::MAX, f64::min), widths[0], *widths.last().unwrap());
    assert!((min - first / 2.0).abs() < 1e-3, "zoom 2 halves the view box: {min} of {first}");
    assert!((last - first).abs() < 1e-3, "zoom 1 restores it: {widths:?}");
    for s in &frames {
        well_formed(s).unwrap();
    }
}
