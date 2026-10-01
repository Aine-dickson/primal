//! The lab presentations of RP-01 to RP-05: dynamic interactive sessions with a display
//! clock (RC section 12), interventions at the instant on display (RC-11.2), and the
//! sampled representations `trace` and `series_plot` (PK-6.3).

mod common;
use common::*;
use prismal_ir::build::num;
use prismal_present::frame::{Frame, Shape};
use prismal_present::interact::Interactive;
use prismal_present::timeline::ease;
use prismal_present::Program;
use prismal_runtime::Config;

fn lab(prog: &Program, name: &str) -> Interactive {
    Interactive::new(prog, name, Config::until(60.0)).unwrap_or_else(|d| panic!("{d:?}"))
}

fn point(f: &Frame, name: &str) -> [f64; 2] {
    match f.rep(name).unwrap_or_else(|| panic!("no {name}")).shape {
        Shape::Point { at } => at,
        ref s => panic!("{name}: {s:?}"),
    }
}

fn polyline<'a>(f: &'a Frame, kind: &str, n: usize) -> &'a Vec<[f64; 2]> {
    match &f.reps().filter(|r| r.kind == kind).nth(n).unwrap().shape {
        Shape::Polyline { points } => points,
        s => panic!("{kind}: {s:?}"),
    }
}

fn close(a: f64, b: f64, tol: f64, what: &str) {
    assert!((a - b).abs() <= tol, "{what}: {a} vs {b}");
}

#[test]
fn projectile_lab() {
    let prog = program(&rp("RP-01"));
    let mut i = lab(&prog, "ProjectileLab");
    let (v, a, g) = (20.0, std::f64::consts::FRAC_PI_4, 9.81);
    assert_eq!(i.t, 0.0, "a session starts at t0");
    assert_eq!(i.seek(1.0), 1.0);
    let f = i.frame();
    assert_eq!(f.t, 1.0);
    // 10 px per metre, y up.
    let ball = point(&f, "ball");
    close(ball[0], 10.0 * v * a.cos(), 1e-6, "x at 1 s");
    close(ball[1], -10.0 * (v * a.sin() - 0.5 * g), 1e-6, "y at 1 s");
    // The trace ends at the ball; the series ends at (1 s, y).
    let trace = polyline(&f, "trace", 0);
    assert_eq!(trace.len(), 21, "0, 0.05, ..., 0.95 s and 1 s");
    assert_eq!(*trace.last().unwrap(), ball);
    let series = polyline(&f, "series_plot", 0);
    assert_eq!(series.len(), 51);
    close(series.last().unwrap()[0], 1.0, 1e-12, "series time");
    close(series.last().unwrap()[1], -ball[1] / 10.0, 1e-9, "series value");
    // Seeking away and back reproduces the frame (RC-12.2).
    i.seek(2.5);
    i.seek(1.0);
    assert_eq!(i.frame(), f);

    // Weaker gravity from t = 1 s: the flight so far is unchanged, the landing is later.
    let landed = |i: &Interactive| i.session.current.log.iter().find(|l| l.name == "landed").unwrap().t;
    let t45 = landed(&i);
    close(t45, 2.0 * v * a.sin() / g, 1e-9, "landing at 9.81 m/s^2");
    let slider = f.reps().find(|r| r.kind == "slider" && r.text.contains("g =")).unwrap().id.clone();
    i.set_control(&slider, num(2.0, "m/s^2")).unwrap();
    assert!(landed(&i) > t45 + 1.0, "later landing");
    let log = i.session.log();
    assert_eq!((log.len(), log[0].t), (1, 1.0), "applied at the instant on display (RC-11.2)");
    i.seek(0.5);
    let before = i.frame();
    i.undo();
    i.seek(0.5);
    assert_eq!(point(&i.frame(), "ball"), point(&before, "ball"), "the past is unchanged");
    close(landed(&i), t45, 0.0, "undo restores the landing");
    // Reset: a new run with an empty log, shown at t0.
    i.redo();
    i.reset();
    assert_eq!((i.t, i.session.log().len()), (0.0, 0));
}

#[test]
fn bounce_lab() {
    let prog = program(&rp("RP-03"));
    let mut i = lab(&prog, "BounceLab");
    i.seek(0.3);
    let f = i.frame();
    // A plot marker at (0, y) in plot coordinates: metres on the vertical axis.
    let ball = point(&f, "ball");
    assert_eq!(ball[0], 0.0);
    close(ball[1], 1.0 - 0.5 * 9.81 * 0.09, 1e-6, "falling");
    // The run continues after the Zeno limit (settle) to the end of the session.
    assert!(i.end_time() >= 59.99, "{}", i.end_time());
    i.seek(10.0);
    assert_eq!(point(&i.frame(), "ball"), [0.0, 0.0], "resting");
    assert!(i.events_between(0.0, 1.0).iter().any(|e| e == "bounce"));
}

