//! Declared functions (MK-10.3, D-048) and enumerations (MK-2.2, MK-10.2, D-049): their
//! lowering, their diagnostics, canonical printing and identities across edits.

use prismal_ir::*;
use prismal_syntax::{check, compile, format, reconcile, rename};

const SRC: &str = "model Throw {
  /// Where the ball is.
  enum Phase { up, down, rest }
  const { g: Acceleration = 9.81 m/s^2 }
  param { m: Mass = 0.5 kg }
  state { h: Length = 0 m; v: Velocity = 10 m/s }
  discrete { phase: Phase = up }
  derived {
    energy: Energy = kinetic(m, v) + m * g * h
    code: Real = match phase { up => 1, down => 2, rest => 0 }
    falling_now: Boolean = phase == Phase.down
  }
  /// Kinetic energy.
  fn kinetic(mass: Mass, speed: Velocity): Energy = 0.5 * mass * speed^2
  fn apex(u: Velocity): Length = u^2 / (2 * g)
  fn apex_energy(mass: Mass, u: Velocity): Energy = mass * g * apex(u)
  flow { der(h) = if phase == rest then 0 m/s else v; der(v) = if phase == rest then 0 m/s^2 else -g }
  event top on falling(v) { set phase = down }
}
";

fn codes(src: &str) -> Vec<&'static str> {
    match check(src) {
        Ok(_) => vec![],
        Err(ds) => ds.iter().map(|d| d.code).collect(),
    }
}

/// `SRC` with one line replaced.
fn variant(from: &str, to: &str) -> String {
    assert!(SRC.contains(from), "{from}");
    SRC.replacen(from, to, 1)
}

