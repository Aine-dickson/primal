//! MathML for the browser medium (D-034, D-046): a formula's math box tree
//! (`prismal_present::math`) written as a display `<math>` element, which browsers typeset.
//! Other media draw the same formula from its layout in the frame description.

use prismal_host::Instance;
use prismal_present::math::{MathBox, OpKind};
use serde_json::{json, Value as Json};

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn ml(b: &MathBox) -> String {
    match b {
        MathBox::Row { items } => format!("<mrow>{}</mrow>", items.iter().map(ml).collect::<String>()),
        MathBox::Ident { text, binding, upright } => {
            let variant = if *upright && text.chars().count() == 1 { " mathvariant=\"normal\"" } else { "" };
            let data = binding.as_ref().map(|b| format!(" data-binding=\"{}\"", esc(b))).unwrap_or_default();
            format!("<mi{variant}{data}>{}</mi>", esc(text))
        }
        MathBox::Number { text } => format!("<mn>{}</mn>", esc(text)),
        MathBox::Op { kind: OpKind::Invisible, .. } => "<mo>&#x2062;</mo>".into(),
        MathBox::Op { text, .. } => format!("<mo>{}</mo>", esc(text)),
        MathBox::Text { text } => format!("<mtext>{}</mtext>", esc(text)),
        MathBox::Space { em } => format!("<mspace width=\"{em}em\"/>"),
        MathBox::Frac { num, den } => format!("<mfrac>{}{}</mfrac>", ml(num), ml(den)),
        MathBox::Sup { base, sup } => format!("<msup>{}{}</msup>", ml(base), ml(sup)),
        MathBox::Sub { base, sub } => format!("<msub>{}{}</msub>", ml(base), ml(sub)),
        MathBox::Sqrt { body } => format!("<msqrt>{}</msqrt>", ml(body)),
        MathBox::Fenced { open, close, body } => {
            let fence = |c: &str| if c.is_empty() { String::new() } else { format!("<mo fence=\"true\">{}</mo>", esc(c)) };
            format!("<mrow>{}{}{}</mrow>", fence(open), ml(body), fence(close))
        }
        MathBox::Cases { rows } => {
            let rows: String = rows.iter().map(|(v, c)| format!("<mtr><mtd columnalign=\"left\">{}</mtd><mtd columnalign=\"left\">{}</mtd></mtr>", ml(v), ml(c))).collect();
            format!("<mrow><mo>{{</mo><mtable>{rows}</mtable></mrow>")
        }
    }
}

/// A box tree as a display `<math>` element.
pub fn math(b: &MathBox) -> String {
    format!("<math display=\"block\">{}</math>", ml(b))
}

/// Adds `mathml` to the formula and equation representations of a frame description.
pub fn add_mathml(frame: &mut Json, inst: &Instance) {
    let mut each = |r: &mut Json| {
        if matches!(r["shape"].as_str(), Some("formula" | "equation")) {
            if let Some(b) = r["id"].as_str().and_then(|id| inst.math(id)) {
                r["mathml"] = json!(math(&b));
            }
        }
    };
    for v in frame.get_mut("views").and_then(Json::as_array_mut).into_iter().flatten() {
        v.get_mut("reps").and_then(Json::as_array_mut).into_iter().flatten().for_each(&mut each);
    }
    frame.get_mut("overlay").and_then(Json::as_array_mut).into_iter().flatten().for_each(&mut each);
}
