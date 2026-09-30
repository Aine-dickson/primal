//! Prismal presentation kernel (docs/spec/03-presentation-kernel.md).
//!
//! A presentation observes a model's runs, projects them into representations arranged in
//! views, lets a learner act on the model through declared inverses, and directs a lesson
//! run through an explanation timeline. Every presented instant is described by a
//! medium-independent frame description (PK-12.1), which a renderer turns into output.
//!
//! - `check`: static checks of a presentation against its model (PK-E diagnostics).
//! - `data`: observations of a run (PK section 3).
//! - `expect`: headless evaluation of run cases and their expectations (PK section 4).
//! - `frame`: projection and frame descriptions (PK sections 5 to 7, 11, 12).
//! - `interact`: controls, direct manipulation, keyboard operation, undo (PK section 10).
//! - `timeline`: the explanation timeline and its lesson run (PK sections 8, 9).
//! - `text`: text alternatives and formula text (PK-6.5, PK-11.1).

pub mod check;
pub mod data;
pub mod expect;
pub mod frame;
pub mod interact;
pub mod math;
pub mod text;
pub mod timeline;

use prismal_ir::present::{Presentation, RunCase};
use prismal_ir::{Document, Expr, Id};
use prismal_kernel::{check_model, compile_expr, CModel, Ctx, Value};
use prismal_runtime::{Config, SolverConfig};

/// A diagnostic about a presentation or a run case.
#[derive(Clone, Debug, PartialEq)]
pub struct PDiag {
    pub code: &'static str,
    pub message: String,
    /// The IR element concerned.
    pub element: Id,
}

impl std::fmt::Display for PDiag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}: {}", self.code, self.element, self.message)
    }
}

/// A checked document: its models compiled, its presentations checked.
pub struct Program {
    pub doc: Document,
    pub models: Vec<CModel>,
}

impl Program {
    /// Checks every model with the kernel and every presentation against its model.
    pub fn new(doc: Document) -> Result<Program, Vec<PDiag>> {
        let mut diags = vec![];
        let mut models = vec![];
        for m in &doc.models {
            match check_model(&doc.spaces, m) {
                Ok(cm) => models.push(cm),
                Err(ds) => diags.extend(ds.into_iter().map(|d| PDiag { code: d.code, message: d.message, element: d.element })),
            }
        }
        if !diags.is_empty() {
            return Err(diags);
        }
        let prog = Program { doc, models };
        for p in &prog.doc.presentations {
            diags.extend(check::check_presentation(prog.model(&p.model), p));
        }
        if diags.is_empty() {
            Ok(prog)
        } else {
            Err(diags)
        }
    }

    pub fn model(&self, id: &str) -> &CModel {
        self.models.iter().find(|m| m.ir.id == id).unwrap_or_else(|| panic!("unknown model {id}"))
    }

    pub fn presentation(&self, name: &str) -> &Presentation {
        self.doc.presentations.iter().find(|p| p.name == name || p.id == name).unwrap_or_else(|| panic!("unknown presentation {name}"))
    }

    pub fn case(&self, name: &str) -> &RunCase {
        self.doc.runs.iter().find(|r| r.name == name || r.id == name).unwrap_or_else(|| panic!("unknown run {name}"))
    }
}

/// Evaluates an expression that reads no state (a literal, a time such as `t0 + 5 s`).
/// Instants are seconds with `t0 = 0`.
pub fn constant(cm: &CModel, e: &Expr) -> Result<Value, String> {
    let mut reads = false;
    e.walk(&mut |x| reads |= matches!(x, Expr::Ref { .. } | Expr::Der { .. }));
    if reads {
        return Err("the expression reads bindings of the model; it is not a constant".into());
    }
    let (c, _) = compile_expr(cm, e, None).map_err(|d| d.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("; "))?;
    c.eval(&Ctx { vals: &[], der: None, t: 0.0, t0: 0.0, args: &[] }).map_err(|s| s.cause)
}

/// A constant number (a quantity in coherent SI units, or an instant in seconds).
pub fn number(cm: &CModel, e: &Expr) -> Result<f64, String> {
    match constant(cm, e)? {
        Value::Num(x) => Ok(x),
        v => Err(format!("{v} is not a number")),
    }
}

/// The components of a value, for comparison: numbers, coordinates, Booleans as 0 and 1.
pub fn flat(v: &Value) -> Vec<f64> {
    match v {
        Value::Bool(b) => vec![if *b { 1.0 } else { 0.0 }],
        Value::Num(x) => vec![*x],
        Value::Vec(a) | Value::Point(a) => a.as_slice().to_vec(),
        Value::Tuple(items) => items.iter().flat_map(flat).collect(),
        Value::Case(c) => vec![*c as f64],
        Value::Func(_) => vec![],
    }
}

/// Default simulated span of a lesson run whose case gives no end (PK-9.5).
pub const LESSON_HORIZON: f64 = 60.0;

/// The run configuration of a case (RC section 16 defaults, overridden by the case).
pub fn config(cm: &CModel, case: &RunCase, default_end: f64) -> Result<Config, String> {
    let end = match &case.end {
        Some(e) => number(cm, e)?,
        None => default_end,
    };
    let mut cfg = Config::until(end);
    let get = |e: &Option<Expr>| e.as_ref().map(|x| number(cm, x)).transpose();
    let (h, rtol, atol) = (get(&case.config.h)?, get(&case.config.rtol)?, get(&case.config.atol)?);
    cfg.solver = match case.config.solver.as_deref() {
        Some("rk4") => SolverConfig::Rk4 { h },
        _ => SolverConfig::Dopri5 { rtol: rtol.unwrap_or(1e-6), atol: atol.unwrap_or(1e-9), h_max: f64::INFINITY },
    };
    for o in &case.params {
        cfg = cfg.param(&o.binding, o.value.clone());
    }
    Ok(cfg)
}
