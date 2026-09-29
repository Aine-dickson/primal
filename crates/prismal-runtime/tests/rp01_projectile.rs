//! RP-01 Projectile, no drag (docs/spec/reference-programs/RP-01-projectile.md).

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_ir::*;
use prismal_runtime::Config;

const P: &str = "Projectile";

fn p(n: &str) -> String {
    id(P, n)
}

#[test]
fn case_a_45deg() {
    let cm = compile(&projectile(false));
    let run = run_checked(&cm, Config::until(5.0));
    let t_land = run.on("landed", &elapsed(), None);
    let range = run.on("landed", &comp(r(&p("pos")), 0), None);
    let y_land = run.on("landed", &comp(r(&p("pos")), 1), None);
    let t_apex = run.on("apex", &elapsed(), None);
    let h_apex = run.on("apex", &comp(r(&p("pos")), 1), None);

    assert_close(t_land[0].num(), 2.88320807823261, 1e-9, "RP-01.E1 t_land");
    assert_close(range[0].num(), 40.7747196738022, 1e-8, "RP-01.E2 range");
    let y = y_land[0].num();
    assert!((-1e-8..=0.0).contains(&y), "RP-01.E3 y_land = {y:e}");
    assert_close(t_apex[0].num(), 1.44160403911630, 1e-9, "RP-01.E4 t_apex");
    assert_close(h_apex[0].num(), 10.1936799184506, 1e-8, "RP-01.E5 h_apex");

    // RP-01.E6: at rest after landing, bit for bit.
    let at_rest = run.at(5.0, &r(&p("pos")));
    let landed_pos = &run.on("landed", &r(&p("pos")), None)[0];
    assert!(at_rest == *landed_pos, "RP-01.E6 at_rest {at_rest} != {landed_pos}");

    // RP-01.E7: exactly apex then landed, nothing at t0.
    let names: Vec<&str> = run.log.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, vec!["apex", "landed"], "RP-01.E7 events");
    assert!(run.log.iter().all(|l| l.t > 0.0));
}

#[test]
fn case_b_60deg() {
    let cm = compile(&projectile(false));
    let run = run_checked(&cm, Config::until(5.0).param(&p("angle"), num(60.0, "deg")));
    assert_close(run.on("landed", &elapsed(), None)[0].num(), 3.53119430697019, 1e-9, "RP-01.E8 t_land");
    assert_close(run.on("landed", &comp(r(&p("pos")), 0), None)[0].num(), 35.3119430697019, 1e-8, "RP-01.E9 range");
}

fn event_mut<'a>(m: &'a mut Model, name: &str) -> &'a mut Event {
    m.events.iter_mut().find(|e| e.name == name).unwrap()
}

#[test]
fn diagnostic_variants() {
    // D1: the landing trigger stored as a level condition.
    let mut m = projectile(false);
    event_mut(&mut m, "landed").trigger = Trigger::Level { cond: le(comp(r(&p("pos")), 1), num(0.0, "m")) };
    assert!(codes(&m).contains(&"MK-E13"), "RP-01.D1 {:?}", codes(&m));

    // D2: a bare non-zero literal in a dimensioned vector.
    let mut b = ModelBuilder::from_model(projectile(false));
    b.contribute(&p("vel"), tuple(vec![lit(1.0), -r(&p("g"))]));
    let c = codes(&b.finish());
    assert!(c.contains(&"MK-E03"), "RP-01.D2 {c:?}");

    // D3: a defining flow next to contributions.
    let mut b = ModelBuilder::from_model(projectile(false));
    b.flow(&p("vel"), tuple(vec![lit(0.0), -r(&p("g"))]));
    let c = codes(&b.finish());
    assert!(c.contains(&"MK-E10"), "RP-01.D3 {c:?}");

    // D4: a flow condition on continuous state.
    let mut m = projectile(false);
    m.flows[0].expr = ite(gt(comp(r(&p("pos")), 1), num(0.0, "m")), r(&p("vel")), lit(0.0));
    let c = codes(&m);
    assert!(c.contains(&"MK-E12"), "RP-01.D4 {c:?}");

    // D5: a handler that sets a parameter.
    let mut m = projectile(false);
    event_mut(&mut m, "landed").handler.push(set(&p("g"), num(0.0, "m/s^2")));
    let c = codes(&m);
    assert!(c.contains(&"MK-E08"), "RP-01.D5 {c:?}");
}

#[test]
fn ir_json_round_trip() {
    let m = projectile(true);
    let doc = Document::new(vec![plane()], vec![m]);
    let json = doc.to_json();
    let back = Document::from_json(&json).unwrap();
    assert_eq!(doc, back, "IR survives a JSON round trip (D-037)");
    // The round-tripped model runs to the same result.
    let a = compile(&doc.models[0]);
    let b = compile(&back.models[0]);
    let ra = prismal_runtime::run(&a, Config::until(5.0));
    let rb = prismal_runtime::run(&b, Config::until(5.0));
    assert!(ra.committed.last().unwrap().vals == rb.committed.last().unwrap().vals);
}
