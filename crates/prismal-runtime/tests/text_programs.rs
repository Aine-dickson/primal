//! The reference programs from their text (docs/spec/reference-programs/): the working
//! syntax lowers to exactly the IR the builders in `common` produce, the diagnostic
//! variants are rejected with their named codes when written as text edits.

mod common;
use common::*;
use prismal_ir::*;
use prismal_syntax::{check, compile, markdown_blocks, markdown_source, Compiled};
use std::path::PathBuf;

fn docs() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs")
}

fn rp_doc(prefix: &str) -> String {
    let dir = docs().join("spec/reference-programs");
    let path = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.file_name().unwrap().to_string_lossy().starts_with(prefix))
        .unwrap_or_else(|| panic!("no document {prefix}"));
    std::fs::read_to_string(path).unwrap()
}

/// The program text of a reference-program document.
fn rp(prefix: &str) -> String {
    markdown_source(&rp_doc(prefix))
}

/// RP-08: the RP-01 model with RP-08's addition, and RP-08's presentation.
fn rp08() -> String {
    let rp01 = markdown_blocks(&rp_doc("RP-01"));
    let rp08 = markdown_blocks(&rp_doc("RP-08"));
    let model = &rp01[0].1;
    let end = model.rfind("\n}").expect("end of the model");
    let mut src = format!("{}\n{}{}", &model[..end], rp08[0].1, &model[end..]);
    for (_, b) in &rp08[1..] {
        src.push_str(b);
    }
    src
}

fn compiled(src: &str, what: &str) -> Compiled {
    compile(src).unwrap_or_else(|ds| panic!("{}", ds.iter().map(|d| d.render(src, what)).collect::<Vec<_>>().join("\n")))
}

/// Compares two models through their JSON form and reports the first differing lines.
fn assert_same_model(got: &Model, want: &Model, what: &str) {
    if got == want {
        return;
    }
    let (g, w) = (serde_json::to_string_pretty(got).unwrap(), serde_json::to_string_pretty(want).unwrap());
    let diff: Vec<String> = g
        .lines()
        .zip(w.lines())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .take(6)
        .map(|(i, (a, b))| format!("line {i}: text {a}\n         builder {b}"))
        .collect();
    panic!("{what}: lowered model differs from the builder model\n{}", diff.join("\n"));
}

#[test]
fn text_lowers_to_the_builder_models() {
    let cases: Vec<(&str, String, &str, Model)> = vec![
        ("RP-01", rp("RP-01"), "Projectile", projectile(false)),
        ("RP-02", rp("RP-01") + &rp("RP-02"), "Projectile", projectile(false)),
        ("RP-03", rp("RP-03"), "BouncingBall", bouncing_ball(settle())),
        ("RP-04", rp("RP-04"), "Pendulum", pendulum()),
        ("RP-05", rp("RP-05"), "SpringMass", spring_mass()),
        ("RP-06", rp("RP-06"), "QuadraticDemo", quadratic()),
        ("RP-07", rp("RP-07"), "VectorDemo", vectors(|_| {})),
        ("RP-08", rp08(), "Projectile", projectile(true)),
    ];
    for (what, src, model, want) in cases {
        let c = compiled(&src, what);
        assert_same_model(c.model(model).expect("model"), &want, what);
        if want.default_space.is_some() {
            assert_eq!(c.doc.spaces, vec![plane()], "{what}: spaces");
        }
        // The IR survives JSON (D-037), and the kernel accepts it.
        assert_eq!(Document::from_json(&c.doc.to_json()).unwrap(), c.doc, "{what}: JSON round trip");
        if let Err(ds) = check(&src) {
            panic!("{what}: {}", ds.iter().map(|d| d.render(&src, what)).collect::<Vec<_>>().join("\n"));
        }
    }
}

// ---------------------------------------------------------------- diagnostic variants

/// Applies a text edit that must match exactly once and returns the diagnostic codes.
fn variant(src: &str, from: &str, to: &str) -> Vec<&'static str> {
    assert_eq!(src.matches(from).count(), 1, "the edit anchor `{from}` must occur once");
    let edited = src.replacen(from, to, 1);
    match check(&edited) {
        Ok(_) => vec![],
        Err(ds) => ds.iter().map(|d| d.code).collect(),
    }
}

fn assert_code(codes: &[&str], code: &str, what: &str) {
    assert!(codes.contains(&code), "{what}: expected {code}, got {codes:?}");
}

