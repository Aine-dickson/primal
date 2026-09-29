//! RP-02 Projectile with drag (docs/spec/reference-programs/RP-02-projectile-drag.md).

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_runtime::{Config, Run, SolverConfig};

fn p(n: &str) -> String {
    id("Projectile", n)
}

fn drag_run(cfg: Config) -> Run {
    let cm = compile(&projectile(false));
    run_checked(&cm, cfg.param(&p("k"), num(0.01, "/m")))
}

#[test]
fn case_a_default() {
    let run = drag_run(Config::until(5.0));
    let t_land = run.on("landed", &elapsed(), None)[0].num();
    assert_rel(t_land, 2.67328857282834, 1e-5, "RP-02.E1 t_land");
    assert_rel(run.on("landed", &comp(r(&p("pos")), 0), None)[0].num(), 31.3229266146783, 1e-5, "RP-02.E2 range");
    let (vx, vy) = xy(&run.on("landed", &r(&p("vel")), Some(0))[0]);
    assert_rel(vx, 9.76677238322685, 1e-5, "RP-02.E3 v_land.x");
    assert_rel(vy, -12.3148244403059, 1e-5, "RP-02.E3 v_land.y");
    assert_rel(run.on("apex", &elapsed(), None)[0].num(), 1.29589657291982, 1e-5, "RP-02.E4 t_apex");
    assert_rel(run.on("apex", &comp(r(&p("pos")), 1), None)[0].num(), 8.78272286661045, 1e-5, "RP-02.E5 h_apex");

    // E6: energy per unit mass strictly decreases during flight.
    let energy = lit(0.5) * pow(norm(r(&p("vel"))), lit(2.0)) + r(&p("g")) * comp(r(&p("pos")) - origin("Plane"), 1);
    let series: Vec<f64> = run.every(0.01, &energy).into_iter().filter(|(t, _)| *t < t_land).map(|(_, v)| v.num()).collect();
    assert!(series.len() > 200);
    for w in series.windows(2) {
        assert!(w[1] < w[0], "RP-02.E6 energy not decreasing: {} then {}", w[0], w[1]);
    }

    // E7: the runtime's combined flow equals the hand-written sum (D-005).
    let balance = der(&p("vel")) - (tuple(vec![lit(0.0), -r(&p("g"))]) - r(&p("k")) * norm(r(&p("vel"))) * r(&p("vel")));
    for (t, v) in run.every(0.01, &balance) {
        if t >= t_land {
            break;
        }
        let (bx, by) = xy(&v);
        assert!(bx.abs() <= 1e-12 && by.abs() <= 1e-12, "RP-02.E7 balance ({bx:e}, {by:e}) at {t}");
    }

    // E8: shorter than without drag.
    assert!(run.on("landed", &comp(r(&p("pos")), 0), None)[0].num() < 40.7747196738022, "RP-02.E8");

    // E9: the final microstep of the landing instant holds the reset velocity.
    let (fx, fy) = xy(&run.on("landed", &r(&p("vel")), None)[0]);
    assert!(fx == 0.0 && fy == 0.0, "RP-02.E9 vel after reset ({fx}, {fy})");
}

#[test]
fn case_b_tight() {
    let run = drag_run(Config::until(5.0).solver(SolverConfig::Dopri5 { rtol: 1e-10, atol: 1e-12, h_max: f64::INFINITY }));
    assert_rel(run.on("landed", &elapsed(), None)[0].num(), 2.67328857282834, 1e-8, "RP-02.E10 t_land");
    assert_rel(run.on("landed", &comp(r(&p("pos")), 0), None)[0].num(), 31.3229266146783, 1e-8, "RP-02.E11 range");
    assert_rel(run.on("apex", &comp(r(&p("pos")), 1), None)[0].num(), 8.78272286661045, 1e-8, "RP-02.E12 h_apex");
}
