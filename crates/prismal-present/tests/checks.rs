//! Static checks of presentations (PK-E01 to PK-E06), each an edit of a reference program.

mod common;
use common::*;
use prismal_present::Program;

fn codes(src: &str) -> Vec<&'static str> {
    let doc = prismal_syntax::compile(src).unwrap_or_else(|d| panic!("{d:?}")).doc;
    match Program::new(doc) {
        Ok(_) => vec![],
        Err(ds) => ds.iter().map(|d| d.code).collect(),
    }
}

fn edit(src: &str, from: &str, to: &str) -> String {
    assert_eq!(src.matches(from).count(), 1, "anchor `{from}`");
    src.replacen(from, to, 1)
}

#[test]
fn presentation_diagnostics() {
    let q = rp("RP-06");
    assert!(codes(&q).is_empty());
    // A control of a derived binding (PK-6.4, D-023).
    assert_eq!(codes(&edit(&q, "slider(a, range: [-5, 5], step: 0.1)", "slider(f)")), vec!["PK-E03"]);
    // A marker outside a spatial or plot view.
    assert_eq!(codes(&edit(&q, "slider(a, range: [-5, 5], step: 0.1)", "marker(at: (1, 2))")), vec!["PK-E05"]);
    // A representation kind the prototype does not implement.
    assert_eq!(codes(&edit(&q, "function_graph(f)", "table(f)")), vec!["PK-E06"]);

    let l = rp08();
    assert!(codes(&l).is_empty());
    // A velocity arrow without a velocity scale (PK-5.5): RP-08 as first written.
    assert_eq!(codes(&edit(&l, "arrow(vel, from: pos, scale: 1 m/s -> 2 px)", "arrow(vel, from: pos)")), vec!["PK-E04"]);
    // A scale of the wrong dimension.
    assert_eq!(codes(&edit(&l, "arrow(vel, from: pos, scale: 1 m/s -> 2 px)", "arrow(vel, from: pos, scale: 1 s -> 2 px)")), vec!["PK-E04"]);
    // Requesting an event that is not requestable (D-027).
    assert_eq!(codes(&edit(&l, "beat b8 { request relaunch;", "beat b8 { request landed;")), vec!["PK-E03"]);
    // An intervention on state that is not intervenable (D-023).
    assert_eq!(codes(&edit(&l, "intervene { set angle = 60 deg }", "intervene { set pos = origin }")), vec!["PK-E03"]);
    // An explore beat keeping a binding that is not intervenable (PK-9.9).
    assert_eq!(codes(&edit(&l, "keep angle", "keep flying")), vec!["PK-E03"]);

    let v = rp("RP-07");
    assert!(codes(&v).is_empty());
    // An inverse proposing a derived binding.
    assert_eq!(codes(&edit(&v, "propose u = h - A", "propose sum = h - A")), vec!["PK-E03"]);
    // An inverse whose proposal does not check: a point proposed for a vector (MK-E04).
    assert_eq!(codes(&edit(&v, "propose u = h - A", "propose u = h")), vec!["PK-E02"]);
}