#[test]
fn diagnostic_variants_as_text() {
    // RP-01.D1 is stored in IR form only (the working syntax has no level trigger).
    let p = rp("RP-01");
    assert_code(&variant(&p, "  event apex", "  flow der(vel) += (1, -g)\n  event apex"), "MK-E03", "RP-01.D2");
    assert_code(&variant(&p, "  event apex", "  flow der(vel) = (0, -g)\n  event apex"), "MK-E10", "RP-01.D3");
    assert_code(&variant(&p, "der(pos) = if flying then vel else 0", "der(pos) = if pos.y > 0 m then vel else 0"), "MK-E12", "RP-01.D4");
    assert_code(&variant(&p, "set flying = false;", "set flying = false; set g = 0 m/s^2;"), "MK-E08", "RP-01.D5");

    let b = rp("RP-03");
    let settle_clause = b[b.find("} zeno settle {").unwrap()..].split_inclusive("\n  }").next().unwrap().to_string();
    assert_code(&variant(&b, &settle_clause, "}"), "MK-E16", "RP-03.D1");
    assert_code(&variant(&b, "der(v) = if resting then 0 else -g", "der(v) = if y > 0 m then -g else 0"), "MK-E12", "RP-03.D2");
    assert_code(&variant(&b, "set v = -e * v", "set v = -e * v\n    set v = 0 m/s"), "MK-E19", "RP-03.D3");
    // RP-03.V1: the model variant with `zeno stop` is valid.
    let v1 = b.replacen(&settle_clause, "} zeno stop", 1);
    let c = compiled(&v1, "RP-03.V1");
    let bounce = c.model("BouncingBall").unwrap().event_by_name("bounce").unwrap();
    assert_eq!(bounce.zeno.as_ref().unwrap().policy, ZenoPolicy::Stop, "RP-03.V1");

    let pd = rp("RP-04");
    assert_code(&variant(&pd, "θ: Angle         = θ0", "θ: Length        = 10 cm"), "MK-E02", "RP-04.D1");
    assert_code(&variant(&pd, "pivot + L * (sin(θ), -cos(θ))", "pivot + (sin(θ), -cos(θ))"), "MK-E01", "RP-04.D2");

    let s = rp("RP-05");
    assert_code(&variant(&s, "T == 2π * sqrt(m / k)", "T == 2π * sqrt(k / m)"), "MK-E01", "RP-05.D1");
    assert_code(&variant(&s, "0.5 * k * x^2", "0.5 * k * x"), "MK-E01", "RP-05.D2");
    assert_code(&variant(&s, "der(v) = -(k / m) * x", "der(v) = -(k / m) * x + 1"), "MK-E03", "RP-05.D3");

    let v = rp("RP-07");
    let mid = "    mid:    Point          = A + u";
    assert_code(&variant(&v, mid, &format!("{mid}\n    bad: Point = A + B")), "MK-E04", "RP-07.D1");
    assert_code(&variant(&v, mid, &format!("{mid}\n    bad: Point = 2 * A")), "MK-E04", "RP-07.D2");
    assert_code(&variant(&v, mid, &format!("{mid}\n    bad: Vector<Length> = u + (3, 0)")), "MK-E03", "RP-07.D3");
    let v4 = v.replacen("  derived {", "  param { vel: Vector<Velocity> = (1 m/s, 0 m/s) }\n  derived {", 1);
    assert_code(&variant(&v4, mid, &format!("{mid}\n    bad: Vector<Length> = u + vel")), "MK-E01", "RP-07.D4");
    let v5 = format!("space Board = euclidean(2)\n{}", v.replacen("  derived {", "  param { z: Vector<Board, Length> = (1 m, 0 m) }\n  derived {", 1));
    assert_code(&variant(&v5, mid, &format!("{mid}\n    bad: Vector<Length> = u + z")), "MK-E05", "RP-07.D5");
    let ok = variant(&v, mid, &format!("{mid}\n    ok: Vector<Length> = u + (0, 0)\n    ok2: Vector<Length> = u + 0"));
    assert!(ok.is_empty(), "RP-07.D6 must be accepted: {ok:?}");
}

#[test]
fn kernel_diagnostics_point_at_the_declaration() {
    let p = rp("RP-01");
    let bad = p.replacen("der(pos) = if flying then vel else 0", "der(pos) = if pos.y > 0 m then vel else 0", 1);
    let line = bad.lines().position(|l| l.contains("if pos.y > 0 m")).unwrap() as u32 + 1;
    let ds = check(&bad).unwrap_err();
    let d = ds.iter().find(|d| d.code == "MK-E12").unwrap();
    assert_eq!(d.span.line, line, "{}", d.render(&bad, "RP-01"));
    assert_eq!(d.element.as_deref(), Some("Projectile.flow.1"));
}

// The cases written in the working syntax are run by `prismal-present` (tests/cases.rs).
