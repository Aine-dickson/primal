//! The formatter (working syntax section 1.4) and identities across edits (D-036), on every
//! reference program and every program of the guide.

use prismal_syntax::{compile, diff, format, reconcile, rename};
use prismal_web::examples;

/// Text to IR to canonical text to IR gives the same IR once identities and order are
/// matched to the first (D-036, IR-1.7); printing the result again gives the same text.
#[test]
fn every_program_round_trips() {
    let mut failures = vec![];
    for ex in examples::all() {
        let first = match compile(&ex.source) {
            Ok(c) => c.doc,
            Err(d) => panic!("{}: {:?}", ex.key, d),
        };
        let text = format(&first);
        let second = match compile(&text) {
            Ok(c) => c.doc,
            Err(ds) => {
                failures.push(format!("{}: the canonical text does not compile:\n{}\n{text}", ex.key, ds.iter().map(|d| d.render(&text, "canonical")).collect::<Vec<_>>().join("\n")));
                continue;
            }
        };
        let matched = reconcile(second, &first);
        if matched != first {
            let (a, b) = (first.to_json(), matched.to_json());
            let line = a.lines().zip(b.lines()).position(|(x, y)| x != y).unwrap_or(0);
            let ctx = |s: &str| s.lines().skip(line.saturating_sub(3)).take(8).collect::<Vec<_>>().join("\n");
            failures.push(format!("{}: IR differs after the round trip\n--- original\n{}\n--- reprinted\n{}\n{text}", ex.key, ctx(&a), ctx(&b)));
            continue;
        }
        let again = format(&matched);
        if again != text {
            failures.push(format!("{}: printing is not idempotent", ex.key));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The canonical form of RP-01's model, as the formatter prints it.
#[test]
fn canonical_form_of_rp01() {
    let ex = examples::all().into_iter().find(|e| e.key == "rp01").unwrap();
    let text = format(&compile(&ex.source).unwrap().doc);
    let model: String = text.split("\n\n").take(2).collect::<Vec<_>>().join("\n\n");
    let expected = r#"space Plane = euclidean(2)

model Projectile in Plane {
  param {
    g:     Acceleration  = 9.81 m/s^2
    k:     Quantity<1/L> = 0   where k >= 0
    speed: Velocity      = 20 m/s   where speed > 0 m/s  symbol "v"
    angle: Angle         = 45 deg   in (0 deg, 90 deg)  symbol "θ"  unit deg
  }"#;
    assert!(model.starts_with(expected), "{model}");
    assert!(text.contains("  process gravity { flow der(vel) += if flying then (0, -g) else 0 }"), "{text}");
    assert!(text.contains("  event landed on falling(pos.y) {\n    set flying = false\n    set vel = 0\n  }"), "{text}");
    assert!(text.contains("beat b2 { run rate 1 until landed }") || !text.contains("timeline"), "{text}");
}

/// A tool rename keeps the identity: the reprinted text uses the new name everywhere, and
/// lowering it again against the renamed IR gives the same identities.
#[test]
fn tool_rename_keeps_identity() {
    let ex = examples::all().into_iter().find(|e| e.key == "rp08").unwrap();
    let doc = compile(&ex.source).unwrap().doc;
    let renamed = rename(&doc, "Projectile.speed", "v0").unwrap();
    let text = format(&renamed);
    assert!(text.contains("    v0:    Velocity") && text.contains("v0^2 * sin(2 * angle) / g"), "{text}");
    assert!(!text.contains("speed:") && !text.contains("speed^2") && !text.contains("speed *"), "{text}");
    let again = reconcile(compile(&text).unwrap().doc, &renamed);
    assert_eq!(again, renamed);
    let b = again.models[0].binding("Projectile.speed").unwrap();
    assert_eq!(b.name, "v0", "identity kept, name changed");
    assert!(diff(&doc, &again).removed.is_empty());
    // A second element may now take the old name; it gets a fresh identity.
    let text2 = text.replace("    g:", "    speed: Velocity = 1 m/s\n    g:");
    let third = reconcile(compile(&text2).unwrap().doc, &renamed);
    let d = diff(&renamed, &third);
    assert_eq!(d.added, vec!["Projectile.speed~2".to_string()]);
    assert!(d.removed.is_empty());
    // Renaming to a taken name is refused.
    assert!(rename(&doc, "Projectile.speed", "angle").is_err());
}

/// A rename made in plain text is a removal and an addition, reported by `diff`.
#[test]
fn plain_text_rename_is_reported() {
    let ex = examples::all().into_iter().find(|e| e.key == "rp06").unwrap();
    let doc = compile(&ex.source).unwrap().doc;
    let edited = ex.source.replace("a: Real", "amp: Real").replace("a * x^2", "amp * x^2").replace("slider(a,", "slider(amp,").replace("propose a =", "propose amp =").replace("a_now = a ", "a_now = amp ");
    let new = reconcile(compile(&edited).unwrap().doc, &doc);
    let d = diff(&doc, &new);
    // The parameter and its range constraint, whose identity follows the parameter's name.
    assert_eq!(d.removed, vec!["QuadraticDemo.a".to_string(), "QuadraticDemo.constraint.a_range".to_string()]);
    assert_eq!(d.added, vec!["QuadraticDemo.amp".to_string(), "QuadraticDemo.constraint.amp_range".to_string()]);
    // Every other element kept its identity; the new one comes last.
    assert_eq!(new.models[0].bindings.iter().map(|b| b.id.as_str()).collect::<Vec<_>>(), vec!["QuadraticDemo.f", "QuadraticDemo.amp"], "a new element is appended (IR-1.7)");
}
