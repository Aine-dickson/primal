//! RP-05 Spring-mass (docs/spec/reference-programs/RP-05-spring-mass.md).

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_runtime::{Category, Config, Run, SolverConfig};

fn s(n: &str) -> String {
    id("SpringMass", n)
}

fn max_energy_dev(run: &Run) -> f64 {
    run.every(0.01, &r(&s("energy"))).iter().map(|(_, v)| ((v.num() - 0.02) / 0.02).abs()).fold(0.0, f64::max)
}

#[test]
fn case_a_dopri5() {
    let cm = compile(&spring_mass());
    let run = run_checked(&cm, Config::until(100.0));
    assert_close(run.at(10.0, &r(&s("x"))).num(), 0.0408082061813392, 1e-5, "RP-05.E1 x_10");
    let passes = run.times("pass");
    assert_rel(passes[0], 2.35619449019234, 1e-5, "RP-05.E2 first pass");
    for w in passes.windows(2) {
        assert_rel(w[1] - w[0], std::f64::consts::PI, 1e-5, "RP-05.E3 period");
    }
    assert_eq!(passes.len(), 32, "RP-05.E4 number of passes");
    let dev = max_energy_dev(&run);
    assert!(dev < 1e-3, "RP-05.E5 energy drift {dev:e}");
    let residuals: Vec<_> = run.diagnostics.iter().filter(|d| d.category == Category::Equation).collect();
    assert!(residuals.is_empty(), "RP-05.E6 residuals reported: {residuals:?}");
    // E10: the derived period is pi seconds, exact up to one rounding.
    assert_close(run.at(0.0, &r(&s("T"))).num(), std::f64::consts::PI, 4.5e-16, "RP-05.E10 T");
}

#[test]
fn case_b_rk4() {
    let cm = compile(&spring_mass());
    let run = run_checked(&cm, Config::until(100.0).solver(SolverConfig::Rk4 { h: Some(0.01) }));
    assert_close(run.at(10.0, &r(&s("x"))).num(), 0.0408082061813392, 1e-7, "RP-05.E7 x_10");
    let dev = max_energy_dev(&run);
    assert!(dev < 1e-7, "RP-05.E8 energy drift {dev:e}");
}

#[test]
fn diagnostic_variants() {
    // E9: the display equation checks (both sides have dimension T); D1 swaps k and m.
    let mut m = spring_mass();
    m.equations[0].rhs = lit(2.0 * std::f64::consts::PI) * sqrt(r(&s("k")) / r(&s("m")));
    let c = codes(&m);
    assert!(c.contains(&"MK-E01"), "RP-05.D1 {c:?}");

    let mut m = spring_mass();
    let e = m.bindings.iter_mut().find(|b| b.name == "energy").unwrap();
    e.def = Some(lit(0.5) * r(&s("m")) * pow(r(&s("v")), lit(2.0)) + lit(0.5) * r(&s("k")) * r(&s("x")));
    let c = codes(&m);
    assert!(c.contains(&"MK-E01"), "RP-05.D2 {c:?}");

    let mut m = spring_mass();
    m.flows[1].expr = -(r(&s("k")) / r(&s("m"))) * r(&s("x")) + lit(1.0);
    let c = codes(&m);
    assert!(c.contains(&"MK-E03"), "RP-05.D3 {c:?}");
}
