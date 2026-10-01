//! Relations (MK-8.5 to MK-8.7, D-058): relation types with endpoints in collections,
//! relation sets with a capacity, `connect` and `disconnect`, endpoints read and compared,
//! relations disconnected when an endpoint is destroyed, events per relation, printing and
//! diagnostics.

mod common;
use common::*;
use prismal_ir::elaborate;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_present::Program;
use prismal_runtime::Config;

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
  relation Spring(a in balls, b in balls) {
    param {
      k:    Quantity<M/T^2> = 2 N/m
      rest: Length          = 1 m
    }
    derived {
      stretch: Length        = |b.pos - a.pos| - rest
      pull:    Vector<Force> = k * stretch * (b.pos - a.pos) / |b.pos - a.pos|
    }
  }
  parts {
    balls: Ball[2, max 3] { pos = origin + ((2 * index - 3) * 1 m, 0 m) }
    springs: Spring[1, max 4] {
      a = balls[1]
      b = balls[2]
    }
  }
  flow {
    for o in balls {
      der(o.vel) = (sum(s.pull for s in springs if s.a == o) - sum(s.pull for s in springs if s.b == o)) / o.m
    }
  }
  event cut on request { disconnect springs[1] }
  event add on request { create balls { pos = origin + (0 m, 2 m) } }
  event join on request { connect springs(balls[1], balls[3]) { k = 4 N/m } }
  event drop on request { destroy balls[2] }
  derived {
    links: Real   = count(springs)
    gap:   Length = balls[2].pos.x - balls[1].pos.x
  }
}

presentation Springs for Pair {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for o in balls { marker(o.pos) as ball }
    for s in springs { segment(s.a.pos, s.b.pos) as spring }
  }
  observe {
    gap   = gap live
    links = links live
  }
}

run oscillates of Pair with Springs {
  until t0 + 1 s
  expect {
    gap   == 0.583853 m within 1e-6 m
    links == 1 within 1e-12
  }
}
";

fn cases(src: &str) {
    let prog = program(src);
    for case in &prog.doc.runs {
        let report = run_case(&prog, case).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        for r in &report.results {
            assert!(r.pass, "{} {}: {}", case.name, r.id, r.message);
        }
    }
}

/// A binding of the model at the instant shown.
fn num(i: &Interactive, name: &str) -> f64 {
    let k = i.cm.index[&format!("Pair.{name}")];
    i.session.current.state_at(i.t)[k].num()
}

fn springs(i: &Interactive) -> Vec<String> {
    i.frame().reps().filter(|r| r.name.as_deref().is_some_and(|n| n.starts_with("spring["))).map(|r| r.name.clone().unwrap()).collect()
}

/// Two balls 2 m apart on a spring of rest length 1 m: their gap is `1 + cos(ω t)` with
/// `ω = sqrt(2 k / m) = 2 rad/s`.
#[test]
fn a_spring_between_members() {
    cases(PAIR);
}

