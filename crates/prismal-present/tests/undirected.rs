//! Undirected relations (MK-8.5b, D-064): `s.has(o)` and `s.other(o)` read a relation's
//! endpoints without an order; an undirected relation's endpoints are in one collection and
//! are not read by role in the model outside its body; a presentation may draw them by role.

mod common;
use common::*;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_runtime::Config;

/// The spring pair of guide chapter 9, written without directions: each ball is pulled
/// towards the other end of every spring it is on. The gap is `1 + cos(2 t)` m, as with the
/// directed springs.
const PAIR: &str = "
space Plane = euclidean(2)

model Pair in Plane {
  object Ball {
    param { m: Mass = 1 kg }
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  undirected relation Spring(a in balls, b in balls) {
    param {
      k:    Quantity<M/T^2> = 2 N/m
      rest: Length          = 1 m
    }
    derived { stretch: Length = |b.pos - a.pos| - rest }
  }
  parts {
    balls: Ball[3, max 3] { pos = origin + ((2 * index - 3) * 1 m, 0 m) }
    springs: Spring[1, max 4] {
      a = balls[2]
      b = balls[1]
    }
  }
  flow {
    for o in balls {
      der(o.vel) = sum(s.k * s.stretch * (s.other(o).pos - o.pos) / |s.other(o).pos - o.pos| for s in springs if s.has(o)) / o.m
    }
  }
  event join on request { connect springs(balls[3], balls[2]) }
  derived {
    gap:    Length = balls[2].pos.x - balls[1].pos.x
    middle: Real   = count(s for s in springs if s.has(balls[2]))
    loose:  Real   = count(s for s in springs if s.has(balls[3]))
  }
}

presentation Springs for Pair {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for o in balls { marker(o.pos) as ball }
    for s in springs { segment(s.a.pos, s.b.pos) as spring }
  }
  observe {
    gap    = gap live
    middle = middle live
    loose  = loose live
  }
}

run oscillates of Pair with Springs {
  until t0 + 1 s
  expect {
    gap    == 0.583853 m within 1e-6 m
    middle == 1 within 1e-12
    loose  == 0 within 1e-12
  }
}
";

#[test]
fn undirected_springs_pull_both_ends() {
    let prog = program(PAIR);
    for case in &prog.doc.runs {
        let report = run_case(&prog, case).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        for r in &report.results {
            assert!(r.pass, "{} {}: {}", case.name, r.id, r.message);
        }
    }
    // A second spring joins the middle ball, written the other way round.
    let mut i = Interactive::new(&prog, "Springs", Config::until(2.0)).unwrap_or_else(|d| panic!("{d:?}"));
    i.request("join", None).unwrap();
    let k = |n: &str| i.cm.index[&format!("Pair.{n}")];
    let at = i.session.current.state_at(0.0);
    assert_eq!((at[k("middle")].num(), at[k("loose")].num()), (2.0, 1.0));
}

#[test]
fn printing_keeps_undirected_and_the_methods() {
    let text = prismal_syntax::format::format(&prismal_syntax::compile(PAIR).unwrap().doc);
    assert!(text.contains("  undirected relation Spring(a in balls, b in balls) {"), "{text}");
    assert!(text.contains("s.other(o).pos - o.pos"), "{text}");
    assert!(text.contains("for s in springs if s.has(o)"), "{text}");
    let again = prismal_syntax::compile(&text).unwrap().doc;
    assert_eq!(again.models, prismal_syntax::compile(PAIR).unwrap().doc.models);
}

fn codes(src: &str) -> Vec<String> {
    match prismal_syntax::compile(src) {
        Ok(c) => match prismal_present::Program::new(c.doc) {
            Ok(_) => vec![],
            Err(ds) => ds.iter().map(|d| format!("{} {}", d.code, d.message)).collect(),
        },
        Err(ds) => ds.iter().map(|d| format!("{} {}", d.code, d.message)).collect(),
    }
}

#[test]
fn undirected_mistakes() {
    // The endpoints of an undirected relation are not read by role in the model.
    let by_role = PAIR.replace("middle: Real   = count(s for s in springs if s.has(balls[2]))", "middle: Real   = count(s for s in springs if s.a == balls[2])");
    let c = codes(&by_role);
    assert!(c.iter().any(|m| m.starts_with("MK-E26") && m.contains("have no order")), "{c:?}");
    // An undirected relation has its two endpoints in one collection.
    let apart = "
model M {
  object Ball { param { m: Mass = 1 kg } }
  undirected relation Link(a in left, b in right) { }
  parts {
    left:  Ball[2]
    right: Ball[2]
    links: Link[1, max 2] { a = left[1]; b = right[1] }
  }
}
";
    let c = codes(apart);
    assert!(c.iter().any(|m| m.starts_with("MK-E26") && m.contains("one collection")), "{c:?}");
}
