//! Members as payloads (D-059): an event requested for one member of a collection
//! (`on request(b in balls)`), several payloads (`on request(b in balls, j: ...)`), `set` on
//! the member chosen, members passed on by `emit`, refusals for members that are not alive,
//! replay, timelines, printing and diagnostics.

mod common;
use common::*;
use prismal_ir::build::num;
use prismal_ir::Expr;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_present::Program;
use prismal_runtime::{run, Action, Config, RunStatus};

const TABLE: &str = "
space Plane = euclidean(2)

model Table in Plane {
  object Ball {
    param { m: Mass = 1 kg }
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  relation Link(a in balls, b in balls) {
    param { k: Real = 1 }
  }
  parts {
    balls: Ball[3, max 4] { pos = origin + (index * 1 m, 0 m) }
    links: Link[1, max 2] {
      a = balls[1]
      b = balls[3]
    }
  }
  discrete {
    last: Length = 0 m
  }
  event remove on request(b in balls) { destroy b }
  event kick on request(b in balls, j: Vector<Momentum>) { set b.vel = b.vel + j / b.m }
  event halt on request(b in balls) if b.pos.x > 1.5 m { set b.vel.x = 0 m/s }
  event pass on request(b in balls) { emit mark(b) }
  event mark on request(b in balls)
  event marked on mark(c in balls) { set last = c.pos.x }
  event tug on request(s in links) { set s.b.vel = (0 m/s, 1 m/s) }
  event add on request { create balls { pos = origin + (4 m, 0 m) } }
  derived {
    n:     Real   = count(balls)
    tied:  Real   = count(links)
    x2:    Length = balls[2].pos.x
    y3:    Length = balls[3].pos.y
  }
}

presentation Show for Table {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for o in balls { marker(o.pos) as ball }
  }
  observe {
    n  = n at t0 + 1.5 s
    y3 = y3 at t0 + 1.5 s
  }
  timeline {
    scene only {
      beat first { run rate 1; wait 0.5 s }
      beat act {
        request remove(balls[2])
        request kick(balls[3], (0 kg*m/s, 2 kg*m/s))
        wait 1 s
      }
    }
  }
}

run lesson of Table with Show {
  expect {
    n  == 2 within 1e-12
    y3 == 2 m within 1e-9 m
  }
}
";

fn value(i: &Interactive, name: &str) -> f64 {
    let k = i.cm.index[&format!("Table.{name}")];
    i.session.current.state_at(i.t)[k].num()
}

fn momentum(x: f64, y: f64) -> Expr {
    Expr::Tuple { tuple: vec![num(x, "kg*m/s"), num(y, "kg*m/s")] }
}

fn session(prog: &Program) -> Interactive {
    Interactive::new(prog, "Show", Config::until(3.0)).unwrap_or_else(|d| panic!("{d:?}"))
}

#[test]
fn a_member_as_payload() {
    let prog = program(TABLE);
    let mut i = session(&prog);
    i.seek(0.5);
    // The payload is the member's number in its collection.
    i.request("remove", Some(num(2.0, ""))).unwrap();
    assert_eq!(value(&i, "n"), 2.0);
    // Removed, `balls[2]` cannot be requested again: the event is not enabled for it.
    let e = i.request("remove", Some(num(2.0, ""))).unwrap_err();
    assert!(e.message.contains("`balls[2]` is not alive"), "{}", e.message);
    // A member not made yet, and a number with no member, are refused too.
    let e = i.request("remove", Some(num(4.0, ""))).unwrap_err();
    assert!(e.message.contains("`balls[4]` is not alive"), "{}", e.message);
    let e = i.request("remove", Some(num(7.0, ""))).unwrap_err();
    assert!(e.message.contains("no member number 7"), "{}", e.message);
    // Made, the fourth ball can be removed.
    i.request("add", None).unwrap();
    assert_eq!(value(&i, "n"), 3.0);
    i.request("remove", Some(num(4.0, ""))).unwrap();
    assert_eq!(value(&i, "n"), 2.0);
    // Destroying a member through a payload disconnects its relations (MK-8.6).
    assert_eq!(value(&i, "tied"), 1.0);
    i.request("remove", Some(num(3.0, ""))).unwrap();
    assert_eq!(value(&i, "tied"), 0.0);
}

#[test]
fn several_payloads_and_set_on_the_member() {
    let prog = program(TABLE);
    let mut i = session(&prog);
    // `kick(balls[3], j)`: ball 3 moves up at 2 m/s; the others do not move.
    i.request("kick", Some(Expr::Tuple { tuple: vec![num(3.0, ""), momentum(0.0, 2.0)] })).unwrap();
    i.seek(1.0);
    assert!((value(&i, "y3") - 2.0).abs() < 1e-9, "{}", value(&i, "y3"));
    assert_eq!(value(&i, "x2"), 2.0);
    // A payload of the wrong shape is rejected.
    let e = i.request("kick", Some(num(3.0, ""))).unwrap_err();
    assert!(e.message.contains("payload"), "{}", e.message);
}

#[test]
fn conditions_emit_and_endpoints() {
    let prog = program(TABLE);
    let mut i = session(&prog);
    // The condition reads the member chosen: ball 1 is at x = 1 m, ball 2 at 2 m.
    let e = i.request("halt", Some(num(1.0, ""))).unwrap_err();
    assert!(e.message.contains("`halt` is not enabled now"), "{}", e.message);
    i.request("halt", Some(num(2.0, ""))).unwrap();
    // `emit mark(b)` passes the member on: `marked` reads ball 3's position.
    i.request("pass", Some(num(3.0, ""))).unwrap();
    assert_eq!(value(&i, "last"), 3.0);
    // `set s.b.vel` writes the ball at the endpoint of the relation chosen.
    i.request("tug", Some(num(1.0, ""))).unwrap();
    i.seek(1.0);
    assert!((value(&i, "y3") - 1.0).abs() < 1e-9, "{}", value(&i, "y3"));
    // A relation not made yet is refused.
    let e = i.request("tug", Some(num(2.0, ""))).unwrap_err();
    assert!(e.message.contains("`links[2]` is not alive"), "{}", e.message);
}

