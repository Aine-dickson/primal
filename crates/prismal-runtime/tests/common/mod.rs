//! The first-slice reference programs (docs/spec/reference-programs/), built through the IR
//! API. Each builder mirrors the working-syntax program of the same number.
#![allow(dead_code)]

use prismal_ir::build::*;
use prismal_ir::*;
use prismal_kernel::{check_model, CModel, Diagnostic, Value};
use prismal_runtime::{run, Config, Run};

pub fn plane() -> Space {
    space("Plane", 2)
}

pub fn compile(m: &Model) -> CModel {
    check_model(&[plane()], m).unwrap_or_else(|d| panic!("model does not check: {d:#?}"))
}

/// Diagnostic codes of a model that must fail to check.
pub fn codes(m: &Model) -> Vec<&'static str> {
    codes_in(&[plane()], m)
}

pub fn codes_in(spaces: &[Space], m: &Model) -> Vec<&'static str> {
    match check_model(spaces, m) {
        Ok(_) => vec![],
        Err(d) => d.iter().map(|x: &Diagnostic| x.code).collect(),
    }
}

pub fn id(model: &str, name: &str) -> String {
    format!("{model}.{name}")
}

pub fn assert_close(got: f64, want: f64, abs: f64, what: &str) {
    assert!((got - want).abs() <= abs, "{what}: got {got:.15e}, expected {want:.15e} within {abs:e} (diff {:e})", (got - want).abs());
}

pub fn assert_rel(got: f64, want: f64, rel: f64, what: &str) {
    let r = ((got - want) / want).abs();
    assert!(r <= rel, "{what}: got {got:.15e}, expected {want:.15e} within rel {rel:e} (rel diff {r:e})");
}

pub fn num_of(v: &Value) -> f64 {
    v.num()
}

pub fn xy(v: &Value) -> (f64, f64) {
    let a = v.arr();
    (a.v[0], a.v[1])
}

/// Runs a model twice and checks bit-identical replay (RC-14.5).
pub fn run_checked(cm: &CModel, cfg: Config) -> Run {
    let a = run(cm, cfg.clone());
    let b = run(cm, cfg);
    assert_eq!(a.committed.len(), b.committed.len(), "replay: committed state count");
    for (x, y) in a.committed.iter().zip(b.committed.iter()) {
        assert!(x.t.to_bits() == y.t.to_bits() && x.n == y.n && x.vals == y.vals, "replay differs at t = {}", x.t);
    }
    assert_eq!(a.log.len(), b.log.len(), "replay: event log length");
    a
}

// ---------------------------------------------------------------- RP-01, RP-02, RP-08

pub fn projectile(relaunch: bool) -> Model {
    let mut b = ModelBuilder::new("Projectile").in_space("Plane");
    let g = b.param("g", Type::qty("L/T^2"), num(9.81, "m/s^2"));
    let k = b.param("k", Type::qty("1/L"), lit(0.0));
    b.range(&k, ge(r(&k), lit(0.0)));
    let speed = b.param("speed", Type::qty("L/T"), num(20.0, "m/s"));
    b.range(&speed, gt(r(&speed), num(0.0, "m/s")));
    b.symbol(&speed, "v");
    let angle = b.param("angle", Type::real(), num(45.0, "deg"));
    b.range(&angle, interval(r(&angle), num(0.0, "deg"), false, num(90.0, "deg"), false));
    b.symbol(&angle, "θ");
    let launch = || r("Projectile.speed") * tuple(vec![cos(r("Projectile.angle")), sin(r("Projectile.angle"))]);
    let pos = b.state("pos", Type::point("Plane"), origin("Plane"));
    let vel = b.state("vel", Type::vector("Plane", "L/T"), launch());
    let flying = b.discrete("flying", Type::Boolean, boolean(true));
    b.flow(&pos, ite(r(&flying), r(&vel), lit(0.0)));
    let gravity = b.process("gravity");
    let drag = b.process("drag");
    b.contribute_in(&gravity, &vel, ite(r(&flying), tuple(vec![lit(0.0), -r(&g)]), lit(0.0)));
    b.contribute_in(&drag, &vel, -r(&k) * norm(r(&vel)) * r(&vel));
    b.event("apex", falling(comp(r(&vel), 1)), vec![]);
    b.event("landed", falling(comp(r(&pos), 1)), vec![set(&flying, boolean(false)), set(&vel, lit(0.0))]);
    if relaunch {
        b.event("relaunch", Trigger::Request, vec![set(&pos, origin("Plane")), set(&vel, launch()), set(&flying, boolean(true))]);
    }
    b.finish()
}

// ---------------------------------------------------------------- RP-03

pub fn bouncing_ball(zeno: ZenoPolicy) -> Model {
    let mut b = ModelBuilder::new("BouncingBall");
    let g = b.param("g", Type::qty("L/T^2"), num(9.81, "m/s^2"));
    let h0 = b.param("h0", Type::qty("L"), num(1.0, "m"));
    b.range(&h0, gt(r(&h0), num(0.0, "m")));
    let e = b.param("e", Type::real(), lit(0.8));
    b.range(&e, interval(r(&e), lit(0.0), true, lit(1.0), false));
    let y = b.state("y", Type::qty("L"), r(&h0));
    let v = b.state("v", Type::qty("L/T"), lit(0.0));
    let resting = b.discrete("resting", Type::Boolean, boolean(false));
    b.flow(&y, r(&v));
    b.flow(&v, ite(r(&resting), lit(0.0), -r(&g)));
    let bounce = b.event("bounce", falling(r(&y)), vec![set(&v, -r(&e) * r(&v))]);
    b.zeno(&bounce, zeno);
    b.event("top", falling(r(&v)), vec![]);
    b.finish()
}