#[test]
fn pendulum_lab() {
    let prog = program(&rp("RP-04"));
    let mut i = lab(&prog, "PendulumLab");
    let f = i.frame();
    let Shape::Segment { from, to } = f.reps().find(|r| r.kind == "segment").unwrap().shape else { panic!() };
    assert_eq!(from, [0.0, 0.0]);
    close(to[0], 150.0 * 10f64.to_radians().sin(), 1e-9, "bob x");
    assert_eq!(to, point(&f, "bob"));
    // Longer rod from 1 s: the energy series jumps at 1 s.
    i.seek(1.0);
    let slider = f.reps().find(|r| r.kind == "slider" && r.text.contains("L =")).unwrap().id.clone();
    i.set_control(&slider, num(2.0, "m")).unwrap();
    i.seek(2.0);
    let e = polyline(&i.frame(), "series_plot", 1).clone();
    let at = |t: f64| e.iter().find(|p| (p[0] - t).abs() < 1e-9).unwrap()[1];
    close(at(0.5), at(0.9), 1e-6, "energy conserved before");
    assert!((at(1.2) - at(0.9)).abs() > 1e-3, "energy changes with L");
}

#[test]
fn spring_lab() {
    let prog = program(&rp("RP-05"));
    let mut i = lab(&prog, "SpringLab");
    close(point(&i.frame(), "mass")[0], 0.1, 1e-12, "x0");
    // Changing k breaks the checked conservation equation: reported, not enforced.
    i.seek(1.0);
    let f = i.frame();
    let slider = f.reps().find(|r| r.kind == "slider" && r.text.contains("k =")).unwrap().id.clone();
    i.set_control(&slider, num(9.0, "N/m")).unwrap();
    assert!(i.session.current.diagnostics.iter().any(|d| d.message.contains("conservation")), "{:?}", i.session.current.diagnostics);
    let t = f.reps().find(|r| r.kind == "formula").unwrap();
    assert!(t.text.starts_with("T = 2 π sqrt(m / k), where m = 1 kg, k = 4 N/m"), "{}", t.text);
}

#[test]
fn sampled_sources_are_checked() {
    let base = rp("RP-03");
    let with = |view: &str| {
        let src = format!("{base}\npresentation P for BouncingBall {{\n  {view}\n}}\n");
        let doc = prismal_syntax::compile(&src).unwrap_or_else(|d| panic!("{d:?}")).doc;
        match Program::new(doc) {
            Ok(_) => vec![],
            Err(ds) => ds.into_iter().map(|d| d.code).collect::<Vec<_>>(),
        }
    };
    assert_eq!(with("view h: plot(x: [0 s, 6 s], y: [0 m, 1 m]) { series_plot(y every 0.01 s) }"), Vec::<&str>::new());
    assert_eq!(with("view h: plot(x: [0, 6], y: [0 m, 1 m]) { series_plot(y every 0.01 s) }"), vec!["PK-E04"], "x axis not time");
    assert_eq!(with("view h: plot(x: [0 s, 6 s], y: [0, 1]) { series_plot(y every 0.01 s) }"), vec!["PK-E04"], "y dimension");
    assert_eq!(with("view h: plot(x: [0 s, 6 s], y: [0 m, 1 m]) { series_plot(y every 0.01 m) }"), vec!["PK-E04"], "interval");
    assert_eq!(with("view h: plot(x: [0 s, 6 s], y: [0 m, 1 m]) { series_plot(y) }"), vec!["PK-E05"], "not sampled");
    assert_eq!(with("view h: plot(x: [-1, 1], y: [0 m, 1 m]) { marker(at: (0, v)) }"), vec!["PK-E05"], "marker units");
    let err = prismal_syntax::compile(&format!("{base}\nmodel Q {{ derived {{ a: Real = max(1 every 2, 3) }} }}\n")).unwrap_err();
    assert_eq!(err[0].code, "SX-E08");
}