#[test]
fn replay_is_deterministic() {
    let prog = program(TABLE);
    let cm = prog.model("Table");
    let cfg = || {
        Config::until(2.0)
            .at(0.5, Action::RequestWith("Table.event.kick".into(), Expr::Tuple { tuple: vec![num(1.0, ""), momentum(1.0, 0.0)] }))
            .at(1.0, Action::RequestWith("Table.event.remove".into(), num(2.0, "")))
            .at(1.5, Action::RequestWith("Table.event.remove".into(), num(2.0, "")))
    };
    let (a, b) = (run(cm, cfg()), run(cm, cfg()));
    assert!(matches!(a.status, RunStatus::Completed), "{:?}", a.status);
    assert_eq!(a.log.iter().map(|e| (e.name.clone(), e.t, e.payload.clone())).collect::<Vec<_>>(), b.log.iter().map(|e| (e.name.clone(), e.t, e.payload.clone())).collect::<Vec<_>>());
    assert_eq!(a.state_at(2.0), b.state_at(2.0));
    // The log records the member's number; the second removal is rejected.
    assert_eq!(a.log.iter().map(|e| e.name.as_str()).collect::<Vec<_>>(), ["kick", "remove"]);
    assert_eq!(a.rejected.len(), 1);
    assert!(a.rejected[0].message.contains("`balls[2]` is not alive"), "{}", a.rejected[0].message);
}

#[test]
fn requests_in_a_timeline() {
    let prog = program(TABLE);
    for case in &prog.doc.runs {
        let report = run_case(&prog, case).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        for r in &report.results {
            assert!(r.pass, "{} {}: {}", case.name, r.id, r.message);
        }
    }
}

#[test]
fn elaborated_payloads() {
    let doc = prismal_syntax::compile(TABLE).unwrap().doc;
    let m = &doc.models[0];
    let kick = m.event_by_name("kick").unwrap().payload.clone().unwrap();
    assert_eq!(kick.name, "b, j");
    assert_eq!(kick.items[0].of.as_deref(), Some("Table.part.balls"));
    assert!(kick.items[1].of.is_none());
    let flat = prismal_ir::elaborate::model(m).unwrap();
    let remove = flat.event_by_name("remove").unwrap();
    assert_eq!(remove.payload.as_ref().unwrap().members.as_deref(), Some("balls"));
    // Enabled for a member alive; the member's number picks its liveness.
    let json = serde_json::to_string(remove.enable.as_ref().unwrap()).unwrap();
    assert!(json.contains("\"pick\":{\"payload\":\"Table.event.remove\"}"), "{json}");
    assert!(json.contains("Table.part.balls.alive@balls[4]"), "{json}");
    // `destroy b` destroys the member whose number is the payload, one conditional per member.
    assert_eq!(remove.handler.iter().filter(|o| matches!(o, prismal_ir::Op::If { .. })).count() >= 4, true);
}

#[test]
fn printing_round_trip() {
    let doc = prismal_syntax::compile(TABLE).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert_eq!(prismal_syntax::compile(&printed).unwrap_or_else(|d| panic!("{d:?}\n{printed}")).doc, doc, "{printed}");
    for line in [
        "event remove on request(b in balls) { destroy b }",
        "event kick on request(b in balls, j: Vector<Momentum>) { set b.vel = b.vel + j / b.m }",
        "event marked on mark(c in balls) { set last = c.pos.x }",
        "emit mark(b)",
        "request kick(balls[3], (0 kg*m/s, 2 kg*m/s))",
        "request remove(balls[2])",
    ] {
        assert!(printed.contains(line), "{line} in\n{printed}");
    }
}

#[test]
fn diagnostics() {
    let codes = |edit: &str, with: &str| -> Vec<String> {
        let bad = TABLE.replace(edit, with);
        assert_ne!(bad, TABLE, "{edit}");
        match prismal_syntax::compile(&bad) {
            Err(ds) => ds.iter().map(|d| d.code.to_string()).collect(),
            Ok(c) => Program::new(c.doc).err().map(|ds| ds.iter().map(|d| d.code.to_string()).collect()).unwrap_or_default(),
        }
    };
    assert_eq!(codes("on request(b in balls) { destroy b }", "on request(b in rocks) { destroy b }"), ["SX-E03"], "unknown collection");
    assert_eq!(codes("request remove(balls[2])", "request remove(2)"), ["MK-E26"], "a member payload is given a member");
    assert_eq!(codes("request remove(balls[2])", "request remove(links[1])"), ["MK-E26"], "a member of the collection declared");
    assert_eq!(codes("event marked on mark(c in balls) { set last = c.pos.x }", "event marked on mark(c in links) { set last = 0 m }"), ["MK-E24"], "followers receive members of the same collection");
    assert_eq!(codes("{ emit mark(b) }", "{ emit mark(b.pos) }"), ["MK-E26"], "emit passes a member");
    assert_eq!(codes("{ set last = c.pos.x }", "{ set last = c }"), ["MK-E26"], "a member is not a value");
    assert_eq!(codes("request kick(balls[3], (0 kg*m/s, 2 kg*m/s))", "request kick(balls[3])"), ["MK-E26"], "two payloads");
    assert_eq!(codes("event remove on request(b in balls)", "event remove on request(b in balls, b: Real)"), ["SX-E09"], "one name per payload");
}
