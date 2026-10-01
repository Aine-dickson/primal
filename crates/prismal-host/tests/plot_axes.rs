//! Plot axes that follow the data, display units on plot axes, and zoom and pan of plots
//! (PK-7.3, D-070).

use prismal_host::{Content, Engine, PointerEvent};

const SWING: &str = "model Swing {
  param { k: Quantity<M/T^2> = 4 N/m; m: Mass = 1 kg; x0: Length = 0.1 m }
  state { x: Length = x0; v: Velocity = 0 }
  flow { der(x) = v; der(v) = -(k / m) * x }
}
presentation Lab for Swing {
  view fixed: plot(x: [0 s, 2 s], y: [-0.05 m, 0.05 m]) { series_plot(x every 0.02 s) }
  view grows: plot(x: [0 s, 2 s], y: [-0.05 m, 0.05 m], follow: (x, y), y_unit: cm) { series_plot(x every 0.02 s) }
  view recent: plot(x: [0 s, 2 s], y: [-0.1 m, 0.1 m], window: 2 s) { series_plot(x every 0.02 s) }
  permit learner { zoom; pan }
}
";

fn session() -> (Engine, String) {
    let mut e = Engine::new();
    let doc = e.load(Content::Text(SWING.into())).unwrap();
    let (h, _) = e.open(&doc, "Lab", false).unwrap();
    (e, h)
}

/// The box `[x, y, width, height]` the frame gives a view.
fn view_box(e: &mut Engine, h: &str, view: &str) -> [f64; 4] {
    let f = e.instance(h).unwrap().frame(0.0, 0.0);
    let v = f["views"].as_array().unwrap().iter().find(|v| v["id"].as_str().unwrap().ends_with(view)).unwrap().clone();
    let b: Vec<f64> = v["box"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    [b[0], b[1], b[2], b[3]]
}

#[test]
fn a_following_axis_grows_to_hold_the_data() {
    let (mut e, h) = session();
    e.instance(&h).unwrap().seek(5.0);
    // The fixed plot keeps its declared ranges, though the swing reaches 0.1 m and 5 s.
    assert_eq!(view_box(&mut e, &h, "fixed"), [0.0, -0.05, 2.0, 0.1]);
    // The following plot grows on both axes to hold every sample, with room around them.
    let [x, y, w, hh] = view_box(&mut e, &h, "grows");
    assert_eq!(x, 0.0);
    assert!(x + w > 5.0, "time axis reaches the instant shown: {}", x + w);
    assert!(y < -0.09 && y + hh > 0.1, "y axis holds the swing: {y}, {}", y + hh);
}

#[test]
fn layout_gives_display_units_and_follow() {
    let (mut e, h) = session();
    let l = e.instance(&h).unwrap().layout();
    let grows = l["views"].as_array().unwrap().iter().find(|v| v["name"] == "grows").unwrap();
    assert_eq!(grows["units"], serde_json::json!(["s", "cm"]));
    assert_eq!(grows["unit_scale"], serde_json::json!([1.0, 0.01]));
    assert_eq!(grows["follow"], serde_json::json!([true, true]));
}

#[test]
fn a_plot_zooms_and_pans() {
    let (mut e, h) = session();
    let i = e.instance(&h).unwrap();
    let fixed = i.layout()["views"][0]["id"].as_str().unwrap().to_string();
    // Zoom out about the centre of the drawn plot: the ranges widen around the same point.
    let size = Some([560.0, 347.0]);
    assert_eq!(i.wheel(&fixed, 280.0, 173.5, size, 400.0, 0.0)["action"], "zoom");
    let [x, y, w, hh] = view_box(&mut e, &h, "fixed");
    assert!(w > 2.0 && hh > 0.1, "zoomed out: {w} {hh}");
    assert!((x + w / 2.0 - 1.0).abs() < 1e-9 && (y + hh / 2.0).abs() < 1e-9);
    // Drag the plot to the left: later times come into view.
    let i = e.instance(&h).unwrap();
    let ev = |phase, x| PointerEvent { phase, view: &fixed, x, y: 173.5, size, pointer: "mouse", id: None, time: 0.0 };
    assert_eq!(i.pointer(&ev("down", 400.0))["action"], "pan");
    i.pointer(&ev("move", 200.0));
    i.pointer(&ev("up", 200.0));
    let [x2, y2, w2, _] = view_box(&mut e, &h, "fixed");
    assert!(x2 > x && (w2 - w).abs() < 1e-9 && (y2 - y).abs() < 1e-9, "panned right in time: {x} -> {x2}");
    // Reset returns to the declared ranges.
    let i = e.instance(&h).unwrap();
    i.view_reset(&fixed);
    assert_eq!(view_box(&mut e, &h, "fixed"), [0.0, -0.05, 2.0, 0.1]);
}

const WAVE: &str = "model Wave {
  param { A: Real = 1 }
  derived { y(x: Real): Real = A * sin(x) }
}
presentation Graph for Wave {
  view graph: plot(x: [0, 6], y: [-2, 2]) { function_graph(y) }
  permit learner { zoom; pan }
}
";

