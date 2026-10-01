//! Collections without a declared limit (D-066): `Drop[max inf]` grows its working capacity
//! as runs need, in cases, lessons and sessions; the run is the run of the same program
//! with that capacity declared, bit for bit; printing keeps `max inf`.

mod common;
use common::*;
use prismal_present::expect::run_case;
use prismal_present::interact::Interactive;
use prismal_runtime::Config;

/// The guide's fountain, spraying every 0.25 s: 41 drops are made in 10 s, four or five in
/// the air at once.
fn fountain(limit: &str) -> String {
    format!(
        "
space Plane = euclidean(2)

model Fountain in Plane {{
  object Drop {{
    state {{
      pos: Point            = origin
      vel: Vector<Velocity> = 0
    }}
    flow {{
      der(pos) = vel
      der(vel) = (0 m/s^2, -9.81 m/s^2)
    }}
  }}
  param {{ speed: Velocity = 5 m/s }}
  parts {{ drops: Drop[max {limit}] }}
  event spray on every 0.25 s {{
    create drops {{ vel = (1 m/s, speed) }}
  }}
  event burst on request {{
    create drops {{ vel = (0 m/s, speed) }}
  }}
  for d in drops {{
    event land on falling(d.pos.y) {{ destroy d }}
  }}
  derived {{ flying: Real = count(drops) }}
}}

presentation Spray for Fountain {{
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {{
    for d in drops {{ marker(d.pos) as drop }}
  }}
  observe {{
    n     = flying at t0 + 9.95 s
    first = drops[1].pos.y at t0 + 9.95 s
  }}
}}

run hundred of Fountain with Spray {{
  until t0 + 10 s
  expect {{
    n     == 4 within 1e-12
    first == 0 m within 1e-9 m
  }}
}}
"
    )
}

#[test]
fn a_collection_without_a_limit_grows() {
    let prog = program(&fountain("inf"));
    let case = &prog.doc.runs[0];
    let report = run_case(&prog, case).unwrap();
    for r in &report.results {
        assert!(r.pass, "{}: {}", r.id, r.message);
    }
    // The run is that of the program with the capacity it grew to, declared: 8, 16, 32,
    // then 64 places for 41 drops.
    let bounded = program(&fountain("64"));
    let same = run_case(&bounded, &bounded.doc.runs[0]).unwrap();
    let (a, b) = (report.run.unwrap(), same.run.unwrap());
    assert_eq!(format!("{:?}", a.committed.last().unwrap().vals), format!("{:?}", b.committed.last().unwrap().vals));
    assert_eq!(a.log.len(), b.log.len());
    // A declared limit still stops the run.
    let small = program(&fountain("40"));
    let stopped = run_case(&small, &small.doc.runs[0]).unwrap();
    assert!(matches!(stopped.run.unwrap().status, prismal_runtime::RunStatus::Stopped(_)));
}

#[test]
fn sessions_grow_and_keep_their_actions() {
    let prog = program(&fountain("inf"));
    let mut i = Interactive::new(&prog, "Spray", Config::until(3.0)).unwrap_or_else(|d| panic!("{d:?}"));
    // Thirteen drops are sprayed in 3 s; requests at 2 s add twelve more, one at a time,
    // past the starting room for eight and then for sixteen.
    i.seek(2.0);
    for _ in 0..12 {
        i.request("burst", None).unwrap();
    }
    assert_eq!(i.session.log().len(), 12, "every request is kept across growth");
    let flying = i.cm.index["Fountain.flying"];
    let n = i.session.current.state_at(2.0)[flying].num();
    assert_eq!(n, 5.0 + 12.0, "five sprayed drops (1 s to 2 s) and twelve from the bursts in the air at 2 s");
    let drawn = i.frame().reps().filter(|r| r.name.as_deref().is_some_and(|x| x.starts_with("drop["))).count();
    assert_eq!(drawn, 17, "every drop in the air is drawn");
    assert!(matches!(i.session.current.status, prismal_runtime::RunStatus::Completed), "{:?}", i.session.current.status);
}

#[test]
fn printing_keeps_max_inf() {
    let doc = prismal_syntax::compile(&fountain("inf")).unwrap().doc;
    let text = prismal_syntax::format(&doc);
    assert!(text.contains("drops: Drop[max inf]"), "{text}");
    let part = &doc.models[0].parts[0];
    assert!(part.unbounded && part.capacity.is_none());
    let json = serde_json::to_string(part).unwrap();
    assert!(json.contains("\"unbounded\":true") && !json.contains("capacity"), "{json}");
}
