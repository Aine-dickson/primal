//! Contained objects and fixed collections (MK sections 7 and 8, D-055): elaboration to a
//! flat model (identities, names, overrides, connections, aggregates, container flows over
//! members), presentations repeated per member, printing, and diagnostics. Also the sign
//! reference of a guard after a bounce on a raised floor (RC-7.2a, D-056).

mod common;
use common::*;
use prismal_ir::elaborate;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_present::Program;
use prismal_runtime::Config;

const DROPS: &str = "
space Plane = euclidean(2)

model Drops in Plane {
  object Ball {
    param { r: Length = 0.1 m }
    input { g: Acceleration = 9.81 m/s^2 }
    state {
      pos: Point            = origin + (0 m, 1 m)
      vel: Vector<Velocity> = 0
    }
    discrete { resting: Boolean = false }
    flow {
      der(pos) = vel
      der(vel) = if resting then 0 else (0 m/s^2, -g)
    }
    event bounce on falling(pos.y - r) {
      set vel = (vel.x, -0.8 * vel.y)
    } zeno settle {
      set vel = 0
      set pos = origin + (pos.x, r)
      set resting = true
    }
  }
  param { g: Acceleration = 9.81 m/s^2 }
  parts {
    row: Ball[3] {
      pos = origin + (index * 1 m, index * 1 m)
      g = g
    }
    moon: Ball { g = 1.62 m/s^2 }
  }
  derived {
    highest: Length  = max(b.pos.y for b in row)
    total:   Length  = sum(b.pos.y for b in row)
    n:       Real    = count(row)
    landed:  Real    = count(b for b in row if b.resting)
    any_low: Boolean = any(b.pos.y < 0.5 m for b in row)
  }
}

presentation DropsView for Drops {
  view scene: spatial(Plane, scale: 1 m -> 60 px, y: up) {
    axes
    for b in row { marker(b.pos) as ball }
    marker(moon.pos) as moon_ball
  }
  observe {
    y2     = row[2].pos.y live
    top    = highest live
    tot    = total live
    count  = n live
    moon_y = moon.pos.y live
    energy = sum(0.5 * b.vel.y^2 / (1 m^2/s^2) for b in row) live
    rested = landed live
  }
}

run short of Drops with DropsView {
  until t0 + 0.2 s
  expect {
    y2     == 1.8038 m within 1e-9 m
    top    == 2.8038 m within 1e-9 m
    tot    == 5.4114 m within 1e-9 m
    count  == 3 within 1e-12
    moon_y == 0.9676 m within 1e-9 m
    energy == 5.774166 within 1e-9
  }
}

run long of Drops with DropsView {
  until t0 + 10 s
  expect {
    y2     == 0.1 m within 1e-9 m
    rested == 3 within 1e-12
  }
}
";

const STARS: &str = "
space Plane = euclidean(2)

