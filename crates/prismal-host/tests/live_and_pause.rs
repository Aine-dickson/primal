//! Drag mode `live` (PK-10.9, D-071) and the interactive failure policy `pause` (RC-10.3).

use prismal_host::{Content, Engine};

const PUCK: &str = "space Plane = euclidean(2)
model Puck in Plane {
  state { pos: Point = origin intervenable }
  flow { der(pos) = (1 m/s, 0 m/s) }
}
presentation Steer for Puck {
  view ice: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    marker(pos) as puck { on drag live as p { propose pos = p } }
  }
}
presentation Hold for Puck {
  view ice: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    marker(pos) as puck { on drag as p { propose pos = p } }
  }
}
";

fn open(src: &str, pres: &str) -> (Engine, String) {
    let mut e = Engine::new();
    let doc = e.load(Content::Text(src.into())).unwrap();
    let (h, _) = e.open(&doc, pres, false).unwrap();
    (e, h)
}

#[test]
fn a_live_drag_commits_as_the_clock_runs() {
    let (mut e, h) = open(PUCK, "Steer");
    let i = e.instance(&h).unwrap();
    assert_eq!(i.pointer_down("puck", None)["live"], true);
    // The pointer holds the puck 3 m up (view pixels, y down): the drag steers the run.
    assert_eq!(i.pointer_move(0.0, -30.0)["ok"], true);
    i.seek(1.0);
    i.seek(2.0);
    assert_eq!(i.pointer_up()["committed"], true);
    // One intervention at each instant the clock passed through, and the last one at 2 s.
    assert_eq!(i.session()["interventions"], 3);
    // The puck was held at x = 0 until released at 2 s, then moves on: x = 1 m at 3 s.
    i.seek(3.0);
    let f = i.frame(0.0, 0.0).to_string();
    assert!(f.contains("puck at x = 1 m, y = 3 m"), "{f}");
}

#[test]
fn a_hold_drag_commits_once_on_release() {
    let (mut e, h) = open(PUCK, "Hold");
    let i = e.instance(&h).unwrap();
    assert_eq!(i.pointer_down("puck", None)["live"], false);
    i.pointer_move(0.0, -30.0);
    i.pointer_move(0.0, -40.0);
    assert_eq!(i.pointer_up()["committed"], true);
    assert_eq!(i.session()["interventions"], 1);
}

const RISE: &str = "model Rise {
  param { limit: Real = 2 in [1, 10] }
  state { x: Real = 0 }
  flow { der(x) = 1 / 1 s }
  constraint below: x <= limit policy stop
}
presentation Lab for Rise {
  panel controls { slider(limit, range: [1, 10]) as lim }
}
";

#[test]
fn a_failed_session_pauses_until_a_change() {
    let (mut e, h) = open(RISE, "Lab");
    let i = e.instance(&h).unwrap();
    let s = i.session();
    assert_eq!(s["status"], "paused", "{s}");
    // The failure is located inside the solver's step: the run holds at the last valid
    // state, just before x passes the limit (RC-10.1).
    let t = s["failure"]["t"].as_f64().unwrap();
    let end = s["end"].as_f64().unwrap();
    assert!((t - 2.0).abs() < 1e-6 && end <= t && t - end < 1e-6, "paused where x passes the limit: {t}, {end}");
    assert!(s["failure"]["message"].as_str().unwrap().contains("below"), "{s}");
    // A change at the paused instant lets the run go on, up to the new limit.
    i.seek(end);
    i.set_control("lim", 10.0);
    let s2 = i.session();
    assert_eq!(s2["interventions"], 1);
    assert!((s2["failure"]["t"].as_f64().unwrap() - 10.0).abs() < 1e-6, "{s2}");
}
