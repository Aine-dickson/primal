//! RP-06 Function plot with draggable parameter, through the presentation: the learner
//! script of docs/spec/reference-programs/RP-06-function-plot.md, E1 to E14.

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_ir::{Op, Target, Type};
use prismal_present::data::Data;
use prismal_present::frame::{Frame, Shape};
use prismal_present::interact::{Interactive, Key, Why};
use prismal_present::text::fmt_num;
use prismal_runtime::Config;

const A: &str = "QuadraticDemo.a";

fn num_of(i: &Interactive, obs: &str) -> f64 {
    match i.observe(obs).unwrap() {
        Data::Value(v) => v.num(),
        d => panic!("{obs}: {d:?}"),
    }
}

fn log_len(i: &Interactive) -> usize {
    i.observe("log").unwrap().len()
}

/// RP-06.E10: the graph, the slider, the formula's live value and the marker show the same
/// `a` in one frame (PK-7.5). Returns that `a`.
fn consistent(f: &Frame) -> f64 {
    let slider = f.reps().find(|r| r.kind == "slider").unwrap();
    let Shape::Control { value: a, .. } = slider.shape else { panic!() };
    let Shape::Point { at } = f.rep("handle").unwrap().shape else { panic!() };
    assert_eq!(at, [1.0, a], "marker at (1, f(1))");
    let Shape::Polyline { points } = &f.reps().find(|r| r.kind == "function_graph").unwrap().shape else { panic!() };
    assert_eq!(points.last().unwrap(), &[3.0, a * 9.0], "graph at x = 3");
    let Shape::Formula { symbols, .. } = &f.reps().find(|r| r.kind == "formula").unwrap().shape else { panic!() };
    let s = symbols.iter().find(|s| s.binding == A).unwrap();
    assert_eq!(s.value.as_deref(), Some(fmt_num(a).as_str()), "formula's live value");
    a
}

#[test]
fn rp06_learner_script() {
    let prog = program(&rp("RP-06"));
    let mut i = Interactive::new(&prog, "QuadraticPlot", Config::until(0.0)).unwrap();

    // E1
    assert_eq!((num_of(&i, "a_now"), num_of(&i, "f2")), (1.0, 4.0), "RP-06.E1");
    assert_eq!(consistent(&i.frame()), 1.0);
    // E2: 1.0 s, slider to 2.
    i.set_control_of("slider", A, lit(2.0)).unwrap();
    assert_eq!((num_of(&i, "a_now"), num_of(&i, "f2"), log_len(&i)), (2.0, 8.0, 1), "RP-06.E2");
    // E3: 2.0 s, slider to 7: rejected by the constraint, reported, not logged.
    let e = i.set_control_of("slider", A, lit(7.0)).unwrap_err();
    assert_eq!(e.why, Why::Rejected, "RP-06.E3 rejected by the model's constraint");
    assert_eq!((num_of(&i, "a_now"), log_len(&i)), (2.0, 1), "RP-06.E3 unchanged");
    assert_eq!(i.reports.len(), 1, "RP-06.E3 rejection reported to the presentation (RC-10.5)");
    consistent(&i.frame());

    // E4: 3.0 s to 3.5 s, drag the marker to y = 2, 3, 4, 6, 9.
    i.pointer_down("handle", None).unwrap();
    let mut valid = vec![];
    for y in [2.0, 3.0, 4.0, 6.0, 9.0] {
        valid.push(i.pointer_move([1.0, y]));
        let f = i.frame();
        let shown = consistent(&f);
        assert_eq!(f.rep("handle").unwrap().valid, Some(y <= 5.0), "RP-06.E4 marked valid or invalid");
        assert_eq!(shown, y.min(4.0), "RP-06.E4 the preview shows the last valid proposal");
        assert_eq!(log_len(&i), 1, "RP-06.E4 nothing committed");
    }
    assert_eq!(valid, vec![true, true, true, false, false], "RP-06.E4 previews");
    // E5: release commits the last valid proposal.
    assert!(i.pointer_up().unwrap());
    assert_eq!((num_of(&i, "a_now"), num_of(&i, "f2"), log_len(&i)), (4.0, 16.0, 2), "RP-06.E5");
    consistent(&i.frame());

    // E6: slider focused, Right twice.
    let slider = slider_id(&i);
    i.key(&slider, Key::Right).unwrap();
    i.key(&slider, Key::Right).unwrap();
    assert!((num_of(&i, "a_now") - 4.2).abs() <= 1e-12, "RP-06.E6");
    assert_eq!(log_len(&i), 4, "RP-06.E6 two entries");
    // E7, E8
    i.undo();
    assert!((num_of(&i, "a_now") - 4.1).abs() <= 1e-12, "RP-06.E7");
    assert_eq!(log_len(&i), 3);
    i.redo();
    assert!((num_of(&i, "a_now") - 4.2).abs() <= 1e-12, "RP-06.E8");
    assert_eq!(log_len(&i), 4);
    // E9: `f` is derived: refused before reaching the model.
    let attempt = vec![Op::Set { target: Target::of("QuadraticDemo.f"), value: lambda(vec![Type::real()], param(0)) }];
    let e = i.submit(attempt).unwrap_err();
    assert_eq!(e.why, Why::Refused, "RP-06.E9 {e:?}");
    assert!((num_of(&i, "a_now") - 4.2).abs() <= 1e-12, "RP-06.E9 a unchanged");
    assert_eq!(log_len(&i), 4);

    let f = i.frame();
    let a = consistent(&f);
    // E11, E12: text alternatives.
    let slider = f.reps().find(|r| r.kind == "slider").unwrap();
    assert!(slider.text.contains("a = 4.2"), "RP-06.E11 {}", slider.text);
    let marker = f.rep("handle").unwrap();
    assert!(marker.text.contains("x = 1, y = 4.2"), "RP-06.E12 {}", marker.text);
    assert!(f.reps().find(|r| r.kind == "formula").unwrap().text.starts_with("f(x) = a x^2"), "D-034");

    // E13: the marker focused, Up moves it by its step along the inverse (1/100 of the
    // y axis span, 0.25) and commits like a pointer drag.
    i.key("handle", Key::Up).unwrap();
    assert!((num_of(&i, "a_now") - (a + 0.25)).abs() <= 1e-12, "RP-06.E13 {}", num_of(&i, "a_now"));
    assert_eq!(log_len(&i), 5, "RP-06.E13 one intervention");
    i.key("handle", Key::Down).unwrap();
    assert!((num_of(&i, "a_now") - a).abs() <= 1e-12, "RP-06.E13 back");
    consistent(&i.frame());

    // E14: one simulation instant; every commit a new microstep at t0.
    assert!(i.session.current.committed.iter().all(|c| c.t == 0.0), "RP-06.E14");
}

fn slider_id(i: &Interactive) -> String {
    i.frame().reps().find(|r| r.kind == "slider").unwrap().id.clone()
}
