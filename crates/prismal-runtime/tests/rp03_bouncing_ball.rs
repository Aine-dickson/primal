//! RP-03 Bouncing ball (docs/spec/reference-programs/RP-03-bouncing-ball.md).

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_ir::*;
use prismal_runtime::{Category, Config, RunStatus};

fn b(n: &str) -> String {
    id("BouncingBall", n)
}

#[test]
fn case_a_settle() {
    let cm = compile(&bouncing_ball(settle()));
    let run = run_checked(&cm, Config::until(6.0));
    let bounces = run.times("bounce");
    let expected = [
        (1, 0.451523640985731),
        (2, 1.17396146656290),
        (3, 1.75191172702464),
        (4, 2.21427193539402),
        (5, 2.58416010208954),
        (10, 3.57889295102044),
    ];
    for (n, t) in expected {
        assert_close(bounces[n - 1], t, 1e-8, &format!("RP-03 bounce_times[{n}]"));
    }
    let tops = run.on("top", &r(&b("y")), None);
    for (i, h) in [0.64, 0.4096, 0.262144].iter().enumerate() {
        assert_close(tops[i].num(), *h, 1e-8, &format!("RP-03.E7 top_heights[{}]", i + 1));
    }
    // E8: no `top` at t0; the first is after the first bounce.
    assert!(run.times("top")[0] > bounces[0], "RP-03.E8");

    // E9, E10: exactly one Zeno entry, at bounce 61 to 65, before t_inf and within 1e-4 s of it.
    let zeno: Vec<_> = run.log.iter().filter(|l| l.zeno).collect();
    assert_eq!(zeno.len(), 1, "RP-03.E9 zeno entries");
    let nth = run.log.iter().filter(|l| l.name == "bounce").position(|l| l.zeno).unwrap() + 1;
    assert!((61..=65).contains(&nth), "RP-03.E9 zeno at bounce {nth}");
    let t_inf = 4.06371276887158;
    assert!(zeno[0].t < t_inf && t_inf - zeno[0].t < 1e-4, "RP-03.E10 zeno at {}", zeno[0].t);

    // E11: final state exactly at rest.
    let fin = run.state_at(6.0);
    assert!(fin[cm.idx(&b("y"))].num() == 0.0 && fin[cm.idx(&b("v"))].num() == 0.0 && fin[cm.idx(&b("resting"))].boolean(), "RP-03.E11");

    // E12: no event after the settle.
    assert!(run.log.iter().all(|l| l.t <= zeno[0].t), "RP-03.E12 late events");
    assert_eq!(run.status, RunStatus::Completed);
}

#[test]
fn case_b_stop() {
    // Model variant RP-03.V1 (D-031): `zeno stop`.
    let cm = compile(&bouncing_ball(ZenoPolicy::Stop));
    let run = run_checked(&cm, Config::until(6.0));
    match &run.status {
        RunStatus::Stopped(d) => {
            assert_eq!(d.category, Category::Zeno, "RP-03.E13");
            assert_eq!(d.element.as_deref(), Some("BouncingBall.event.bounce"), "RP-03.E13 names bounce");
        }
        s => panic!("RP-03.E13 status {s:?}"),
    }
    // E14: the last committed state is the one before the detected occurrence (its (t, 0)).
    let last = run.committed.last().unwrap();
    assert_eq!(last.n, 0, "RP-03.E14 no partial commit");
}

#[test]
fn case_c_invalid_e() {
    let cm = compile(&bouncing_ball(settle()));
    let run = prismal_runtime::run(&cm, Config::until(6.0).param(&b("e"), lit(1.2)));
    match &run.status {
        RunStatus::NotStarted(d) => {
            assert!(d.message.contains("e_range") && d.message.contains("1.2"), "RP-03.E15 {}", d.message);
        }
        s => panic!("RP-03.E15 status {s:?}"),
    }
}

#[test]
fn diagnostic_variants() {
    let mut m = bouncing_ball(settle());
    m.events[0].zeno = None;
    let c = codes(&m);
    assert!(c.contains(&"MK-E16"), "RP-03.D1 {c:?}");

    let mut m = bouncing_ball(settle());
    m.flows[1].expr = ite(gt(r(&b("y")), num(0.0, "m")), -r(&b("g")), lit(0.0));
    let c = codes(&m);
    assert!(c.contains(&"MK-E12"), "RP-03.D2 {c:?}");

    let mut m = bouncing_ball(settle());
    m.events[0].handler.push(set(&b("v"), num(0.0, "m/s")));
    let c = codes(&m);
    assert!(c.contains(&"MK-E19"), "RP-03.D3 {c:?}");
}
