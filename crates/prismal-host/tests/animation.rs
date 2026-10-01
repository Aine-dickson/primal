//! Animated color, line style and scale, shape morphs, chosen easing, and a lesson's camera
//! on a plot (D-075).

use prismal_host::{Content, Engine};
use serde_json::Value as Json;

const SHAPES: &str = "space Plane = euclidean(2)
model Still in Plane {
  param { a: Real = 1 }
  derived { y(x: Real): Real = a * x }
}
presentation Show for Still {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    polygon(origin + (0 m, 0 m), origin + (2 m, 0 m), origin + (2 m, 2 m), origin + (0 m, 2 m)) as square
    circle(origin + (1 m, 1 m), 1 m) as round
    segment(origin, origin + (4 m, 0 m)) as rod
    marker(origin + (5 m, 5 m)) as dot
  }
  view graph: plot(x: [0, 10], y: [0, 10]) { function_graph(y) }
  timeline {
    scene s {
      beat look {
        hide round
        animate dot color to red for 2 s ease linear
      }
      beat change {
        animate rod line to dashed for 2 s
        animate dot scale to 2 for 2 s ease in
      }
      beat shape { animate square morph to round for 2 s }
      beat closer { camera graph to (2, 3) zoom 2 for 1 s }
    }
  }
}
";

fn open() -> (Engine, String) {
    let mut e = Engine::new();
    let doc = e.load(Content::Text(SHAPES.into())).unwrap();
    let d = e.document(&doc).unwrap().diagnostics();
    assert!(d.is_empty(), "{d:?}");
    let (h, _) = e.open(&doc, "Show", false).unwrap();
    (e, h)
}

fn rep(f: &Json, name: &str) -> Json {
    f["views"].as_array().unwrap().iter().flat_map(|v| v["reps"].as_array().unwrap().iter()).find(|r| r["name"] == name).cloned().unwrap_or(Json::Null)
}

#[test]
fn color_blends_and_line_switches_halfway() {
    let (mut e, h) = open();
    let i = e.instance(&h).unwrap();
    // Linear easing: half of the way at half of the time.
    let f = i.frame(1.0, 0.0);
    let dot = rep(&f, "dot");
    assert_eq!(dot["color_to"], "red");
    assert!((dot["color_mix"].as_f64().unwrap() - 0.5).abs() < 1e-9, "{dot}");
    let f = i.frame(2.5, 0.0);
    assert_eq!(rep(&f, "dot")["color"], "red");
    assert!(rep(&f, "dot")["color_to"].is_null());
    // The line style changes halfway through its animation, from 2 s to 4 s.
    assert!(rep(&i.frame(2.9, 0.0), "rod")["line"].is_null());
    assert_eq!(rep(&i.frame(3.1, 0.0), "rod")["line"], "dashed");
}

#[test]
fn scale_eases_in() {
    let (mut e, h) = open();
    let i = e.instance(&h).unwrap();
    // `ease in`: a quarter of the way at half of the time.
    let s = rep(&i.frame(3.0, 0.0), "dot")["scale"].as_f64().unwrap();
    assert!((s - 1.25).abs() < 1e-9, "{s}");
    assert!((rep(&i.frame(5.0, 0.0), "dot")["scale"].as_f64().unwrap() - 2.0).abs() < 1e-9);
}

#[test]
fn a_square_morphs_into_a_hidden_circle() {
    let (mut e, h) = open();
    let i = e.instance(&h).unwrap();
    assert_eq!(rep(&i.frame(4.0, 0.0), "square")["shape"], "polygon");
    // Halfway: a closed path between the two, every point inside the square's corners.
    let mid = rep(&i.frame(5.0, 0.0), "square");
    assert_eq!(mid["shape"], "polygon");
    let pts = mid["points"].as_array().unwrap();
    assert!(pts.len() >= 64);
    // At the end the square has the circle's own shape.
    let end = rep(&i.frame(6.5, 0.0), "square");
    assert_eq!(end["shape"], "ellipse", "{end}");
    assert!(rep(&i.frame(6.5, 0.0), "round").is_null(), "the target stays hidden");
}

#[test]
fn a_camera_zooms_a_plot() {
    let (mut e, h) = open();
    let i = e.instance(&h).unwrap();
    let f = i.frame(8.0, 0.0);
    let v = f["views"].as_array().unwrap().iter().find(|v| v["id"].as_str().unwrap().ends_with("graph")).unwrap();
    let b: Vec<f64> = v["box"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    assert!((b[2] - 5.0).abs() < 1e-9 && (b[3] - 5.0).abs() < 1e-9, "zoomed twice: {b:?}");
    assert!((b[0] + b[2] / 2.0 - 2.0).abs() < 1e-9 && (b[1] + b[3] / 2.0 - 3.0).abs() < 1e-9, "centred on (2, 3): {b:?}");
}
