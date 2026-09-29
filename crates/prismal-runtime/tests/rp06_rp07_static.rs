//! RP-06 Function plot with draggable parameter and RP-07 Vector addition: the model and
//! interaction parts (the presentation parts need the presentation prototype).

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_ir::*;
use prismal_runtime::{Action, Config, Session};

fn q(n: &str) -> String {
    id("QuadraticDemo", n)
}

fn set_a(v: f64) -> Action {
    Action::Intervene(vec![set(&q("a"), lit(v))])
}

#[test]
fn rp06_interventions_drag_undo() {
    let cm = compile(&quadratic());
    assert!(cm.is_static, "RP-06 model is static (MK-14.15)");
    let f2 = apply(r(&q("f")), vec![lit(2.0)]);
    let a = r(&q("a"));
    let mut s = Session::new(cm, Config::until(0.0));

    // E1
    assert_eq!(s.value(&a).num(), 1.0, "RP-06.E1 a");
    assert_eq!(s.value(&f2).num(), 4.0, "RP-06.E1 f(2)");
    // E2
    s.commit(0.0, set_a(2.0)).unwrap();
    assert_eq!((s.value(&a).num(), s.value(&f2).num(), s.log_len()), (2.0, 8.0, 1), "RP-06.E2");
    // E3: rejected by the range constraint, not logged.
    assert!(s.commit(0.0, set_a(7.0)).is_err(), "RP-06.E3 rejection");
    assert_eq!((s.value(&a).num(), s.log_len()), (2.0, 1), "RP-06.E3 unchanged");
    // E4: drag previews; nothing committed.
    let proposals = [2.0, 3.0, 4.0, 6.0, 9.0];
    let valid: Vec<bool> = proposals.iter().map(|v| s.propose(0.0, set_a(*v)).is_ok()).collect();
    assert_eq!(valid, vec![true, true, true, false, false], "RP-06.E4 previews");
    assert_eq!(s.log_len(), 1, "RP-06.E4 log unchanged");
    // E5: release commits the last valid proposal.
    let last_valid = proposals.iter().zip(valid.iter()).filter(|(_, ok)| **ok).last().unwrap().0;
    s.commit(0.0, set_a(*last_valid)).unwrap();
    assert_eq!((s.value(&a).num(), s.value(&f2).num(), s.log_len()), (4.0, 16.0, 2), "RP-06.E5");
    // E6: two keyboard steps of 0.1.
    for _ in 0..2 {
        let cur = s.value(&a).num();
        s.commit(0.0, set_a(cur + 0.1)).unwrap();
    }
    assert_close(s.value(&a).num(), 4.2, 1e-12, "RP-06.E6 a");
    assert_eq!(s.log_len(), 4, "RP-06.E6 log");
    // E7, E8: undo and redo.
    s.undo();
    assert_close(s.value(&a).num(), 4.1, 1e-12, "RP-06.E7 undo");
    assert_eq!(s.log_len(), 3);
    s.redo();
    assert_close(s.value(&a).num(), 4.2, 1e-12, "RP-06.E8 redo");
    // E9: `f` is derived, not intervenable (RC-11.4, D-023).
    let attempt = Action::Intervene(vec![set(&q("f"), lambda(vec![Type::real()], param(0)))]);
    assert!(s.commit(0.0, attempt).is_err(), "RP-06.E9");
    assert_close(s.value(&a).num(), 4.2, 1e-12, "RP-06.E9 a unchanged");
    // E14: a single simulation instant; every commit is a new microstep at t0.
    assert!(s.current.committed.iter().all(|c| c.t == 0.0), "RP-06.E14 single instant");
    let ns: Vec<u32> = s.current.committed.iter().map(|c| c.n).collect();
    assert_eq!(ns, vec![0, 1, 2, 3, 4], "RP-06.E14 microsteps");
}

fn v(n: &str) -> String {
    id("VectorDemo", n)
}

fn pt(x: f64, y: f64) -> Expr {
    origin("Plane") + tuple(vec![num(x, "m"), num(y, "m")])
}

#[test]
fn rp07_vectors_and_drag() {
    let cm = compile(&vectors(|_| {}));
    let mut s = Session::new(cm, Config::until(0.0));
    let xy_of = |s: &Session, n: &str| xy(&s.value(&r(&v(n))));
    assert_eq!(xy_of(&s, "sum"), (3.0, 2.0), "RP-07.E1");
    assert_eq!(s.value(&r(&v("length"))).num(), 13f64.sqrt(), "RP-07.E2");
    assert_eq!(xy_of(&s, "B"), (4.0, 4.0), "RP-07.E3");
    // Drag the head of u to the model point (5 m, 1 m): the inverse proposes u = head - A.
    s.commit(0.0, Action::Intervene(vec![set(&v("u"), pt(5.0, 1.0) - r(&v("A")))])).unwrap();
    assert_eq!(xy_of(&s, "u"), (4.0, -1.0), "RP-07.E4 u");
    assert_eq!(xy_of(&s, "sum"), (4.0, 1.0), "RP-07.E4 sum");
    assert_eq!(s.value(&r(&v("length"))).num(), 17f64.sqrt(), "RP-07.E4 length");
    assert_eq!(xy_of(&s, "B"), (5.0, 3.0), "RP-07.E4 B");
    assert_eq!(xy_of(&s, "mid"), (5.0, 1.0), "RP-07.E5 mid");
}

#[test]
fn rp07_diagnostic_variants() {
    let pl = Type::point("Plane");
    let vl = Type::vector("Plane", "L");
    let c = codes(&vectors(|b| {
        b.derived("bad", pl.clone(), r(&v("A")) + r(&v("B")));
    }));
    assert!(c.contains(&"MK-E04"), "RP-07.D1 {c:?}");
    let c = codes(&vectors(|b| {
        b.derived("bad", pl.clone(), lit(2.0) * r(&v("A")));
    }));
    assert!(c.contains(&"MK-E04"), "RP-07.D2 {c:?}");
    let c = codes(&vectors(|b| {
        b.derived("bad", vl.clone(), r(&v("u")) + tuple(vec![lit(3.0), lit(0.0)]));
    }));
    assert!(c.contains(&"MK-E03"), "RP-07.D3 {c:?}");
    let c = codes(&vectors(|b| {
        let vel = b.param("vel", Type::vector("Plane", "L/T"), tuple(vec![num(1.0, "m/s"), num(0.0, "m/s")]));
        b.derived("bad", vl.clone(), r(&v("u")) + r(&vel));
    }));
    assert!(c.contains(&"MK-E01"), "RP-07.D4 {c:?}");
    let board = space("Board", 2);
    let c = codes_in(
        &[plane(), board],
        &vectors(|b| {
            let z = b.param("z", Type::vector("Board", "L"), tuple(vec![num(1.0, "m"), num(0.0, "m")]));
            b.derived("bad", vl.clone(), r(&v("u")) + r(&z));
        }),
    );
    assert!(c.contains(&"MK-E05"), "RP-07.D5 {c:?}");
    // D6: positive controls (MK-3.8, D-030).
    let c = codes(&vectors(|b| {
        b.derived("ok", vl.clone(), r(&v("u")) + tuple(vec![lit(0.0), lit(0.0)]));
        b.derived("ok2", vl.clone(), r(&v("u")) + lit(0.0));
    }));
    assert!(c.is_empty(), "RP-07.D6 must be accepted: {c:?}");
}
