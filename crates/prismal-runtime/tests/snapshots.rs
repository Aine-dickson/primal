//! Snapshots and resumed runs (RC section 14.1): a run resumed from a snapshot after its
//! action log changes is the run computed from the start, bit for bit (RC-14.2), whether an
//! action is added (a commit, a drag preview), removed (undo) or replaced.

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_runtime::{resume, run, Action, Config, Run, SolverConfig};

/// Bit-identical runs: `Debug` prints every `f64` in its shortest exact form.
fn assert_same(a: &Run, b: &Run, what: &str) {
    assert_eq!(format!("{:?}", a.committed), format!("{:?}", b.committed), "{what}: committed states");
    assert_eq!(format!("{:?}", a.segments), format!("{:?}", b.segments), "{what}: steps");
    assert_eq!(format!("{:?}", a.log), format!("{:?}", b.log), "{what}: event log");
    assert_eq!(format!("{:?}", a.diagnostics), format!("{:?}", b.diagnostics), "{what}: diagnostics");
    assert_eq!(format!("{:?}", a.rejected), format!("{:?}", b.rejected), "{what}: rejections");
    assert_eq!(format!("{:?}", a.status), format!("{:?}", b.status), "{what}: status");
}

/// Adds `action` at `t` to `prev`'s log, resumes, and compares with a full run; then removes
/// it again (undo) and compares with `prev`. Returns the resumed run.
fn add_and_undo(prev: &Run, t: f64, action: Action, what: &str) -> Run {
    let cfg = prev.config.clone().at(t, action);
    let resumed = resume(prev, cfg.clone());
    assert_same(&resumed, &run(&prev.model, cfg), &format!("{what}, added at {t}"));
    let undone = resume(&resumed, prev.config.clone());
    assert_same(&undone, prev, &format!("{what}, undone at {t}"));
    resumed
}

fn ball(name: &str) -> String {
    id("BouncingBall", name)
}

#[test]
fn resumed_runs_are_bit_identical() {
    let cm = compile(&bouncing_ball(settle()));
    let base = run(&cm, Config::until(4.0));
    let e = |v: f64| Action::Intervene(vec![set(&ball("e"), lit(v))]);
    // Between steps, at a bounce, at a snapshot's instant, at the start and at the end.
    let bounce = base.times("bounce")[1];
    let at_snapshot = base.snapshots[base.snapshots.len() / 2].t;
    for t in [0.0, 0.3, 1.2345, bounce, at_snapshot, 3.999, 4.0] {
        add_and_undo(&base, t, e(0.5), "restitution");
    }
    // Several actions, added one after another, before and after earlier ones.
    let one = add_and_undo(&base, 2.5, e(0.6), "first");
    let two = add_and_undo(&one, 1.1, e(0.9), "earlier second");
    let three = add_and_undo(&two, 3.2, e(0.4), "later third");
    // Replacing an action is a removal and an addition at once.
    let mut cfg = three.config.clone();
    cfg.log[1].action = e(0.7);
    assert_same(&resume(&three, cfg.clone()), &run(&cm, cfg), "replaced action");
    // The same log again: the run as it was.
    assert_same(&resume(&three, three.config.clone()), &three, "same log");
}

#[test]
fn requests_and_fixed_steps_resume_exactly() {
    let cm = compile(&projectile(true));
    let base = run(&cm, Config::until(10.0));
    let landed = base.times("landed")[0];
    let relaunch = Action::Request(id("Projectile", "event.relaunch"));
    let angle = Action::Intervene(vec![set(&id("Projectile", "angle"), num(60.0, "deg"))]);
    let a = add_and_undo(&base, landed, angle, "angle at landing");
    add_and_undo(&a, landed, relaunch.clone(), "relaunch after the angle");
    add_and_undo(&base, 1.0, Action::Intervene(vec![set(&id("Projectile", "k"), num(0.01, "/m"))]), "drag in flight");

    let cm = compile(&spring_mass());
    let rk4 = Config::until(6.0).solver(SolverConfig::Rk4 { h: Some(0.01) });
    let base = run(&cm, rk4);
    let k = |v: f64| Action::Intervene(vec![set(&id("SpringMass", "k"), num(v, "N/m"))]);
    for t in [0.005, 2.0, 3.333] {
        add_and_undo(&base, t, k(9.0), "rk4 stiffness");
    }
}

#[test]
fn a_late_action_resumes_near_its_instant() {
    // The work saved: a change at 9.5 s of a 10 s pendulum run resumes from a snapshot close
    // to it, and the resumed run is the full one.
    let cm = compile(&pendulum());
    let base = run(&cm, Config::until(10.0));
    assert!(base.snapshots.len() > 5, "snapshots at the start and every few steps: {}", base.snapshots.len());
    let t = 9.5;
    add_and_undo(&base, t, Action::Intervene(vec![set(&id("Pendulum", "L"), num(2.0, "m"))]), "pendulum length");
    let usable = base.snapshots.iter().filter(|s| s.t < t && s.theta < t).map(|s| s.t).fold(f64::NEG_INFINITY, f64::max);
    assert!(usable > 9.0, "latest usable snapshot before {t} s is at {usable} s");
}
