//! Formulas typeset as MathML from the IR (D-034): the web medium's rendering of a formula
//! representation. Structure follows `prismal_present::text::print` (same precedences and
//! display symbols); fractions, powers, roots and components use two-dimensional layout.

use prismal_ir::{BinOp, Builtin, Expr, Func, Type};
use prismal_kernel::CModel;
use prismal_present::text::{fmt_num, symbol};

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn mi(s: &str) -> String {
    format!("<mi>{}</mi>", esc(s))
}

fn mo(s: &str) -> String {
    format!("<mo>{}</mo>", esc(s))
}

fn row(parts: &[String]) -> String {
    format!("<mrow>{}</mrow>", parts.concat())
}

fn parens(inner: String) -> String {
    row(&[r#"<mo stretchy="false">(</mo>"#.to_string(), inner, r#"<mo stretchy="false">)</mo>"#.to_string()])
}

fn prec(e: &Expr) -> u8 {
    match e {
        Expr::If { .. } | Expr::Otherwise { .. } => 0,
        Expr::Bin { bin, .. } => match bin {
            BinOp::Or => 1,
            BinOp::And => 2,
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => 3,
            BinOp::Add | BinOp::Sub => 4,
            // A fraction is typeset as one block.
            BinOp::Div => 8,
            BinOp::Mul => 5,
            BinOp::Pow => 7,
        },
        Expr::Neg { .. } | Expr::Not { .. } => 6,
        Expr::Num { num, unit } if *num < 0.0 || unit.is_some() => 6,
        _ => 8,
    }
}

/// A number with its unit: `9.81 m/s^2` as `9.81 m/s²`.
fn number(num: f64, unit: Option<&str>) -> String {
    let n = format!("<mn>{}</mn>", esc(&fmt_num(num)));
    match unit {
        Some(u) => row(&[n, "<mspace width=\"0.2em\"/>".into(), unit_ml(u)]),
        None => n,
    }
}

fn unit_ml(u: &str) -> String {
    let parts: Vec<String> = u
        .split('/')
        .map(|p| match p.split_once('^') {
            Some((b, e)) => format!("<msup><mi mathvariant=\"normal\">{}</mi><mn>{}</mn></msup>", esc(b), esc(e)),
            None => format!("<mi mathvariant=\"normal\">{}</mi>", esc(p)),
        })
        .collect();
    row(&[parts.join(&mo("/"))])
}

/// An expression as MathML content of a `<math>` element; `params` names lambda parameters.
pub fn expr(e: &Expr, cm: &CModel, params: &[String]) -> String {
    let p = |x: &Expr| expr(x, cm, params);
    let wrap = |x: &Expr, min: u8| if prec(x) < min { parens(p(x)) } else { p(x) };
    match e {
        Expr::Num { num, unit } => number(*num, unit.as_ref().map(|u| u.text.as_str())),
        Expr::Bool { bool } => format!("<mtext>{bool}</mtext>"),
        Expr::Case { case } => format!("<mtext>{}</mtext>", esc(case)),
        Expr::Ref { r#ref } => format!("<mi data-binding=\"{}\">{}</mi>", esc(r#ref), esc(&symbol(cm, r#ref))),
        Expr::Param { param } => mi(&params.get(*param).cloned().unwrap_or_else(|| format!("x{}", param + 1))),
        Expr::Builtin { builtin } => match builtin {
            Builtin::T => mi("t"),
            Builtin::T0 => "<msub><mi>t</mi><mn>0</mn></msub>".into(),
            Builtin::Elapsed => "<mtext>elapsed</mtext>".into(),
        },
        Expr::Const { .. } => mi("π"),
        Expr::Origin { .. } => "<mi mathvariant=\"normal\">O</mi>".into(),
        Expr::Der { der } => format!("<mfrac><mrow><mi>d</mi>{}</mrow><mrow><mi>d</mi><mi>t</mi></mrow></mfrac>", mi(&symbol(cm, der))),
        Expr::Neg { neg } => row(&[mo("−"), wrap(neg, 6)]),
        Expr::Not { not } => row(&[mo("¬"), wrap(not, 6)]),
        Expr::Norm { norm } => row(&[mo("|"), p(norm), mo("|")]),
        Expr::Bin { bin: BinOp::Div, l, r } => format!("<mfrac>{}{}</mfrac>", p(l), p(r)),
        Expr::Bin { bin: BinOp::Pow, l, r } => format!("<msup>{}{}</msup>", wrap(l, 8), p(r)),
        Expr::Bin { bin, l, r } => {
            let me = prec(e);
            let (ls, rs) = match bin {
                BinOp::Sub => (wrap(l, me), wrap(r, me + 1)),
                _ => (wrap(l, me), wrap(r, me)),
            };
            let op = match bin {
                BinOp::Add => mo("+"),
                BinOp::Sub => mo("−"),
                // Juxtapose, except before a number.
                BinOp::Mul if matches!(**r, Expr::Num { .. }) => mo("·"),
                BinOp::Mul => "<mo>&#x2062;</mo>".into(),
                BinOp::And => mo("∧"),
                BinOp::Or => mo("∨"),
                BinOp::Eq => mo("="),
                BinOp::Ne => mo("≠"),
                BinOp::Lt => mo("<"),
                BinOp::Le => mo("≤"),
                BinOp::Gt => mo(">"),
                BinOp::Ge => mo("≥"),
                BinOp::Div | BinOp::Pow => unreachable!(),
            };
            row(&[ls, op, rs])
        }
        Expr::Call { call: Func::Sqrt, args } => format!("<msqrt>{}</msqrt>", args.iter().map(p).collect::<String>()),
        Expr::Call { call: Func::Abs, args } => row(&[mo("|"), args.iter().map(p).collect(), mo("|")]),
        Expr::Call { call, args } => {
            let name = match call {
                Func::Sin => "sin",
                Func::Cos => "cos",
                Func::Tan => "tan",
                Func::Exp => "exp",
                Func::Log => "log",
                Func::Min => "min",
                Func::Max => "max",
                Func::Atan2 => "atan2",
                Func::Sqrt | Func::Abs => unreachable!(),
            };
            row(&[mi(name), "<mo>&#x2061;</mo>".into(), parens(args.iter().map(p).collect::<Vec<_>>().join(&mo(",")))])
        }
        Expr::Apply { apply, args } => row(&[wrap(apply, 8), "<mo>&#x2061;</mo>".into(), parens(args.iter().map(p).collect::<Vec<_>>().join(&mo(",")))]),
        Expr::If { r#if, then, r#else } => row(&[
            mo("{"),
            format!("<mtable><mtr><mtd>{}</mtd><mtd><mtext>if&#xA0;</mtext>{}</mtd></mtr><mtr><mtd>{}</mtd><mtd><mtext>otherwise</mtext></mtd></mtr></mtable>", p(then), p(r#if), p(r#else)),
        ]),
        Expr::Tuple { tuple } => parens(tuple.iter().map(p).collect::<Vec<_>>().join(&mo(","))),
        Expr::Comp { comp, axis } => {
            let space = match &**comp {
                Expr::Ref { r#ref } => cm.ir.binding(r#ref).and_then(|b| match &b.ty {
                    Type::Point { space } | Type::Vector { space, .. } => Some(space.clone()),
                    _ => None,
                }),
                _ => cm.ir.default_space.clone(),
            };
            let names = space.and_then(|s| cm.spaces.iter().find(|x| x.id == s)).map(|s| s.axes.clone()).unwrap_or_else(|| vec!["x".into(), "y".into(), "z".into()]);
            let axis = names.get(*axis).cloned().unwrap_or_else(|| axis.to_string());
            format!("<msub>{}{}</msub>", wrap(comp, 8), mi(&axis))
        }
        Expr::Lambda { lambda } => expr(&lambda.body, cm, &lambda.names),
        Expr::Otherwise { otherwise, default } => row(&[p(otherwise), "<mtext>&#xA0;otherwise&#xA0;</mtext>".into(), p(default)]),
    }
}

/// A formula `lhs = rhs` as a display `<math>` element. `lhs` is the representation's
/// label: a symbol, `f(x)`, or a quoted name such as `R`.
pub fn formula(lhs: &str, rhs: &Expr, cm: &CModel, params: &[String]) -> String {
    let left = match lhs.split_once('(') {
        Some((f, rest)) => {
            let args = rest.trim_end_matches(')').split(',').map(|a| mi(a.trim())).collect::<Vec<_>>().join(&mo(","));
            row(&[mi(f), "<mo>&#x2061;</mo>".into(), parens(args)])
        }
        None => mi(lhs),
    };
    format!("<math display=\"block\">{}</math>", row(&[left, mo("="), expr(rhs, cm, params)]))
}

/// A model equation `lhs = rhs` as a display `<math>` element (PK-6.5).
pub fn equation(lhs: &Expr, rhs: &Expr, cm: &CModel) -> String {
    format!("<math display=\"block\">{}</math>", row(&[expr(lhs, cm, &[]), mo("="), expr(rhs, cm, &[])]))
}