const DROP: &str = "space Plane = euclidean(2)
model FreeFall in Plane {
  param { g: Acceleration = 9.81 m/s^2; h: Length = 10 m in [1 m, 50 m] }
  state { pos: Point = origin + (0 m, h); vel: Vector<Velocity> = 0 }
  discrete { airborne: Boolean = true }
  flow {
    der(pos) = if airborne then vel else 0
    der(vel) = if airborne then (0, -g) else 0
  }
  event landed on falling(pos.y) { set airborne = false; set vel = 0 }
  event drop on request { set pos = origin + (0 m, h); set vel = 0; set airborne = true }
  equation fall_time: sqrt(2 * h / g) == sqrt(2 * h / g)
}
presentation DropLab for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) {
    marker(pos) as ball
    polygon(origin, origin + (1 m, 0 m), origin + (0 m, 1 m))
  }
  panel controls {
    button(drop, label: \"Drop again\")
    table((pos.y, vel.y) every 0.5 s)
    equation(fall_time, live: true)
  }
}
presentation DropLesson for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) { marker(pos) as ball }
  timeline {
    scene s {
      beat a { run rate 1; wait 1 s }
      beat b { hide ball; wait 1 s }
    }
  }
}
presentation DropMovie for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 12 px, y: up) {
    axes
  }
  timeline {
    scene only {
      beat appear   { reveal fade in scene { marker(pos) as ball } }
      beat ground   { reveal draw for 2 s in scene { polyline(origin + (-5 m, 0 m), origin + (5 m, 0 m)) } }
      beat close_up { camera scene to pos zoom 2 for 1.5 s }
      beat fall     { run rate 0.5 until landed }
      beat away     { camera scene zoom 1 for 1 s; hide ball for 1 s }
    }
  }
}
";

/// `button`, `table`, `polygon`, `equation` (PK-6.3) and `hide` (PK-9.2).
#[test]
fn button_table_polygon_equation_hide() {
    let prog = program(DROP);
    let mut i = lab(&prog, "DropLab");
    // The ball lands at sqrt(2 h / g) = 1.42784 s; at 3 s it rests; the button drops it again.
    i.seek(3.0);
    let f = i.frame();
    let Shape::Button { label, .. } = &f.rep("button.1").unwrap().shape else { panic!() };
    assert_eq!(label, "Drop again");
    i.press("button.1").unwrap();
    assert!(i.session.current.log.iter().any(|l| l.name == "drop" && l.t == 3.0), "requested at the instant shown");
    i.seek(3.5);
    let y = point(&i.frame(), "ball")[1];
    close(y, -10.0 * (10.0 - 0.5 * 9.81 * 0.25), 1e-6, "falling again");
    assert!(i.press("ball").is_err(), "a marker is not a button");
    // Table: the time, then one column per component, up to the instant shown.
    let f = i.frame();
    let Shape::Table { columns, rows } = &f.rep("table.1").unwrap().shape else { panic!() };
    assert_eq!(columns, &["t (s)", "pos.y", "vel.y"]);
    assert_eq!(rows.len(), 8, "0, 0.5, ..., 3.5 s");
    assert_eq!(rows[0], vec!["0", "10 m", "0 m/s"]);
    // Polygon in view coordinates; equation with live values.
    let Shape::Polygon { points } = &f.rep("polygon.1").unwrap().shape else { panic!() };
    assert_eq!(points, &vec![[0.0, 0.0], [10.0, 0.0], [0.0, -10.0]]);
    let eq = i.frame().rep("equation.1").unwrap().clone();
    assert!(eq.text.starts_with("equation fall_time: sqrt(2 h / g) = sqrt(2 h / g), where h = 10 m"), "{}", eq.text);
    // A button for an event that is not requestable is a presentation error (PK-E03).
    let bad = DROP.replace("button(drop,", "button(landed,");
    let doc = prismal_syntax::compile(&bad).unwrap().doc;
    let codes: Vec<&str> = Program::new(doc).err().unwrap().iter().map(|d| d.code).collect();
    assert_eq!(codes, vec!["PK-E03"]);

    // `hide ball` at 1 s: shown before, not after.
    let pb = prismal_present::timeline::play(&prog, "DropLesson", Config::until(10.0), prismal_present::timeline::Medium::Interactive, vec![]).unwrap();
    assert!(pb.frame(0.5, 0.1).rep("ball").is_some());
    assert!(pb.frame(1.5, 0.1).rep("ball").is_none());
}

