//! Lexical rules, lowering choices and diagnostics of the working syntax.

use prismal_ir::build::*;
use prismal_ir::*;
use prismal_syntax::{compile, parse, Diag};

fn model(src: &str) -> Model {
    let c = compile(src).unwrap_or_else(|d| panic!("{}", d.iter().map(|x| x.render(src, "t")).collect::<Vec<_>>().join("\n")));
    c.doc.models[0].clone()
}

fn init(m: &Model, name: &str) -> Expr {
    let b = m.binding_by_name(name).unwrap();
    b.init.clone().or(b.def.clone()).unwrap()
}

fn errors(src: &str) -> Vec<Diag> {
    compile(src).expect_err("must be rejected")
}

fn codes(src: &str) -> Vec<&'static str> {
    errors(src).iter().map(|d| d.code).collect()
}

#[test]
fn units_are_written_without_spaces() {
    let m = model(
        "model M {
           param {
             m: Mass           = 1 kg
             a: Quantity<1/M>  = 4 / m
             k: Quantity<1/L>  = 0.01 /m
             g: Acceleration   = 9.81 m/s^2
             c: Quantity<L^-2> = 3 m^-2
           }
         }",
    );
    assert_eq!(init(&m, "a"), lit(4.0) / r("M.m"), "`4 / m` divides by the binding m");
    assert_eq!(init(&m, "k"), num(0.01, "/m"), "`0.01 /m` is a unit");
    assert_eq!(init(&m, "g"), num(9.81, "m/s^2"));
    assert_eq!(init(&m, "c"), num(3.0, "m^-2"));
}

#[test]
fn numbers_constants_and_operators() {
    let m = model(
        "model M {
           param { x: Real = 0.5 }
           derived {
             tau: Real    = 2π
             p:   Real    = -x^2
             n:   Real    = -2
             ok:  Boolean = 0 < x <= 1
             r:   Boolean = (x in [0, 1))   // after a declaration's value, `in` starts a range
           }
         }",
    );
    assert_eq!(init(&m, "tau"), lit(2.0) * pi());
    assert_eq!(init(&m, "p"), -pow(r("M.x"), lit(2.0)), "`^` binds tighter than unary minus");
    assert_eq!(init(&m, "n"), lit(-2.0), "a minus sign on a literal is a negative literal");
    assert_eq!(init(&m, "ok"), and(lt(lit(0.0), r("M.x")), le(r("M.x"), lit(1.0))));
    assert_eq!(init(&m, "r"), interval(r("M.x"), lit(0.0), true, lit(1.0), false));
    assert!(compile("model M { derived { y: Real = 2x } }").is_err(), "`2x` is not a product");
}

#[test]
fn ranges_lower_to_reject_constraints() {
    let m = model("model M { param { k: Real = 0 in [0, inf) ; q: Real = 1 where q > 0 } }");
    assert_eq!(m.constraints.len(), 2);
    let k = &m.constraints[0];
    assert_eq!((k.name.as_str(), k.policy, k.attached_to.as_deref()), ("k_range", Policy::Reject, Some("M.k")));
    assert_eq!(k.cond, ge(r("M.k"), lit(0.0)), "an unbounded end gives a one-sided range");
    let w = model("model M { param { k: Real = 0 where k >= 0 } }");
    assert_eq!(w.constraints[0].cond, k.cond, "`in [0, inf)` and `where k >= 0` are one IR (section 1.4)");
    let other = model("model M { param { x: Real = 1 }; constraint x < 2 }");
    assert_eq!(other.constraints[0].policy, Policy::Report, "MK-12.4: other constraints default to `report`");
    assert_eq!(other.constraints[0].name, "c1");
}

#[test]
fn comments_before_a_declaration_become_notes() {
    let m = model(
        "/// A falling body.
         model M {
           param {
             /// Gravity.
             // at sea level
             g: Acceleration = 9.81 m/s^2   // trailing comments are not notes
             // separated by a blank line: not a note

             h: Length = 1 m
           }
           state { y: Length = h ; v: Velocity = 0 }
           flow { der(y) = v ; der(v) = -g }
         }",
    );
    assert_eq!(m.notes, vec!["A falling body.".to_string()]);
    assert_eq!(m.binding_by_name("g").unwrap().notes, vec!["Gravity.\nat sea level".to_string()]);
    assert!(m.binding_by_name("h").unwrap().notes.is_empty());
}

#[test]
fn modifiers_and_processes() {
    let m = model(
        "space Plane = euclidean(2)
         model M in Plane {
           param { a: Angle = 45 deg symbol \"θ\" unit deg }
           state { p: Point = origin intervenable ; v: Vector<Velocity> = 0 }
           flow der(p) = v
           process drag { flow der(v) += -v / (1 s) }
           event tick on every 1 s
         }",
    );
    let a = m.binding_by_name("a").unwrap();
    assert_eq!((a.display.symbol.as_deref(), a.display.unit.as_deref()), (Some("θ"), Some("deg")));
    assert_eq!(m.binding_by_name("p").unwrap().intervenable, Some(true));
    assert_eq!(m.processes[0].name, "drag", "`drag` is a contextual keyword (D-040)");
    assert_eq!(m.flows[1].process.as_deref(), Some("M.process.drag"));
    assert_eq!(m.events[0].trigger, Trigger::Every { period: num(1.0, "s"), from: t0() });
}

#[test]
fn diagnostics_are_located() {
    let src = "model M {\n  param { g: Acceleration = 9.81 m/s^2 }\n  state { y: Length = h }\n}\n";
    let e = errors(src);
    assert_eq!(e.len(), 1);
    assert_eq!((e[0].code, e[0].span.line, e[0].span.col), ("SX-E03", 3, 23));
    assert!(e[0].render(src, "m.prismal").starts_with("m.prismal:3:23: SX-E03: unknown name `h` in model `M`"));
}

#[test]
fn several_errors_are_reported() {
    let (_, d) = parse("model M {\n  param { a: Real = 1 + }\n  param { in: Real = 1 }\n  state { y: Length = 1 m where y = 0 m }\n}\n");
    let c: Vec<&str> = d.iter().map(|x| x.code).collect();
    assert_eq!(c, vec!["SX-E02", "SX-E07", "SX-E02"], "{d:#?}");
    assert!(d[2].message.contains("`==`"), "`=` where `==` is meant: {}", d[2].message);
}

#[test]
fn lowering_errors() {
    assert_eq!(codes("model M { state { p: Point = origin } }"), vec!["SX-E08", "SX-E08"], "no default space");
    assert_eq!(codes("model M { param { x: Length = 3 px } }"), vec!["SX-E05"]);
    assert_eq!(codes("model M { param { x: Speed = 3 } }"), vec!["SX-E04"]);
    assert_eq!(codes("model M { object O { } }"), vec!["SX-E06"]);
    assert_eq!(codes("model M { event a on start\n event a on start }"), vec!["SX-E09"]);
    assert_eq!(codes("model M { event a on b }"), vec!["SX-E03"]);
    assert_eq!(codes("model M { param { f(x: Real): Real = x } }"), vec!["SX-E08"]);
    assert_eq!(codes("model M { derived { y: Real = sin(1, 2) } }"), vec!["SX-E08"]);
}