model Stars in Plane {
  object Body {
    param { m: Mass = 1e24 kg }
    state {
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  const { G: Quantity<L^3/M/T^2> = 6.674e-11 m^3/kg/s^2 }
  parts {
    bodies: Body[3] {
      pos = origin + (index * 1e7 m, 0 m)
      vel = (0 m/s, index * 100 m/s)
    }
  }
  flow {
    for b in bodies {
      der(b.vel) = sum(G * o.m * (o.pos - b.pos) / |o.pos - b.pos|^3 for o in bodies if o != b)
    }
  }
  derived {
    momentum: Vector<Momentum> = sum(b.m * b.vel for b in bodies)
  }
}

presentation Sky for Stars {
  view sky: spatial(Plane, scale: 1e6 m -> 10 px, y: up) {
    for b in bodies {
      marker(b.pos) as star
      trace(b.pos every 100 s)
    }
  }
  observe {
    px = momentum.x live
    py = momentum.y live
  }
  timeline {
    scene s {
      beat a { run rate 100; wait 2 s }
      beat b { hold; highlight star }
    }
  }
}

run orbit of Stars {
  until t0 + 5000 s
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
fn members_and_aggregates() {
    cases(DROPS);
}

#[test]
fn elaborated_identities_and_names() {
    let doc = prismal_syntax::compile(DROPS).unwrap().doc;
    let m = &doc.models[0];
    // The IR keeps the structure: one object type, two parts, aggregates in derived bindings.
    assert_eq!(m.objects.len(), 1);
    assert_eq!(m.objects[0].id, "Drops.Ball");
    assert_eq!(m.parts.iter().map(|p| (p.name.as_str(), p.count)).collect::<Vec<_>>(), [("row", Some(3)), ("moon", None)]);
    let flat = elaborate::model(m).unwrap();
    let b = flat.binding("Drops.Ball.pos@row[2]").expect("member binding");
    assert_eq!(b.name, "row[2].pos");
    // An override is read in the container's scope with `index` the member's number.
    let init = serde_json::to_string(b.init.as_ref().unwrap()).unwrap();
    assert!(init.contains("\"num\":2.0"), "{init}");
    // A connected input follows the container's expression; an unconnected one keeps its
    // default; `moon`'s is a constant connection.
    let g2 = flat.binding("Drops.Ball.g@row[2]").unwrap();
    assert_eq!(g2.role, prismal_ir::Role::Derived);
    assert_eq!(g2.def, Some(prismal_ir::Expr::Ref { r#ref: "Drops.g".into() }));
    assert_eq!(flat.binding("Drops.Ball.g@moon").unwrap().role, prismal_ir::Role::Derived);
    // Every member has its own event, named by its path.
    let names: Vec<&str> = flat.events.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["row[1].bounce", "row[2].bounce", "row[3].bounce", "moon.bounce"]);
    assert_eq!(elaborate::declaration("Drops.Ball.event.bounce@row[1]"), "Drops.Ball.event.bounce");
    // `max` over the row is a chain of `max` of the members' heights.
    let hi = flat.binding("Drops.highest").unwrap().def.clone().unwrap();
    assert_eq!(hi.refs(), ["Drops.Ball.pos@row[1]", "Drops.Ball.pos@row[2]", "Drops.Ball.pos@row[3]"]);
    // A model without parts is left as it is.
    let plain = prismal_syntax::compile("model P { param { a: Real = 1 } }").unwrap().doc;
    assert_eq!(elaborate::model(&plain.models[0]).unwrap(), plain.models[0]);
}

#[test]
fn container_flows_over_members() {
    let prog = program(STARS);
    let cm = prog.model("Stars");
    // One flow per member for `der(b.vel)`, reading the other members only.
    let flows: Vec<&prismal_ir::Flow> = cm.ir.flows.iter().filter(|f| f.target.starts_with("Stars.Body.vel@")).collect();
    assert_eq!(flows.len(), 3);
    let reads = flows[0].expr.refs();
    assert!(!reads.contains(&"Stars.Body.m@bodies[1]".to_string()) && reads.contains(&"Stars.Body.m@bodies[2]".to_string()), "{reads:?}");
    // Pairwise forces are opposite: total momentum is conserved (RK methods keep linear
    // invariants).
    let pres = &prog.doc.presentations[0];
    let mut i = Interactive::new(&prog, &pres.name, Config::until(5000.0)).unwrap_or_else(|d| panic!("{d:?}"));
    let num = |i: &Interactive, name: &str| match i.observe(name).unwrap() {
        prismal_present::data::Data::Value(v) => v.num(),
        d => panic!("{d:?}"),
    };
    let p0 = num(&i, "py");
    i.seek(5000.0);
    let (px, py) = (num(&i, "px"), num(&i, "py"));
    assert!(px.abs() < 1e-12 * p0 && (py - p0).abs() < 1e-12 * p0, "momentum drifted: ({px}, {py}) from {p0}");
    assert!((p0 - 6e26).abs() < 1e-12 * 6e26, "{p0}");
}

#[test]
fn representations_per_member() {
    let prog = program(STARS);
    let pres = &prog.doc.presentations[0];
    let reps = &pres.views[0].representations;
    let names: Vec<String> = reps.iter().map(|r| r.name.clone().unwrap_or(r.kind.clone())).collect();
    // Each representation of the block is repeated over the members in turn.
    assert_eq!(names, ["star[1]", "star[2]", "star[3]", "trace", "trace", "trace"]);
    let i = Interactive::new(&prog, &pres.name, Config::until(10.0)).unwrap();
    let f = i.frame();
    assert_eq!(f.rep("Sky.view.sky.star[2]").unwrap().text, "star[2] at x = 2e7 m, y = 0 m");
    // `highlight star` highlights every member of the family.
    let t = pres.timeline.as_ref().unwrap();
    let highlights = t.scenes[0].beats[1].actions.iter().filter(|a| matches!(a, prismal_ir::present::Action::Highlight { .. })).count();
    assert_eq!(highlights, 3);
}

#[test]
fn printing_round_trip() {
    for src in [DROPS, STARS] {
        let doc = prismal_syntax::compile(src).unwrap().doc;
        let printed = prismal_syntax::format::format(&doc);
        assert_eq!(prismal_syntax::compile(&printed).unwrap_or_else(|d| panic!("{d:?}\n{printed}")).doc, doc, "{printed}");
        assert_eq!(prismal_syntax::format::format(&doc), printed);
    }
    let printed = prismal_syntax::format::format(&prismal_syntax::compile(DROPS).unwrap().doc);
    for line in ["  object Ball {", "    row: Ball[3] {", "      pos = origin + (index * 1 m, index * 1 m)", "    moon: Ball { g = 1.62 m/s^2 }", "max(b.pos.y for b in row)", "count(b for b in row if b.resting)", "for b in row { marker(b.pos) as ball }"] {
        assert!(printed.contains(line), "{line} in\n{printed}");
    }
    let printed = prismal_syntax::format::format(&prismal_syntax::compile(STARS).unwrap().doc);
    assert!(printed.contains("    for b in bodies {\n      der(b.vel) = sum("), "{printed}");
}

#[test]
fn diagnostics() {
    let codes = |edit: &str, with: &str| -> Vec<String> {
        let bad = DROPS.replace(edit, with);
        assert_ne!(bad, DROPS, "{edit}");
        match prismal_syntax::compile(&bad) {
            Err(ds) => ds.iter().map(|d| d.code.to_string()).collect(),
            Ok(c) => Program::new(c.doc).err().map(|ds| ds.iter().map(|d| d.code.to_string()).collect()).unwrap_or_default(),
        }
    };
    assert_eq!(codes("max(b.pos.y for b in row)", "max(b.height for b in row)"), ["SX-E03"], "no such binding");
    assert_eq!(codes("row[2].pos.y live", "row[4].pos.y live"), ["MK-E26"], "no such member");
    assert_eq!(codes("row[2].pos.y live", "row.pos.y live"), ["SX-E08"], "a collection is not one object");
    assert_eq!(codes("moon.pos.y live", "moon[1].pos.y live"), ["SX-E08"], "one object is not a collection");
    assert_eq!(codes("max(b.pos.y for b in row)", "max(b.pos.y for b in row if b.resting)"), ["MK-E26"], "min and max filter members only");
    assert_eq!(codes("moon: Ball { g = 1.62 m/s^2 }", "moon: Ball { speed = 1 m/s }"), ["SX-E03"], "override of no binding");
    assert_eq!(codes("moon: Ball { g = 1.62 m/s^2 }", "moon: Ball { g = index * 1 m/s^2 }"), ["SX-E03"], "`index` only in a collection");
    assert_eq!(codes("moon: Ball { g = 1.62 m/s^2 }", "moon: Planet"), ["SX-E03"], "unknown object type");
    assert_eq!(codes("    total:   Length  = sum(b.pos.y for b in row)", "    total:   Length  = sum(b.pos for b in row)"), ["MK-E04"], "types are checked after elaboration: points do not add");
    // A type error inside an object type is reported once per member, at the declaration.
    let bad = DROPS.replace("der(pos) = vel", "der(pos) = vel * 2 s");
    let c = prismal_syntax::compile(&bad).unwrap();
    let ds = Program::new(c.doc).err().unwrap();
    assert!(ds.iter().all(|d| elaborate::declaration(&d.element) == "Drops.Ball.flow.1"), "{ds:?}");
    assert!(c.source_map.contains_key("Drops.Ball.flow.1"));
}

/// RC-7.2a (D-056): after a bounce on a floor that is not at zero, the ball's next landing
/// is found even when its whole excursion lies inside one step, so bounces accumulate and
/// the Zeno policy settles the ball instead of letting it fall through.
#[test]
fn bounces_on_a_raised_floor_settle() {
    let src = "
model Ball {
  param {
    g: Acceleration = 9.81 m/s^2
    r: Length       = 0.1 m
  }
  state {
    y: Length   = 1 m
    v: Velocity = 0 m/s
  }
  discrete { resting: Boolean = false }
  flow {
    der(y) = v
    der(v) = if resting then 0 else -g
  }
  event bounce on falling(y - r) {
    set v = -0.8 * v
  } zeno settle {
    set v = 0 m/s
    set y = r
    set resting = true
  }
}
presentation V for Ball {
  observe {
    height = y live
    rest   = resting live
  }
}
run settles of Ball with V {
  until t0 + 5 s
  expect {
    height == 0.1 m within 1e-12 m
    rest == true
  }
}
";
    cases(src);
}