#[test]
fn lowering() {
    let c = compile(SRC).unwrap();
    let m = &c.doc.models[0];
    assert_eq!(m.enums.len(), 1);
    let phase = &m.enums[0];
    assert_eq!((phase.id.as_str(), phase.cases.clone()), ("Throw.enum.Phase", vec!["up".to_string(), "down".into(), "rest".into()]));
    assert_eq!(phase.notes, vec!["Where the ball is.".to_string()]);
    assert_eq!(m.binding_by_name("phase").unwrap().ty, phase.ty(), "a declared enumeration as a type");
    assert_eq!(m.binding_by_name("phase").unwrap().init, Some(Expr::Case { case: "up".into() }));
    assert_eq!(m.binding_by_name("falling_now").unwrap().def.as_ref().map(|e| matches!(e, Expr::Bin { r, .. } if **r == Expr::Case { case: "down".into() })), Some(true), "`Phase.down` is the case `down`");

    let names: Vec<&str> = m.functions.iter().map(|f| f.id.as_str()).collect();
    assert_eq!(names, ["Throw.fn.kinetic", "Throw.fn.apex", "Throw.fn.apex_energy"]);
    let k = &m.functions[0];
    assert_eq!(k.param_names(), ["mass", "speed"]);
    assert_eq!(k.notes, vec!["Kinetic energy.".to_string()]);
    assert!(matches!(&k.body, Expr::Bin { .. }));
    let energy = m.binding_by_name("energy").unwrap().def.clone().unwrap();
    let mut applies = vec![];
    energy.walk(&mut |e| {
        if let Expr::Apply { apply, .. } = e {
            applies.push((**apply).clone());
        }
    });
    assert_eq!(applies, vec![Expr::Fn { r#fn: "Throw.fn.kinetic".into() }]);
    let code = m.binding_by_name("code").unwrap().def.clone().unwrap();
    assert!(matches!(code, Expr::Match { ref arms, .. } if arms.len() == 3));

    // The IR survives JSON and checks.
    let json = c.doc.to_json();
    assert!(json.contains("\"fn\": \"Throw.fn.kinetic\"") && json.contains("\"enum\": \"Throw.enum.Phase\"") && json.contains("\"match\""));
    assert_eq!(Document::from_json(&json).unwrap(), c.doc);
    assert!(check(SRC).is_ok());
}

#[test]
fn function_diagnostics() {
    // MK-E18: a function reads only its parameters, constants and other functions.
    assert!(codes(&variant("fn apex(u: Velocity): Length = u^2 / (2 * g)", "fn apex(u: Velocity): Length = u * v / g")).contains(&"MK-E18"), "reads state");
    assert!(codes(&variant("fn apex(u: Velocity): Length = u^2 / (2 * g)", "fn apex(u: Velocity): Length = u^2 / (2 * g) + 0 * m / (1 kg / 1 m)")).contains(&"MK-E18"), "reads a parameter");
    assert!(codes(&variant("fn apex(u: Velocity): Length = u^2 / (2 * g)", "fn apex(u: Velocity): Length = u * elapsed")).contains(&"MK-E18"), "reads the time");
    // MK-E23: no recursion, direct or through another function.
    assert!(codes(&variant("fn apex(u: Velocity): Length = u^2 / (2 * g)", "fn apex(u: Velocity): Length = apex(u)")).contains(&"MK-E23"));
    let mutual = variant(
        "fn apex(u: Velocity): Length = u^2 / (2 * g)",
        "fn apex(u: Velocity): Length = u^2 / (2 * g) + 0 * apex_energy(1 kg, u) / (1 N)",
    );
    assert!(codes(&mutual).contains(&"MK-E23"), "{:?}", codes(&mutual));
    // The result type and the arguments are checked.
    assert!(codes(&variant("fn apex(u: Velocity): Length = u^2 / (2 * g)", "fn apex(u: Velocity): Length = u / g")).contains(&"MK-E01"));
    assert!(codes(&variant("kinetic(m, v) + m * g * h", "kinetic(v, m) + m * g * h")).contains(&"MK-E01"));
    assert!(codes(&variant("kinetic(m, v) + m * g * h", "kinetic(m) + m * g * h")).contains(&"MK-E01"));
    // One namespace: a function may not share a binding's name.
    assert!(codes(&variant("fn apex(u", "fn h(u")).contains(&"SX-E09"));
    assert!(codes(&variant("fn apex_energy(", "fn kinetic(")).contains(&"SX-E09"));
}

#[test]
fn enumeration_diagnostics() {
    // MK-E17: a match covers every case exactly once.
    assert!(codes(&variant("up => 1, down => 2, rest => 0", "up => 1, down => 2")).contains(&"MK-E17"));
    assert!(codes(&variant("up => 1, down => 2, rest => 0", "up => 1, down => 2, rest => 0, up => 3")).contains(&"MK-E17"));
    // A case of another type, a match on a number, an ordered comparison.
    assert!(codes(&variant("discrete { phase: Phase = up }", "discrete { phase: Phase = 1 }")).contains(&"MK-E01"));
    assert!(codes(&variant("match phase {", "match h {")).contains(&"MK-E01"));
    assert!(codes(&variant("phase == Phase.down", "phase < down")).contains(&"MK-E01"));
    assert!(codes(&variant("Phase.down", "Phase.sideways")).contains(&"SX-E03"));
    // Two enumerations are different types even with the same cases (MK-2.2).
    let two = variant("enum Phase { up, down, rest }", "enum Phase { up, down, rest }\n  enum Other { up, down, rest }").replacen("discrete { phase: Phase = up }", "discrete { phase: Phase = up; other: Other = up }", 1);
    assert!(check(&two).is_ok());
    assert!(codes(&two.replacen("falling_now: Boolean = phase == Phase.down", "falling_now: Boolean = phase == other", 1)).contains(&"MK-E01"));
    // Cases share the namespace of bindings; duplicates are reported.
    assert!(codes(&variant("enum Phase { up, down, rest }", "enum Phase { up, down, h }")).contains(&"SX-E09"));
    assert!(codes(&variant("enum Phase { up, down, rest }", "enum Phase { up, down, rest, up }")).contains(&"MK-E22"));
    // `enum` and `match` are reserved words.
    assert!(codes(&variant("param { m: Mass = 0.5 kg }", "param { m: Mass = 0.5 kg; match: Real = 1 }")).contains(&"SX-E07"));
}

#[test]
fn printing_and_identities() {
    let c = compile(SRC).unwrap();
    let text = format(&c.doc);
    assert!(text.contains("  /// Where the ball is.\n  enum Phase { up, down, rest }"), "{text}");
    assert!(text.contains("  fn kinetic(mass: Mass, speed: Velocity): Energy = 0.5 * mass * speed^2"), "{text}");
    assert!(text.contains("code:        Real    = match phase { up => 1, down => 2, rest => 0 }") || text.contains("match phase { up => 1, down => 2, rest => 0 }"));
    let again = compile(&text).unwrap();
    assert_eq!(reconcile(again.doc, &c.doc), c.doc, "printing and reading back gives the same IR");

    // A tool rename keeps identity; every use follows (D-036).
    let r = rename(&c.doc, "Throw.fn.kinetic", "motion_energy").unwrap();
    let printed = format(&r);
    assert!(printed.contains("fn motion_energy(") && printed.contains("motion_energy(m, v)"), "{printed}");
    let r = rename(&c.doc, "Throw.enum.Phase", "Stage").unwrap();
    let printed = format(&r);
    assert!(printed.contains("enum Stage {") && printed.contains("phase: Stage = up"), "{printed}");
    assert!(rename(&c.doc, "Throw.fn.apex", "energy").is_err(), "a name already declared");

    // A text edit that renames a function is a removal and an addition.
    let edited = compile(&SRC.replace("apex(", "peak(")).unwrap();
    let d = prismal_syntax::diff(&c.doc, &reconcile(edited.doc, &c.doc));
    assert_eq!(d.removed, vec!["Throw.fn.apex".to_string()]);
}
