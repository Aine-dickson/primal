//! Collections whose membership changes during a run (MK-8.2, MK section 16, D-057):
//! `create` and `destroy` in handlers, events repeated per member, aggregates over the
//! members alive, representations drawn while their member is alive, the capacity, printing
//! and diagnostics.

mod common;
use common::*;
use prismal_ir::elaborate;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_present::Program;
use prismal_runtime::{Config, RunStatus};

const FOUNTAIN: &str = "
space Plane = euclidean(2)

model Fountain in Plane {
  object Drop {
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow {
      der(pos) = vel
      der(vel) = (0 m/s^2, -9.81 m/s^2)
    }
  }
  object Stone {
    state { pos: Point = origin }
  }
  param { speed: Velocity = 5 m/s }
  parts {
    drops: Drop[max 20]
    still: Stone[2, max 4] { pos = origin + (index * -1 m, 0 m) }
  }
  event spray on every 0.5 s {
    create drops { vel = (1 m/s, speed) }
  }
  event burst on request(h: Length) {
    create still { pos = origin + (0 m, h) }
    create still { pos = origin + (0 m, 2 * h) }
  }
  for d in drops {
    event land on falling(d.pos.y) { destroy d }
  }
  derived {
    flying:  Real   = count(drops)
    highest: Length = max(d.pos.y for d in drops) otherwise 0 m
    lowest:  Length = min(s.pos.y for s in still) otherwise 0 m
    climbing: Real  = count(d for d in drops if d.vel.y > 0 m/s)
  }
}

presentation Show for Fountain {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for d in drops { marker(d.pos) as drop }
    for s in still { marker(s.pos) as stone }
  }
  observe {
    n     = flying live
    top   = highest live
    up    = climbing live
    first = drops[1].pos.y live
    low   = lowest live
  }
}

run early of Fountain with Show {
  until t0 + 1.2 s
  expect {
    n     == 2 within 1e-12
    top   == 1.09655 m within 1e-9 m
    up    == 1 within 1e-12
    first == 0 m within 1e-9 m
    low   == 0 m within 1e-12 m
  }
}