/// Animations as named effects (D-042): the frame values of `reveal fade`, `reveal draw`,
/// `camera` and `hide ... for`, in guide chapter 8's `DropMovie`.
#[test]
fn animation_frames() {
    use prismal_present::frame::Camera;
    use prismal_present::timeline::{play, Medium};
    let prog = program(DROP);
    let pb = play(&prog, "DropMovie", Config::until(10.0), Medium::Interactive, vec![]).unwrap();
    let (g, h): (f64, f64) = (9.81, 10.0);
    let fall = (2.0 * h / g).sqrt();
    let camera = |p: f64| pb.frame(p, 0.1).views.iter().find(|v| v.id.ends_with("scene")).unwrap().camera.clone();
    let ground = |p: f64| pb.frame(p, 0.1).reps().find(|r| r.kind == "polyline").cloned();

    // appear, 0 to 1 s: the ball fades in; eased, so half way is exactly 0.5.
    assert_eq!(pb.frame(0.5, 0.1).rep("ball").unwrap().opacity, Some(0.5));
    close(pb.frame(0.25, 0.1).rep("ball").unwrap().opacity.unwrap(), ease(0.25), 0.0, "eased fade");
    assert_eq!(pb.frame(1.0, 0.1).rep("ball").unwrap().opacity, None, "fully shown after the fade");
    assert!(ground(0.5).is_none(), "not yet revealed");
    assert_eq!(camera(2.0), None, "no camera before the first move");

    // ground, 1 to 3 s: the line is drawn from start to end, at full opacity.
    let mid = ground(2.0).unwrap();
    assert_eq!((mid.drawn, mid.opacity), (Some(0.5), None));
    let Shape::Polyline { points } = &mid.shape else { panic!() };
    assert_eq!(points, &vec![[-60.0, 0.0], [60.0, 0.0]]);
    assert_eq!(ground(3.0).unwrap().drawn, None);

    // close_up, 3 to 4.5 s: from the view's own framing to the ball, zoom 1 to 2.
    assert_eq!(camera(3.75), Some(Camera { center: Some([0.0, -12.0 * h]), zoom: 1.5, blend: 0.5 }));
    assert_eq!(camera(4.5), Some(Camera { center: Some([0.0, -12.0 * h]), zoom: 2.0, blend: 1.0 }));

    // fall, at half speed: the camera keeps following the ball.
    let end_fall = pb.beat("fall").end;
    close(end_fall, 4.5 + fall / 0.5, 1e-8, "end of fall");
    let f = pb.frame(5.5, 0.1);
    close(f.t, 0.5, 1e-12, "simulation time");
    let c = camera(5.5).unwrap();
    assert_eq!(c.center, Some(point(&f, "ball")));
    close(c.center.unwrap()[1], -12.0 * (h - 0.5 * g * 0.25), 1e-6, "ball height");

    // away: zoom back to 1 and fade the ball out over 1 s, then it is gone.
    let p = end_fall + 0.5;
    let c = camera(p).unwrap();
    close(c.zoom, 1.5, 1e-12, "zoom half way back");
    close(c.center.unwrap()[1], 0.0, 1e-6, "centred on the landed ball");
    close(pb.frame(p, 0.1).rep("ball").unwrap().opacity.unwrap(), 0.5, 1e-9, "fading out");
    assert!(pb.frame(end_fall + 1.0, 0.1).rep("ball").is_none(), "hidden after the fade");
    close(camera(end_fall + 1.0).unwrap().zoom, 1.0, 1e-12, "own scale");
    // The same frames every time the movie plays (D-042, PK-8.4).
    let again = play(&prog, "DropMovie", Config::until(10.0), Medium::Interactive, vec![]).unwrap();
    for p in [0.3, 1.7, 3.2, 6.0, end_fall + 0.4] {
        assert_eq!(again.frame(p, 0.1), pb.frame(p, 0.1));
    }
}

const RIGID: &str = "
presentation RigidLab for Pendulum {
  view scene: spatial(Plane, scale: 1 m -> 100 px, y: up) {
    marker(bob) as bob
    group(at: pivot, rotate: θ) as body {
      segment(origin, origin + (0 m, -L))
      marker(origin + (0 m, -L)) as tip
      arrow((0.5 m, 0 m), from: origin + (0 m, -L)) as tangent
      group(at: origin + (0 m, -L), scale: 2) as badge {
        polygon(origin, origin + (0.1 m, 0 m), origin + (0 m, 0.1 m))
      }
    }
  }
  timeline {
    scene s {
      beat a { run rate 1; wait 1 s }
      beat b { highlight tip; hide tangent for 1 s }
      beat c { reveal draw for 2 s in scene { group(rotate: 90 deg) { marker(origin + (1 m, 0 m)) as far; segment(origin, origin + (1 m, 0 m)) } } }
    }
  }
}
";