/// The `x` extent of the first polyline the frame draws in its first view.
fn graph_extent(e: &mut Engine, h: &str) -> (f64, f64) {
    let f = e.instance(h).unwrap().frame(0.0, 0.0);
    let r = f["views"][0]["reps"].as_array().unwrap().iter().find(|r| r["shape"] == "polyline").unwrap().clone();
    let xs: Vec<f64> = r["points"].as_array().unwrap().iter().map(|p| p[0].as_f64().unwrap()).collect();
    (xs.iter().cloned().fold(f64::INFINITY, f64::min), xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
}

#[test]
fn a_function_graph_follows_zoom_and_pan() {
    let mut e = Engine::new();
    let doc = e.load(Content::Text(WAVE.into())).unwrap();
    let (h, _) = e.open(&doc, "Graph", false).unwrap();
    assert_eq!(graph_extent(&mut e, &h), (0.0, 6.0));
    let i = e.instance(&h).unwrap();
    let view = i.layout()["views"][0]["id"].as_str().unwrap().to_string();
    let size = Some([560.0, 347.0]);
    let ev = |phase, x| PointerEvent { phase, view: &view, x, y: 173.5, size, pointer: "mouse", id: None, time: 0.0 };
    i.pointer(&ev("down", 400.0));
    i.pointer(&ev("move", 100.0));
    i.pointer(&ev("up", 100.0));
    let [bx, _, bw, _] = view_box(&mut e, &h, "graph");
    let (lo, hi) = graph_extent(&mut e, &h);
    assert!(bx > 0.0 && (lo - bx).abs() < 1e-9 && (hi - (bx + bw)).abs() < 1e-9, "graph drawn over the range shown: {lo} {hi} vs {bx} {}", bx + bw);
    e.instance(&h).unwrap().view_reset(&view);
    assert_eq!(graph_extent(&mut e, &h), (0.0, 6.0));
}

#[test]
fn two_touches_pinch_a_plot() {
    let mut e = Engine::new();
    let doc = e.load(Content::Text(WAVE.into())).unwrap();
    let (h, _) = e.open(&doc, "Graph", false).unwrap();
    let i = e.instance(&h).unwrap();
    let view = i.layout()["views"][0]["id"].as_str().unwrap().to_string();
    let size = Some([560.0, 347.0]);
    let ev = |phase, x, id| PointerEvent { phase, view: &view, x, y: 173.5, size, pointer: "touch", id: Some(id), time: 0.0 };
    i.pointer(&ev("down", 250.0, 1));
    assert_eq!(i.pointer(&ev("down", 310.0, 2))["action"], "pinch");
    // Spreading the touches apart zooms in: a narrower range is shown.
    i.pointer(&ev("move", 190.0, 1));
    i.pointer(&ev("move", 370.0, 2));
    i.pointer(&ev("up", 370.0, 2));
    let [_, _, w, hh] = view_box(&mut e, &h, "graph");
    assert!((w - 2.0).abs() < 1e-9 && (hh - 4.0 / 3.0).abs() < 1e-9, "zoomed in three times: {w} {hh}");
}

#[test]
fn a_window_shows_the_latest_span() {
    let (mut e, h) = session();
    e.instance(&h).unwrap().seek(1.0);
    assert_eq!(view_box(&mut e, &h, "recent"), [0.0, -0.1, 2.0, 0.2]);
    e.instance(&h).unwrap().seek(5.0);
    let [x, _, w, _] = view_box(&mut e, &h, "recent");
    assert!((w - 2.0).abs() < 1e-9 && (x + w - 5.1).abs() < 0.03, "the last 2 s up to the instant shown: {x} {}", x + w);
}
