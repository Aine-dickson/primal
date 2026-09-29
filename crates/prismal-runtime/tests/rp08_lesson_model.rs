//! RP-08 Narrated projectile lesson: the model-side behavior the timeline relies on
//! (requestable relaunch, D-027; interventions on the learner's branch; replay).
//! Timeline timing, captions and export need the presentation prototype.

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_runtime::{Action, Config};

const T45: f64 = 2.88320807823261;
const T60: f64 = 3.53119430697019;
const R60: f64 = 35.3119430697019;
const R45: f64 = 40.7747196738022;

fn p(n: &str) -> String {
    id("Projectile", n)
}

#[test]
fn keep_branch_relaunch_at_60deg() {
    let cm = compile(&projectile(true));
    // The lesson run lands at T45 (b2, and again after the seek in b5, by replay).
    let lesson = run_checked(&cm, Config::until(10.0));
    let t_land = lesson.times("landed")[0];
    assert_close(t_land, T45, 1e-9, "RP-08 first landing");

    // Case A-keep: at the branch instant the learner sets 60 deg (b7), then b8 requests relaunch.
    let branch = run_checked(
        &cm,
        Config::until(10.0)
            .at(t_land, Action::Intervene(vec![set(&p("angle"), num(60.0, "deg"))]))
            .at(t_land, Action::Request(p("event.relaunch"))),
    );
    assert!(branch.rejected.is_empty(), "{:?}", branch.rejected);
    let lands = branch.on("landed", &elapsed(), None);
    let ranges = branch.on("landed", &comp(r(&p("pos")), 0), None);
    assert_eq!(lands.len(), 2);
    assert_close(lands[1].num(), T45 + T60, 1e-8, "RP-08.E5 landing on the branch");
    assert_close(ranges[1].num(), R60, 1e-8, "RP-08.E5 range R60");

    // Case B-no-keep: the lesson run keeps 45 deg; relaunch lands at 2 T45 with range R45.
    let lesson2 = run_checked(&cm, Config::until(10.0).at(t_land, Action::Request(p("event.relaunch"))));
    let lands = lesson2.on("landed", &elapsed(), None);
    assert_close(lands[1].num(), 2.0 * T45, 1e-8, "RP-08.E9 landing");
    assert_close(lesson2.on("landed", &comp(r(&p("pos")), 0), None)[1].num(), R45, 1e-8, "RP-08.E9 range");
}

#[test]
fn only_requestable_events_can_be_requested() {
    let cm = compile(&projectile(true));
    let run = prismal_runtime::run(&cm, Config::until(1.0).at(0.5, Action::Request(p("event.landed"))));
    assert_eq!(run.rejected.len(), 1, "D-027: `landed` is not requestable");
    // Interventions outside the model's permissions never reach it (RC-11.4).
    let run = prismal_runtime::run(&cm, Config::until(1.0).at(0.5, Action::Intervene(vec![set(&p("pos"), origin("Plane"))])));
    assert_eq!(run.rejected.len(), 1, "state is not intervenable unless declared (D-023)");
}