pub fn settle() -> ZenoPolicy {
    ZenoPolicy::Settle {
        ops: vec![
            set("BouncingBall.y", num(0.0, "m")),
            set("BouncingBall.v", num(0.0, "m/s")),
            set("BouncingBall.resting", boolean(true)),
        ],
    }
}

// ---------------------------------------------------------------- RP-04

pub fn pendulum() -> Model {
    let mut b = ModelBuilder::new("Pendulum").in_space("Plane");
    let g = b.param("g", Type::qty("L/T^2"), num(9.81, "m/s^2"));
    let l = b.param("L", Type::qty("L"), num(1.0, "m"));
    b.range(&l, gt(r(&l), num(0.0, "m")));
    let m = b.param("m", Type::qty("M"), num(1.0, "kg"));
    b.range(&m, gt(r(&m), num(0.0, "kg")));
    let pivot = b.param("pivot", Type::point("Plane"), origin("Plane"));
    let th0 = b.param("θ0", Type::real(), num(10.0, "deg"));
    let th = b.state("θ", Type::real(), r(&th0));
    let om = b.state("ω", Type::qty("1/T"), lit(0.0));
    let bob = b.derived("bob", Type::point("Plane"), r(&pivot) + r(&l) * tuple(vec![sin(r(&th)), -cos(r(&th))]));
    b.derived(
        "energy",
        Type::qty("M L^2 T^-2"),
        lit(0.5) * r(&m) * pow(r(&l) * r(&om), lit(2.0)) + r(&m) * r(&g) * r(&l) * (lit(1.0) - cos(r(&th))),
    );
    b.flow(&th, r(&om));
    b.flow(&om, -(r(&g) / r(&l)) * sin(r(&th)));
    b.constraint("rod", eq(norm(r(&bob) - r(&pivot)), r(&l)), Some(num(1e-9, "m")), Policy::Report, None);
    b.event("upswing", rising(r(&th)), vec![]);
    b.finish()
}

// ---------------------------------------------------------------- RP-05

pub fn spring_mass() -> Model {
    let mut b = ModelBuilder::new("SpringMass");
    let m = b.param("m", Type::qty("M"), num(1.0, "kg"));
    b.range(&m, gt(r(&m), num(0.0, "kg")));
    let k = b.param("k", Type::qty("M/T^2"), num(4.0, "N/m"));
    b.range(&k, gt(r(&k), num(0.0, "N/m")));
    let x0 = b.param("x0", Type::qty("L"), num(0.1, "m"));
    let x = b.state("x", Type::qty("L"), r(&x0));
    let v = b.state("v", Type::qty("L/T"), lit(0.0));
    let energy = b.derived(
        "energy",
        Type::qty("M L^2 T^-2"),
        lit(0.5) * r(&m) * pow(r(&v), lit(2.0)) + lit(0.5) * r(&k) * pow(r(&x), lit(2.0)),
    );
    let tt = b.derived("T", Type::qty("T"), lit(2.0 * std::f64::consts::PI) * sqrt(r(&m) / r(&k)));
    b.flow(&x, r(&v));
    b.flow(&v, -(r(&k) / r(&m)) * r(&x));
    b.equation("period_law", r(&tt), lit(2.0 * std::f64::consts::PI) * sqrt(r(&m) / r(&k)), None);
    b.equation("conservation", r(&energy), lit(0.5) * r(&k) * pow(r(&x0), lit(2.0)), Some(num(2e-5, "J")));
    b.event("pass", rising(r(&x)), vec![]);
    b.finish()
}

// ---------------------------------------------------------------- RP-06

pub fn quadratic() -> Model {
    let mut b = ModelBuilder::new("QuadraticDemo");
    let a = b.param("a", Type::real(), lit(1.0));
    b.range(&a, interval(r(&a), lit(-5.0), true, lit(5.0), true));
    b.derived("f", Type::func(vec![Type::real()], Type::real()), lambda(vec![Type::real()], r(&a) * pow(param(0), lit(2.0))));
    b.finish()
}

// ---------------------------------------------------------------- RP-07

pub fn vectors(extra: impl FnOnce(&mut ModelBuilder)) -> Model {
    let mut b = ModelBuilder::new("VectorDemo").in_space("Plane");
    let a = b.param("A", Type::point("Plane"), origin("Plane") + tuple(vec![num(1.0, "m"), num(2.0, "m")]));
    let u = b.param("u", Type::vector("Plane", "L"), tuple(vec![num(3.0, "m"), num(0.0, "m")]));
    let w = b.param("w", Type::vector("Plane", "L"), tuple(vec![num(0.0, "m"), num(2.0, "m")]));
    let sum = b.derived("sum", Type::vector("Plane", "L"), r(&u) + r(&w));
    b.derived("length", Type::qty("L"), norm(r(&sum)));
    b.derived("B", Type::point("Plane"), r(&a) + r(&sum));
    b.derived("mid", Type::point("Plane"), r(&a) + r(&u));
    extra(&mut b);
    b.finish()
}
