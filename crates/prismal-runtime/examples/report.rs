//! Prints the values the prototype measures for the provisional expectations of the
//! reference programs: `cargo run --release -p prismal-runtime --example report`.

#[path = "../tests/common/mod.rs"]
mod common;
use common::*;
use prismal_ir::build::*;
use prismal_runtime::{run, Config, SolverConfig};

fn main() {
    let cm = compile(&bouncing_ball(settle()));
    let r3 = run(&cm, Config::until(6.0));
    let z = r3.log.iter().find(|l| l.zeno).unwrap();
    let nth = r3.log.iter().filter(|l| l.name == "bounce").position(|l| l.zeno).unwrap() + 1;
    println!("RP-03 zeno applied at bounce {nth}, t = {:.15}, t_inf - t = {:.3e}", z.t, 4.06371276887158 - z.t);
    println!("RP-03 bounce 10 abs err {:.2e}", (r3.times("bounce")[9] - 3.57889295102044).abs());

    let cm = compile(&projectile(false));
    let r1 = run(&cm, Config::until(5.0));
    println!(
        "RP-01 t_land abs err {:.2e}, range abs err {:.2e}",
        (r1.on("landed", &elapsed(), None)[0].num() - 2.88320807823261).abs(),
        (r1.on("landed", &comp(r("Projectile.pos"), 0), None)[0].num() - 40.7747196738022).abs()
    );
    for (name, s) in [
        ("default", SolverConfig::Dopri5 { rtol: 1e-6, atol: 1e-9, h_max: f64::INFINITY }),
        ("tight", SolverConfig::Dopri5 { rtol: 1e-10, atol: 1e-12, h_max: f64::INFINITY }),
    ] {
        let r2 = run(&cm, Config::until(5.0).param("Projectile.k", num(0.01, "/m")).solver(s));
        let t = r2.on("landed", &elapsed(), None)[0].num();
        let x = r2.on("landed", &comp(r("Projectile.pos"), 0), None)[0].num();
        let h = r2.on("apex", &comp(r("Projectile.pos"), 1), None)[0].num();
        println!(
            "RP-02 {name}: t_land rel err {:.2e}, range rel err {:.2e}, h_apex rel err {:.2e}",
            (t / 2.67328857282834 - 1.0).abs(),
            (x / 31.3229266146783 - 1.0).abs(),
            (h / 8.78272286661045 - 1.0).abs()
        );
    }

    let cm = compile(&pendulum());
    for (name, s) in [
        ("dopri5", SolverConfig::Dopri5 { rtol: 1e-6, atol: 1e-9, h_max: f64::INFINITY }),
        ("rk4", SolverConfig::Rk4 { h: Some(0.01) }),
    ] {
        let r4 = run(&cm, Config::until(100.0).solver(s));
        let ups = r4.times("upswing");
        let worst = ups.windows(2).map(|w| ((w[1] - w[0]) / 2.00989262729860 - 1.0).abs()).fold(0.0, f64::max);
        let e = r4.every(0.01, &r("Pendulum.energy"));
        let e0 = e[0].1.num();
        let dev = e.iter().map(|(_, v)| ((v.num() - e0) / e0).abs()).fold(0.0, f64::max);
        println!(
            "RP-04 {name}: first upswing rel err {:.2e}, worst period rel err {worst:.2e}, energy drift {dev:.2e}, steps {}",
            (ups[0] / 1.50741947047395 - 1.0).abs(),
            r4.segments.len()
        );
    }

    let cm = compile(&spring_mass());
    for (name, s) in [
        ("dopri5", SolverConfig::Dopri5 { rtol: 1e-6, atol: 1e-9, h_max: f64::INFINITY }),
        ("rk4", SolverConfig::Rk4 { h: Some(0.01) }),
    ] {
        let r5 = run(&cm, Config::until(100.0).solver(s));
        let x10 = r5.at(10.0, &r("SpringMass.x")).num();
        let p = r5.times("pass");
        let worst = p.windows(2).map(|w| ((w[1] - w[0]) / std::f64::consts::PI - 1.0).abs()).fold(0.0, f64::max);
        let e = r5.every(0.01, &r("SpringMass.energy"));
        let dev = e.iter().map(|(_, v)| ((v.num() - 0.02) / 0.02).abs()).fold(0.0, f64::max);
        println!("RP-05 {name}: x_10 abs err {:.2e}, worst period rel err {worst:.2e}, energy drift {dev:.2e}", (x10 - 0.0408082061813392).abs());
    }
}
