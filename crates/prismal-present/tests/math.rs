//! Formula layout for every medium (D-046): the layout carried by formula and equation
//! frames, checked for its geometry. The layouts are also written as SVG to
//! `target/math-layout.svg` for inspection.

mod common;
use common::*;
use prismal_present::frame::{Frame, Shape};
use prismal_present::interact::Interactive;
use prismal_present::math::{MathItem, MathLayout};
use prismal_runtime::Config;

const SRC: &str = "space Plane = euclidean(2)
model Shapes in Plane {
  param {
    g: Acceleration = 9.81 m/s^2
    h: Length = 10 m
    v: Velocity = 20 m/s symbol \"v\"
    angle: Angle = 45 deg symbol \"θ\"
    moon: Boolean = false
    m: Mass = 1 kg
    L: Length = 1 m
  }
  state { ω: Quantity<1/T> = 0 }
  derived {
    grav: Acceleration = if moon then 1.62 m/s^2 else 9.81 m/s^2
    energy: Energy = 0.5 * m * (L * ω)^2 + m * g * L * (1 - cos(angle))
    reach: Length = v^2 * sin(2 * angle) / g
  }
  flow { der(ω) = -(g / L) * sin(angle) }
  equation fall: sqrt(2 * h / g) == sqrt(2 * h / g)
}
presentation Formulas for Shapes {
  panel p {
    formula(grav)
    formula(energy, live: true)
    formula(reach)
    formula(\"t\", sqrt(2 * h / g))
    formula(\"T\", 2 * pi * sqrt(L / g) * (1 + (h / L)^2) + abs(h - L) / (1 m/s))
    equation(fall)
  }
}
";

fn layouts(f: &Frame) -> Vec<(String, MathLayout)> {
    f.reps()
        .filter_map(|r| match &r.shape {
            Shape::Formula { layout, .. } | Shape::Equation { layout, .. } => Some((r.text.clone(), layout.clone())),
            _ => None,
        })
        .collect()
}

fn texts(l: &MathLayout) -> Vec<(&str, f64, f64, f64)> {
    l.items
        .iter()
        .filter_map(|i| match i {
            MathItem::Text { text, x, y, size, .. } => Some((text.as_str(), *x, *y, *size)),
            _ => None,
        })
        .collect()
}

fn find<'a>(l: &'a MathLayout, s: &str) -> (&'a str, f64, f64, f64) {
    *texts(l).iter().find(|t| t.0 == s).unwrap_or_else(|| panic!("no `{s}` in {:?}", texts(l)))
}

/// A minimal SVG drawing of layouts, for inspection only.
fn svg(ls: &[(String, MathLayout)]) -> String {
    let em = 32.0;
    let mut y = 20.0;
    let mut body = String::new();
    for (_, l) in ls {
        y += l.ascent * em;
        let x0 = 20.0;
        for it in &l.items {
            match it {
                MathItem::Text { text, x, y: ty, size, width, italic, .. } => body += &format!(
                    "<text x='{}' y='{}' font-size='{}' textLength='{}' lengthAdjust='spacingAndGlyphs' font-family='serif'{}>{}</text>",
                    x0 + x * em,
                    y + ty * em,
                    size * em,
                    width * em,
                    if *italic { " font-style='italic'" } else { "" },
                    text.replace('&', "&amp;").replace('<', "&lt;")
                ),
                MathItem::Rule { x, y: ry, w, h } => body += &format!("<rect x='{}' y='{}' width='{}' height='{}'/>", x0 + x * em, y + ry * em, w * em, h * em),
                MathItem::Path { points, stroke } => {
                    let pts: Vec<String> = points.iter().map(|p| format!("{},{}", x0 + p[0] * em, y + p[1] * em)).collect();
                    body += &format!("<polyline fill='none' stroke='black' stroke-width='{}' points='{}'/>", stroke * em, pts.join(" "));
                }
            }
        }
        y += l.descent * em + 24.0;
    }
    format!("<svg xmlns='http://www.w3.org/2000/svg' width='900' height='{}' style='background:white'>{body}</svg>", y)
}

#[test]
fn formula_layouts() {
    let prog = program(SRC);
    let i = Interactive::new(&prog, "Formulas", Config::until(1.0)).unwrap();
    let ls = layouts(&i.frame());
    assert_eq!(ls.len(), 6);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
    let _ = std::fs::write(dir.join("math-layout.svg"), svg(&ls));

    for (text, l) in &ls {
        assert!(l.width > 0.0 && l.ascent > 0.0 && l.descent >= 0.0, "{text}");
        // Every item lies within the declared extent.
        for it in &l.items {
            if let MathItem::Text { x, y, size, width, .. } = it {
                assert!(*x >= -1e-9 && x + width <= l.width + 1e-9, "{text}: {it:?}");
                assert!(y - 0.72 * size >= -l.ascent - 1e-9 && y + 0.22 * size <= l.descent + 1e-9, "{text}: {it:?}");
            }
        }
    }

    // grav = { 1.62 m/s² if moon, 9.81 m/s² otherwise }: a brace and two rows.
    let grav = &ls[0].1;
    assert!(grav.items.iter().any(|i| matches!(i, MathItem::Path { points, .. } if points.len() == 7)), "brace");
    let (_, _, y1, _) = find(grav, "1.62");
    let (_, _, y2, _) = find(grav, "9.81");
    assert!(y2 > y1 + 0.8, "second row below the first");
    assert_eq!(find(grav, "moon").3, 1.0);

    // energy: a power raised and smaller; symbols carry their bindings (PK-6.5).
    let energy = &ls[1].1;
    let (_, _, y, size) = find(energy, "2");
    assert!(y < -0.3 && size < 1.0, "exponent raised and smaller");
    let bound = energy.items.iter().any(|i| matches!(i, MathItem::Text { text, binding: Some(b), italic: true, .. } if text == "ω" && b == "Shapes.ω"));
    assert!(bound, "ω is linked to its binding");
    assert!(texts(energy).iter().any(|t| t.0 == "cos"), "function names upright");

    // reach = v² sin(2θ) / g: numerator above the bar, denominator below.
    let reach = &ls[2].1;
    let bar = reach.items.iter().find_map(|i| match i {
        MathItem::Rule { y, h, .. } => Some((*y, *h)),
        _ => None,
    });
    let (bar_y, bar_h) = bar.expect("a fraction bar");
    assert!(find(reach, "v").2 < bar_y && find(reach, "g").2 - 0.72 * find(reach, "g").3 > bar_y + bar_h, "numerator above, denominator below");

    // t = √(2h/g): a radical sign whose overline covers its body.
    let t = &ls[3].1;
    let sign = t.items.iter().find_map(|i| match i {
        MathItem::Path { points, .. } if points.len() == 5 => Some(points.clone()),
        _ => None,
    });
    let sign = sign.expect("a radical sign");
    let right = sign.last().unwrap()[0];
    assert!(texts(t).iter().filter(|x| x.0 != "t" && x.0 != "=").all(|x| x.1 < right), "overline covers the body");

    // π, grown parentheses and absolute value bars.
    let tt = &ls[4].1;
    assert!(texts(tt).iter().any(|t| t.0 == "π"));
    // Parentheses around a fraction grow with it: drawn as paths, not glyphs; so do the
    // bars of `abs`.
    let paths = tt.items.iter().filter(|i| matches!(i, MathItem::Path { .. })).count();
    assert!(paths >= 4 && !texts(tt).iter().any(|t| t.0 == "("), "{:?}", tt.items);

    // The equation: both sides typeset.
    assert_eq!(texts(&ls[5].1).iter().filter(|t| t.0 == "=").count(), 1);
    assert_eq!(texts(&ls[5].1).iter().filter(|t| t.0 == "2").count(), 2);
}
