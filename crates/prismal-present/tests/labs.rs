//! The lab presentations of RP-01 to RP-05: dynamic interactive sessions with a display
//! clock (RC section 12), interventions at the instant on display (RC-11.2), and the
//! sampled representations `trace` and `series_plot` (PK-6.3).

mod common;
use common::*;
use prismal_ir::build::num;
use prismal_present::frame::{Frame, Shape};
use prismal_present::interact::Interactive;
use prismal_present::Program;
use prismal_runtime::Config;

fn lab<'a>(prog: &'a Program, name: &str) -> Interactive<'a> {
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
