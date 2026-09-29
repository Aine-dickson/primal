//! RP-04 Pendulum (docs/spec/reference-programs/RP-04-pendulum.md).

mod common;
use common::*;
use prismal_ir::build::*;
use prismal_ir::*;
use prismal_runtime::{Category, Config, Run, RunStatus, SolverConfig};

const T_EXACT: f64 = 2.00989262729860;
const T_SMALL: f64 = 2.00606668071065;

fn pd(n: &str) -> String {
    id("Pendulum", n)
}

fn max_energy_dev(run: &Run) -> f64 {
    let e = run.every(0.01, &r(&pd("energy")));
    let e0 = e[0].1.num();
    e.iter().map(|(_, v)| ((v.num() - e0) / e0).abs()).fold(0.0, f64::max)
}

fn periods(run: &Run) -> Vec<f64> {
    run.times("upswing").windows(2).map(|w| w[1] - w[0]).collect()
}

#[test]
fn case_a_dopri5_and_b_rk4() {
    let cm = compile(&pendulum());
    let a = run_checked(&cm, Config::until(100.0));
    assert_close(a.at(0.0, &r(&pd("θ"))).num(), 0.174532925199433, 1e-15, "RP-04.E1 theta_start");
    let ups = a.times("upswing");
    assert_rel(ups[0], 1.50741947047395, 1e-5, "RP-04.E2 first upswing");
    for (k, p) in periods(&a).iter().enumerate() {
        assert_rel(*p, T_EXACT, 1e-5, &format!("RP-04.E3 period {k}"));
    }
    let mean: f64 = periods(&a).iter().sum::<f64>() / periods(&a).len() as f64;
    let ratio = mean / T_SMALL - 1.0;
    assert!((1.85e-3..=1.96e-3).contains(&ratio), "RP-04.E4 period / T0 - 1 = {ratio}");
    let dev_a = max_energy_dev(&a);
    assert!(dev_a < 1e-3, "RP-04.E5 energy drift {dev_a:e}");
    assert!(a.diagnostics.is_empty(), "RP-04.E6 diagnostics {:?}", a.diagnostics);
    assert_eq!(ups.len(), 50, "RP-04.E7 number of upswings");

    let b = run_checked(&cm, Config::until(100.0).solver(SolverConfig::Rk4 { h: Some(0.01) }));
    for (k, p) in periods(&b).iter().enumerate() {
        assert_rel(*p, T_EXACT, 1e-6, &format!("RP-04.E8 period {k}"));
    }
    let dev_b = max_energy_dev(&b);
    assert!(dev_b < 1e-6, "RP-04.E9 energy drift {dev_b:e}");
    assert!(dev_b < dev_a, "RP-04.E10 rk4 drift {dev_b:e} not below dopri5 drift {dev_a:e}");
    println!("RP-04 energy drift: dopri5 {dev_a:e}, rk4 {dev_b:e}");
}

#[test]
fn case_c_rk4_no_step() {
    let cm = compile(&pendulum());
    let run = prismal_runtime::run(&cm, Config::until(100.0).solver(SolverConfig::Rk4 { h: None }));
    match run.status {
        RunStatus::NotStarted(d) => assert_eq!(d.category, Category::Configuration, "RP-04.E11"),
        s => panic!("RP-04.E11 status {s:?}"),
    }
}

#[test]
fn diagnostic_variants() {
    // D1: an angle state declared as a length.
    let mut m = pendulum();
    let th = m.bindings.iter_mut().find(|b| b.name == "θ").unwrap();
    th.ty = Type::qty("L");
    th.init = Some(num(10.0, "cm"));
    let c = codes(&m);
    assert!(c.contains(&"MK-E02"), "RP-04.D1 {c:?}");

    // D2: the bob without the rod length (D-032: the tuple is expected to be a displacement).
    let mut m = pendulum();
    let bob = m.bindings.iter_mut().find(|b| b.name == "bob").unwrap();
    bob.def = Some(r(&pd("pivot")) + tuple(vec![sin(r(&pd("θ"))), -cos(r(&pd("θ")))]));
    let c = codes(&m);
    assert!(c.contains(&"MK-E01"), "RP-04.D2 {c:?}");
}