/// `group` (PK-6.3b, D-043): members drawn with a shared transform, nested groups,
/// text alternatives, timeline actions on members, formatting and diagnostics.
#[test]
fn groups() {
    let src = format!("{}\n{RIGID}", rp("RP-04"));
    let prog = program(&src);
    let mut i = lab(&prog, "RigidLab");
    let f = i.frame();
    // The body hangs from the pivot turned by θ: its tip is the bob.
    let (tip, bob) = (point(&f, "tip"), point(&f, "bob"));
    close(tip[0], bob[0], 1e-9, "tip x");
    close(tip[1], bob[1], 1e-9, "tip y");
    let th = 10f64.to_radians();
    close(tip[0], 100.0 * th.sin(), 1e-9, "tip at L sin θ");
    // Vectors turn with the group: the tangent is perpendicular to the rod.
    let Shape::Arrow { from, to } = f.rep("tangent").unwrap().shape else { panic!() };
    assert_eq!(from, tip);
    close(to[0] - from[0], 50.0 * th.cos(), 1e-9, "tangent x");
    close(to[1] - from[1], -50.0 * th.sin(), 1e-9, "tangent y (view y down)");
    // A nested group composes: placed at the tip, turned by θ, scaled by 2.
    let Shape::Polygon { points } = &f.rep("badge").map(|g| match &g.shape {
        Shape::Group { members } => members[0].shape.clone(),
        s => panic!("{s:?}"),
    }).unwrap() else { panic!() };
    close(points[0][0], tip[0], 1e-9, "badge at the tip");
    close(points[1][0] - tip[0], 20.0 * th.cos(), 1e-9, "scaled and turned");
    // Text alternatives: a summary of the members, in the view's space.
    let text = &f.rep("body").unwrap().text;
    assert!(text.starts_with("body: segment; tip at x = 0.1736"), "{text}");
    // Moving on: the body follows θ.
    i.seek(0.7);
    let f = i.frame();
    close(point(&f, "tip")[0], point(&f, "bob")[0], 1e-9, "follows");

    // Timeline actions reach members by name.
    let pb = prismal_present::timeline::play(&prog, "RigidLab", Config::until(10.0), prismal_present::timeline::Medium::Interactive, vec![]).unwrap();
    let f = pb.frame(1.2, 0.1);
    assert!(f.rep("tip").unwrap().highlighted);
    assert!(f.rep("tangent").unwrap().opacity.unwrap() < 1.0);
    assert!(pb.frame(2.5, 0.1).rep("tangent").is_none(), "hidden after its fade");
    assert!(pb.frame(2.5, 0.1).rep("tip").is_some());
    // A group revealed by drawing: its paths are drawn, its markers fade in.
    let f = pb.frame(3.0, 0.1);
    let far = f.rep("far").unwrap();
    close(far.opacity.unwrap(), 0.5, 1e-12, "marker fades");
    let p = point(&f, "far");
    assert!(p[0].abs() < 1e-9 && (p[1] + 100.0).abs() < 1e-9, "turned 90 deg: {p:?}");
    let seg = f.reps().find(|r| r.kind == "segment" && r.drawn.is_some()).unwrap();
    close(seg.drawn.unwrap(), 0.5, 1e-12, "segment drawn");

    // The formatter prints groups and the identities of members survive a round trip.
    let doc = prismal_syntax::compile(&src).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert!(printed.contains("group(at: pivot, rotate: θ) as body {"), "{printed}");
    assert_eq!(prismal_syntax::compile(&printed).unwrap().doc, doc);

    // Diagnostics.
    let codes = |edit: &str, with: &str| -> Vec<&'static str> {
        let bad = src.replace(edit, with);
        let doc = prismal_syntax::compile(&bad).unwrap_or_else(|d| panic!("{d:?}")).doc;
        Program::new(doc).err().map(|ds| ds.iter().map(|d| d.code).collect()).unwrap_or_default()
    };
    assert_eq!(codes("rotate: θ)", "rotate: L)"), vec!["PK-E04"], "rotate is an angle");
    assert_eq!(codes("scale: 2)", "scale: 0)"), vec!["PK-E02"], "scale is positive");
    // A member of a group may be dragged (D-062).
    assert_eq!(codes("as tip", "as tip { on drag as p { propose θ0 = 0 } }"), Vec::<&str>::new());
    assert_eq!(codes("segment(origin, origin + (0 m, -L))", "trace(bob every 0.1 s)"), vec!["PK-E05"], "no sampled members");
    assert_eq!(codes("view scene: spatial(Plane, scale: 1 m -> 100 px, y: up) {", "view scene: plot(x: [-1, 1], y: [-1, 1]) {"), vec!["PK-E05", "PK-E05"]);
}