#[test]
fn elaborated_endpoints() {
    let doc = prismal_syntax::compile(PAIR).unwrap().doc;
    let m = &doc.models[0];
    let spring = m.objects.iter().find(|o| o.name == "Spring").unwrap();
    assert_eq!(spring.ends.iter().map(|e| (e.name.as_str(), e.over.as_str())).collect::<Vec<_>>(), [("a", "Pair.part.balls"), ("b", "Pair.part.balls")]);
    let flat = elaborate::model(m).unwrap();
    // An endpoint is the number of its member, from the part's overrides.
    let a1 = flat.binding("Pair.Spring.a@springs[1]").unwrap();
    assert_eq!(a1.init, Some(prismal_ir::Expr::Num { num: 1.0, unit: None }));
    assert!(a1.private);
    // The endpoint's position is picked among the balls by that number.
    let stretch = flat.binding("Pair.Spring.stretch@springs[2]").unwrap();
    let json = serde_json::to_string(stretch.def.as_ref().unwrap()).unwrap();
    assert!(json.contains("\"pick\":{\"ref\":\"Pair.Spring.b@springs[2]\"}"), "{json}");
    // A relation not alive has no derived values.
    assert_eq!(stretch.when, Some(prismal_ir::Expr::Ref { r#ref: elaborate::alive("Pair.part.springs", "springs[2]") }));
}

#[test]
fn connect_disconnect_and_destroy() {
    let prog = program(PAIR);
    let mut i = Interactive::new(&prog, "Springs", Config::until(3.0)).unwrap_or_else(|d| panic!("{d:?}"));
    assert_eq!(springs(&i), ["spring[1]"]);
    // Cut at 1 s: the balls keep their velocities, the gap closing at 2 sin 2 m/s.
    i.seek(1.0);
    i.request("cut", None).unwrap();
    i.seek(2.0);
    assert!((num(&i, "gap") - (1.0 + 2f64.cos() - 2.0 * 2f64.sin())).abs() < 1e-5, "{}", num(&i, "gap"));
    assert_eq!(num(&i, "links"), 0.0);
    assert!(springs(&i).is_empty());

    // A third ball, a spring to it, then the second ball destroyed: its spring goes with it.
    i.reset();
    i.seek(0.5);
    i.request("add", None).unwrap();
    i.request("join", None).unwrap();
    assert_eq!(num(&i, "links"), 2.0);
    assert_eq!(springs(&i), ["spring[1]", "spring[2]"]);
    i.request("drop", None).unwrap();
    assert_eq!(num(&i, "links"), 1.0);
    assert_eq!(springs(&i), ["spring[2]"]);
    assert!(matches!(i.frame().rep("spring[2]").unwrap().shape, prismal_present::frame::Shape::Segment { .. }));
}

/// An event per relation: a thread that snaps when stretched by 1 m, between balls moving
/// apart at 2 m/s relative speed from their rest length.
#[test]
fn relations_snap() {
    let src = "
space Plane = euclidean(2)
model Threads in Plane {
  object Ball {
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  relation Thread(a in balls, b in balls) {
    derived { stretch: Length = |b.pos - a.pos| - 1 m }
  }
  parts {
    balls: Ball[2, max 2] {
      pos = origin + ((index - 1.5) * 1 m, 0 m)
      vel = ((2 * index - 3) * 1 m/s, 0 m/s)
    }
    threads: Thread[1, max 1] {
      a = balls[1]
      b = balls[2]
    }
  }
  for s in threads {
    event snap on rising(s.stretch - 1 m) { disconnect s }
  }
  derived { held: Real = count(threads) }
}
presentation V for Threads {
  observe {
    early = held at t0 + 0.4 s
    late  = held at t0 + 0.6 s
  }
}
run snaps of Threads with V {
  until t0 + 1 s
  expect {
    early == 1 within 1e-12
    late  == 0 within 1e-12
  }
}
";
    cases(src);
}

#[test]
fn printing_round_trip() {
    let doc = prismal_syntax::compile(PAIR).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert_eq!(prismal_syntax::compile(&printed).unwrap_or_else(|d| panic!("{d:?}\n{printed}")).doc, doc, "{printed}");
    for line in ["  relation Spring(a in balls, b in balls) {", "      stretch: Length        = |b.pos - a.pos| - rest", "connect springs(balls[1], balls[3]) { k = 4 N/m }", "disconnect springs[1]", "sum(s.pull for s in springs if s.a == o)", "for s in springs { segment(s.a.pos, s.b.pos) as spring }"] {
        assert!(printed.contains(line), "{line} in\n{printed}");
    }
}

#[test]
fn diagnostics() {
    let codes = |edit: &str, with: &str| -> Vec<String> {
        let bad = PAIR.replace(edit, with);
        assert_ne!(bad, PAIR, "{edit}");
        match prismal_syntax::compile(&bad) {
            Err(ds) => ds.iter().map(|d| d.code.to_string()).collect(),
            Ok(c) => Program::new(c.doc).err().map(|ds| ds.iter().map(|d| d.code.to_string()).collect()).unwrap_or_default(),
        }
    };
    assert_eq!(codes("relation Spring(a in balls, b in balls)", "relation Spring(a in balls, b in rocks)"), ["SX-E03"], "unknown collection");
    assert_eq!(codes("{ disconnect springs[1] }", "{ destroy springs[1] }"), ["MK-E26"], "a relation is disconnected");
    assert_eq!(codes("{ destroy balls[2] }", "{ disconnect balls[2] }"), ["MK-E26"], "an object is destroyed");
    assert_eq!(codes("connect springs(balls[1], balls[3])", "connect springs(balls[1])"), ["MK-E26"], "two endpoints");
    assert_eq!(codes("connect springs(balls[1], balls[3])", "create springs"), ["MK-E26"], "relations are connected");
    assert_eq!(codes("springs: Spring[1, max 4]", "springs: Spring[1]"), ["MK-E26", "MK-E26", "MK-E26"], "connect, disconnect and destroy need capacities");
    assert_eq!(codes("gap:   Length = balls[2].pos.x - balls[1].pos.x", "gap:   Length = springs[1].c.pos.x"), ["SX-E03"], "no such endpoint");
    assert_eq!(codes("      b = balls[2]\n", ""), ["MK-E26"], "starting relations name their endpoints");
}