run steady of Fountain with Show {
  until t0 + 9.9 s
  expect {
    n == 2 within 1e-12
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

#[test]
fn members_made_and_destroyed() {
    cases(FOUNTAIN);
}

#[test]
fn elaborated_members_and_liveness() {
    let doc = prismal_syntax::compile(FOUNTAIN).unwrap().doc;
    let m = &doc.models[0];
    assert_eq!(m.parts.iter().map(|p| (p.name.as_str(), p.count, p.capacity)).collect::<Vec<_>>(), [("drops", None, Some(20)), ("still", Some(2), Some(4))]);
    let flat = elaborate::model(m).unwrap();
    // One member per place in the collection, each with its liveness; the first `count`
    // members are alive from the start.
    assert!(flat.binding("Fountain.Drop.pos@drops[20]").is_some() && flat.binding("Fountain.Drop.pos@drops[21]").is_none());
    let alive = |id: &str| flat.binding(&elaborate::alive("Fountain.part.still", id)).unwrap().init.clone();
    assert_eq!(alive("still[2]"), Some(prismal_ir::Expr::Bool { bool: true }));
    assert_eq!(alive("still[3]"), Some(prismal_ir::Expr::Bool { bool: false }));
    assert!(flat.binding(&elaborate::alive("Fountain.part.drops", "drops[1]")).unwrap().private);
    // The event repeated per member: one per place, named by the member's number.
    let names: Vec<&str> = flat.events.iter().filter(|e| e.name.starts_with("land")).map(|e| e.name.as_str()).collect();
    assert_eq!(names.len(), 20);
    assert_eq!(names[1], "land[2]");
    let land2 = flat.events.iter().find(|e| e.id == "Fountain.event.land@drops[2]").unwrap();
    assert_eq!(land2.enable, Some(prismal_ir::Expr::Ref { r#ref: elaborate::alive("Fountain.part.drops", "drops[2]") }));
    assert_eq!(land2.handler, [prismal_ir::Op::Destroy { member: prismal_ir::Expr::Ref { r#ref: elaborate::alive("Fountain.part.drops", "drops[2]") } }]);
    // Two creates in one handler take the next two members; the count is set once.
    let burst = flat.events.iter().find(|e| e.name == "burst").unwrap();
    let ifs = burst.handler.iter().filter(|o| matches!(o, prismal_ir::Op::If { .. })).count();
    assert_eq!(ifs, 2 + 1, "members 3 and 4 for the first create, 4 for the second");
    assert!(matches!(burst.handler.last(), Some(prismal_ir::Op::Set { target, .. }) if target.binding == elaborate::created("Fountain.part.still", "")));
    // A capacity constraint bounds the members made in a run.
    assert!(flat.constraints.iter().any(|c| c.name == "drops.capacity" && c.policy == prismal_ir::Policy::Stop));
}

#[test]
fn frames_draw_members_alive() {
    let prog = program(FOUNTAIN);
    let mut i = Interactive::new(&prog, "Show", Config::until(12.0)).unwrap_or_else(|d| panic!("{d:?}"));
    let drops = |i: &Interactive| i.frame().reps().filter(|r| r.name.as_deref().is_some_and(|n| n.starts_with("drop["))).map(|r| r.name.clone().unwrap()).collect::<Vec<_>>();
    // At the start the first spray has made `drops[1]`.
    assert_eq!(drops(&i), ["drop[1]"]);
    i.seek(0.7);
    assert_eq!(drops(&i), ["drop[1]", "drop[2]"]);
    // `drops[1]` landed at 2 v / g = 1.0194 s and is no longer drawn.
    i.seek(1.2);
    assert_eq!(drops(&i), ["drop[2]", "drop[3]"]);
    let text = i.frame().rep("drop[2]").unwrap().text.clone();
    assert_eq!(text, "drop[2] at x = 0.7 m, y = 1.09655 m");
    let stones = i.frame().reps().filter(|r| r.name.as_deref().is_some_and(|n| n.starts_with("stone["))).count();
    assert_eq!(stones, 2);
}

#[test]
fn requests_create_and_the_capacity_is_kept() {
    let prog = program(FOUNTAIN);
    // `burst` makes two more stones at the instant of the request, from its payload.
    let mut i = Interactive::new(&prog, "Show", Config::until(12.0)).unwrap();
    i.seek(0.3);
    i.request("burst", Some(prismal_ir::build::num(1.5, "m"))).unwrap_or_else(|r| panic!("{r:?}"));
    let stones: Vec<String> = i.frame().reps().filter(|r| r.name.as_deref().is_some_and(|n| n.starts_with("stone["))).map(|r| r.text.clone()).collect();
    assert_eq!(stones, ["stone[1] at x = -1 m, y = 0 m", "stone[2] at x = -2 m, y = 0 m", "stone[3] at x = 0 m, y = 1.5 m", "stone[4] at x = 0 m, y = 3 m"]);
    // `still` holds at most four: a third burst stops the run at the capacity constraint.
    let _ = i.request("burst", Some(prismal_ir::build::num(1.0, "m")));
    match &i.session.current.status {
        RunStatus::Stopped(d) => assert!(d.message.contains("still.capacity"), "{}", d.message),
        s => panic!("expected the run to stop at the capacity, got {s:?}"),
    }
    // Twenty sprays fit in 9.5 s; the twenty-first, at 10 s, is one too many.
    let prog = program(&FOUNTAIN.replace("until t0 + 9.9 s", "until t0 + 10.2 s"));
    let report = run_case(&prog, &prog.doc.runs[1]).unwrap();
    match &report.run.unwrap().status {
        RunStatus::Stopped(d) => assert!(d.message.contains("drops.capacity") && (d.t - 10.0).abs() < 1e-9, "{d:?}"),
        s => panic!("expected the run to stop at the capacity, got {s:?}"),
    }
}

#[test]
fn printing_round_trip() {
    let doc = prismal_syntax::compile(FOUNTAIN).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert_eq!(prismal_syntax::compile(&printed).unwrap_or_else(|d| panic!("{d:?}\n{printed}")).doc, doc, "{printed}");
    for line in ["    drops: Drop[max 20]", "    still: Stone[2, max 4] { pos = origin + (index * (-1 m), 0 m) }", "create drops { vel = (1 m/s, speed) }", "  for d in drops {\n    event land on falling(d.pos.y) { destroy d }\n  }"] {
        assert!(printed.contains(line), "{line} in\n{printed}");
    }
}

#[test]
fn a_container_writes_its_members() {
    // A container's event per member may set the member's bindings (MK-7.10).
    let src = "
space Plane = euclidean(2)
model Box in Plane {
  object Ball {
    state {
      pos: Point            = origin + (0 m, 1 m)
      vel: Vector<Velocity> = (1 m/s, 0 m/s)
    }
    flow { der(pos) = vel }
  }
  parts { balls: Ball[2, max 2] { vel = (index * 1 m/s, 0 m/s) } }
  for b in balls {
    event wall on rising(b.pos.x - 1 m) { set b.vel.x = -b.vel.x } zeno stop
  }
}
presentation V for Box {
  observe {
    x1 = balls[1].pos.x live
    x2 = balls[2].pos.x live
  }
}
run back of Box with V {
  until t0 + 1.5 s
  expect {
    x1 == 0.5 m within 1e-9 m
    x2 == -1 m within 1e-9 m
  }
}
";
    cases(src);
}

/// MK-16.5: destroying a member and writing it in one transition is a conflict; destroying
/// it twice is not.
#[test]
fn destroyed_members_in_conflicts() {
    let at_landing = |handler: &str| {
        let prog = program(&FOUNTAIN.replace("{ destroy d }", handler));
        let report = run_case(&prog, &prog.doc.runs[0]).unwrap();
        report.run.unwrap().status
    };
    match at_landing("{ set d.vel = 0; destroy d }") {
        RunStatus::Stopped(d) => assert!(d.category == prismal_runtime::Category::Conflict && d.message.contains("drops[1].vel"), "{d:?}"),
        s => panic!("expected a conflict, got {s:?}"),
    }
    let prog = program(&FOUNTAIN.replace("  for d in drops {\n", "  for d in drops {\n    event again on falling(d.pos.y) { destroy d }\n"));
    let report = run_case(&prog, &prog.doc.runs[0]).unwrap();
    assert!(report.results.iter().all(|r| r.pass), "{:?}", report.failures());
}

#[test]
fn diagnostics() {
    let codes = |edit: &str, with: &str| -> Vec<String> {
        let bad = FOUNTAIN.replace(edit, with);
        assert_ne!(bad, FOUNTAIN, "{edit}");
        match prismal_syntax::compile(&bad) {
            Err(ds) => ds.iter().map(|d| d.code.to_string()).collect(),
            Ok(c) => Program::new(c.doc).err().map(|ds| ds.iter().map(|d| d.code.to_string()).collect()).unwrap_or_default(),
        }
    };
    assert_eq!(codes("drops: Drop[max 20]", "drops: Drop[20]"), ["MK-E26", "MK-E26"], "create and destroy need a collection declared with `max`");
    assert_eq!(codes("create drops { vel = (1 m/s, speed) }", "create drops { mass = 1 kg }"), ["SX-E03"], "no such binding");
    assert_eq!(codes("create drops { vel = (1 m/s, speed) }", "create drops { vel = speed }"), ["MK-E01"], "types are checked after elaboration");
    assert_eq!(codes("{ destroy d }", "{ destroy speed }"), ["SX-E08"], "destroy names a member");
    assert_eq!(codes("still: Stone[2, max 4]", "still: Stone[5, max 4]"), ["MK-E26"], "more starting members than places");
    assert_eq!(codes("still: Stone[2, max 4]", "still: Stone[2, 4]"), ["SX-E02"], "`max` is written");
    assert_eq!(codes("max(d.pos.y for d in drops) otherwise 0 m", "max(d.pos.y for d in drops if d.vel.y > 0 m/s) otherwise 0 m"), ["MK-E26"], "min and max filter members only");
}
