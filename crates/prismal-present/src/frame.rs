//! Projection and frame descriptions (PK sections 5 to 7, 11 and 12).
//!
//! A frame description lists, for one presented instant, the visible representations with
//! their resolved properties in view coordinates, their text alternatives, and the
//! captions and announcements due (PK-12.1). It is medium-independent and serializable.
//!
//! View coordinates: in a spatial view, pixels with the model origin at `(0, 0)` and `y`
//! pointing down on screen (a view with `y: up` negates model `y`); in a plot view, the
//! plot's own coordinates; in a panel, none.

use crate::text::{fmt_binding, fmt_num, fmt_value, print, symbol};
use crate::{constant, number, PDiag};
use prismal_ir::build::{num, origin, tuple};
use prismal_ir::present::{Arg, Presentation, Rep, View, ViewKind};
use prismal_ir::{Dim, Expr, Id, Lambda, Role, Type};
use prismal_kernel::{compile_expr, CExpr, CModel, Ctx, Value};
use prismal_runtime::Run;
use serde::Serialize;

// ---------------------------------------------------------------- frame description

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Frame {
    /// Presentation time (PK-8.1).
    pub time: f64,
    /// The run shown (`lesson`, `branch 1`, `session`).
    pub run: String,
    /// The simulation instant shown by every view (PK-7.5).
    pub t: f64,
    pub views: Vec<ViewFrame>,
    /// Representations shown over the presentation, outside any view.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub overlay: Vec<RepFrame>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub captions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub announcements: Vec<String>,
}

impl Frame {
    /// Every representation in the frame, views first.
    pub fn reps(&self) -> impl Iterator<Item = &RepFrame> {
        self.views.iter().flat_map(|v| v.reps.iter()).chain(self.overlay.iter())
    }
    pub fn rep(&self, id_or_name: &str) -> Option<&RepFrame> {
        self.reps().find(|r| r.id == id_or_name || r.name.as_deref() == Some(id_or_name) || r.id.ends_with(&format!(".{id_or_name}")))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ViewFrame {
    pub id: Id,
    pub kind: &'static str,
    pub reps: Vec<RepFrame>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RepFrame {
    pub id: Id,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub shape: Shape,
    /// Text alternative (PK-11.1).
    pub text: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub highlighted: bool,
    /// During a drag preview: whether the proposal passed validation (PK-10.6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "shape", rename_all = "snake_case")]
pub enum Shape {
    Point { at: [f64; 2] },
    Arrow { from: [f64; 2], to: [f64; 2] },
    Segment { from: [f64; 2], to: [f64; 2] },
    Polyline { points: Vec<[f64; 2]> },
    Axes,
    Grid,
    Text { value: String },
    /// Typeset from the IR (PK-6.5, D-034): `rhs` is the symbolic form; each symbol carries
    /// its binding's identity and, when live, its current value.
    Formula { lhs: String, rhs: Expr, symbols: Vec<Symbol> },
    Control { control: String, binding: Id, value: f64, #[serde(skip_serializing_if = "Option::is_none")] min: Option<f64>, #[serde(skip_serializing_if = "Option::is_none")] max: Option<f64>, #[serde(skip_serializing_if = "Option::is_none")] step: Option<f64> },
    /// The source has no value at this instant (PK-3.3).
    Status { status: String },
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Symbol {
    pub binding: Id,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// ---------------------------------------------------------------- views and representations

/// The coordinate context of a view (PK-7.2, PK-7.3).
#[derive(Clone, Debug)]
pub enum ViewCtx {
    Spatial { space: Id, px_per_m: f64, y_up: bool },
    Plot { x: (f64, f64), y: (f64, f64) },
    Panel,
}

impl ViewCtx {
    pub fn of(cm: &CModel, v: &View) -> Result<ViewCtx, PDiag> {
        let err = |code, message: String| PDiag { code, message, element: v.id.clone() };
        match &v.kind {
            ViewKind::Spatial { space, scale, y_up } => {
                if !cm.spaces.iter().any(|s| &s.id == space) {
                    return Err(err("PK-E01", format!("unknown space `{space}` (PK-2.3)")));
                }
                let q = match compile_expr(cm, &scale.quantity, None) {
                    Ok((_, Type::Quantity { dim })) if dim == Dim::length() => number(cm, &scale.quantity).map_err(|m| err("PK-E02", m))?,
                    _ => return Err(err("PK-E04", "a spatial view's scale maps a length to pixels: `1 m -> 40 px` (PK-5.5)".into())),
                };
                Ok(ViewCtx::Spatial { space: space.clone(), px_per_m: scale.px / q, y_up: *y_up })
            }
            ViewKind::Plot { x, y } => {
                let n = |e: &Expr| number(cm, e).map_err(|m| err("PK-E02", m));
                Ok(ViewCtx::Plot { x: (n(&x[0])?, n(&x[1])?), y: (n(&y[0])?, n(&y[1])?) })
            }
            ViewKind::Panel => Ok(ViewCtx::Panel),
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ViewCtx::Spatial { .. } => "spatial",
            ViewCtx::Plot { .. } => "plot",
            ViewCtx::Panel => "panel",
        }
    }

    /// Model coordinates to view coordinates.
    pub fn to_view(&self, p: &[f64]) -> [f64; 2] {
        let (x, y) = (p.first().copied().unwrap_or(0.0), p.get(1).copied().unwrap_or(0.0));
        match self {
            ViewCtx::Spatial { px_per_m, y_up, .. } => [x * px_per_m, if *y_up { -y * px_per_m } else { y * px_per_m }],
            _ => [x, y],
        }
    }

    /// A pointer position in view coordinates as a model expression: a point of the view's
    /// space, or the pair of plot coordinates. This is the value of a gesture (PK-5.6).
    pub fn pointer(&self, p: [f64; 2]) -> Expr {
        match self {
            ViewCtx::Spatial { space, px_per_m, y_up } => {
                let x = p[0] / px_per_m;
                let y = if *y_up { -p[1] } else { p[1] } / px_per_m;
                origin(space) + tuple(vec![num(x, "m"), num(y, "m")])
            }
            _ => tuple(vec![prismal_ir::build::lit(p[0]), prismal_ir::build::lit(p[1])]),
        }
    }
}

/// A representation compiled for its view.
#[derive(Clone, Debug)]
pub struct CRep {
    pub rep: Rep,
    pub kind: CKind,
}

#[derive(Clone, Debug)]
pub enum CKind {
    Marker { pos: CExpr, label: String },
    Arrow { vec: CExpr, ty: Type, from: Option<CExpr>, px_per_unit: f64, label: String },
    Segment { a: CExpr, b: CExpr },
    Graph { f: CExpr, lo: f64, hi: f64, label: String },
    Label { value: CExpr, ty: Type, label: String },
    Formula { lhs: String, rhs: Expr, params: Vec<String>, refs: Vec<(Id, usize)>, live: bool },
    Control { control: String, binding: Id, idx: usize, min: Option<f64>, max: Option<f64>, step: Option<f64> },
    Axes,
    Grid,
}

/// Replaces the gesture parameter `{"param": 0}` by `v`.
pub fn subst(e: &Expr, v: &Expr) -> Expr {
    let s = |x: &Expr| Box::new(subst(x, v));
    match e {
        Expr::Param { param: 0 } => v.clone(),
        Expr::Neg { neg } => Expr::Neg { neg: s(neg) },
        Expr::Not { not } => Expr::Not { not: s(not) },
        Expr::Norm { norm } => Expr::Norm { norm: s(norm) },
        Expr::Bin { bin, l, r } => Expr::Bin { bin: *bin, l: s(l), r: s(r) },
        Expr::Call { call, args } => Expr::Call { call: *call, args: args.iter().map(|a| subst(a, v)).collect() },
        Expr::Apply { apply, args } => Expr::Apply { apply: s(apply), args: args.iter().map(|a| subst(a, v)).collect() },
        Expr::If { r#if, then, r#else } => Expr::If { r#if: s(r#if), then: s(then), r#else: s(r#else) },
        Expr::Tuple { tuple } => Expr::Tuple { tuple: tuple.iter().map(|a| subst(a, v)).collect() },
        Expr::Comp { comp, axis } => Expr::Comp { comp: s(comp), axis: *axis },
        Expr::Otherwise { otherwise, default } => Expr::Otherwise { otherwise: s(otherwise), default: s(default) },
        other => other.clone(),
    }
}

fn source(rep: &Rep, i: usize) -> Option<&Expr> {
    match rep.sources.get(i) {
        Some(Arg::Expr { expr }) => Some(expr),
        _ => None,
    }
}

fn prop_expr<'a>(rep: &'a Rep, name: &str) -> Option<&'a Expr> {
    match rep.prop(name) {
        Some(Arg::Expr { expr }) => Some(expr),
        _ => None,
    }
}

/// Compiles a representation for a view, checking its sources, scales and inverse.
pub fn compile_rep(cm: &CModel, ctx: &ViewCtx, rep: &Rep) -> Result<CRep, Vec<PDiag>> {
    let d = |code: &'static str, message: String| vec![PDiag { code, message, element: rep.id.clone() }];
    let ce = |e: &Expr, exp: Option<&Type>| -> Result<(CExpr, Type), Vec<PDiag>> {
        compile_expr(cm, e, exp).map_err(|ds| d("PK-E02", ds.iter().map(|x| format!("{}: {}", x.code, x.message)).collect::<Vec<_>>().join("; ")))
    };
    let needs = |what: &str| d("PK-E05", format!("`{}` needs {what} (PK-6.3)", rep.kind));
    let label = |e: &Expr| rep.name.clone().unwrap_or_else(|| print(e, cm, &[]));
    let kind = match rep.kind.as_str() {
        "marker" => {
            let e = source(rep, 0).or(prop_expr(rep, "at")).ok_or_else(|| needs("a position"))?;
            let (c, t) = match ctx {
                ViewCtx::Spatial { space, .. } => ce(e, Some(&Type::Point { space: space.clone() }))?,
                ViewCtx::Plot { .. } => ce(e, None)?,
                ViewCtx::Panel => return Err(d("PK-E05", "a marker belongs in a spatial or plot view".into())),
            };
            let ok = match (&t, ctx) {
                (Type::Point { .. }, ViewCtx::Spatial { .. }) => true,
                (Type::Tuple { items }, ViewCtx::Plot { .. }) => items.len() == 2 && items.iter().all(|i| *i == Type::real()),
                _ => false,
            };
            if !ok {
                return Err(needs("a point of the view's space, or a pair of plot coordinates"));
            }
            CKind::Marker { pos: c, label: label(e) }
        }
        "arrow" => {
            let ViewCtx::Spatial { space, px_per_m, .. } = ctx else { return Err(d("PK-E05", "an arrow belongs in a spatial view".into())) };
            let e = source(rep, 0).ok_or_else(|| needs("a vector"))?;
            // A tuple is typed as a vector by context (D-032): the dimension of its first
            // dimensioned component.
            let (mut c, mut t) = ce(e, None)?;
            if let Type::Tuple { items } = &t {
                let dim = items.iter().find_map(|i| match i {
                    Type::Quantity { dim } if !dim.is_none() => Some(*dim),
                    _ => None,
                });
                let want = Type::Vector { space: space.clone(), dim: dim.unwrap_or(Dim::NONE) };
                (c, t) = ce(e, Some(&want))?;
            }
            let Type::Vector { space: vs, dim } = &t else { return Err(needs("a vector")) };
            if vs != space {
                return Err(d("PK-E05", format!("the vector is in space `{vs}`, the view shows `{space}` (D-022)")));
            }
            let px_per_unit = match rep.prop("scale") {
                Some(Arg::Scale { scale }) => {
                    let (_, qt) = ce(&scale.quantity, None)?;
                    if qt != (Type::Quantity { dim: *dim }) {
                        return Err(d("PK-E04", format!("the arrow's scale maps {}, the vector has dimension {dim} (PK-5.5)", crate::check::show_type(&qt))));
                    }
                    scale.px / number(cm, &scale.quantity).map_err(|m| d("PK-E02", m))?
                }
                _ if *dim == Dim::length() => *px_per_m,
                _ => {
                    return Err(d(
                        "PK-E04",
                        format!("an arrow of dimension {dim} needs a declared scale, `scale: 1 {} -> n px` (PK-5.5)", crate::text::unit_text(dim)),
                    ))
                }
            };
            let from = match prop_expr(rep, "from") {
                Some(f) => Some(ce(f, Some(&Type::Point { space: space.clone() }))?.0),
                None => None,
            };
            CKind::Arrow { vec: c, ty: t, from, px_per_unit, label: label(e) }
        }
        "segment" => {
            let ViewCtx::Spatial { space, .. } = ctx else { return Err(d("PK-E05", "a segment belongs in a spatial view".into())) };
            let p = Type::Point { space: space.clone() };
            let a = ce(source(rep, 0).ok_or_else(|| needs("two points"))?, Some(&p))?.0;
            let b = ce(source(rep, 1).ok_or_else(|| needs("two points"))?, Some(&p))?.0;
            CKind::Segment { a, b }
        }
        "function_graph" => {
            let ViewCtx::Plot { x, .. } = ctx else { return Err(d("PK-E05", "a function graph belongs in a plot view".into())) };
            let e = source(rep, 0).ok_or_else(|| needs("a function"))?;
            let (c, t) = ce(e, None)?;
            if t != Type::func(vec![Type::real()], Type::real()) {
                return Err(needs("a function from Real to Real"));
            }
            CKind::Graph { f: c, lo: x.0, hi: x.1, label: label(e) }
        }
        "label" => {
            let e = source(rep, 0).ok_or_else(|| needs("a value"))?;
            let (c, t) = ce(e, None)?;
            CKind::Label { value: c, ty: t, label: label(e) }
        }
        "formula" => {
            let live = matches!(prop_expr(rep, "live"), Some(Expr::Bool { bool: true }));
            let (lhs, rhs, params) = match (rep.sources.first(), rep.sources.get(1)) {
                (Some(Arg::Text { text }), Some(Arg::Expr { expr })) => {
                    ce(expr, None)?;
                    (text.clone(), expr.clone(), vec![])
                }
                (Some(Arg::Expr { expr: Expr::Ref { r#ref } }), None) => {
                    let b = cm.ir.binding(r#ref).ok_or_else(|| d("PK-E01", format!("unknown binding `{ref}`", ref = r#ref)))?;
                    match (&b.role, &b.def) {
                        (Role::Derived, Some(Expr::Lambda { lambda: Lambda { body, names, params } })) => {
                            let names = if names.is_empty() { (1..=params.len()).map(|i| format!("x{i}")).collect() } else { names.clone() };
                            (format!("{}({})", symbol(cm, r#ref), names.join(", ")), (**body).clone(), names)
                        }
                        (Role::Derived, Some(def)) => (symbol(cm, r#ref), def.clone(), vec![]),
                        _ => return Err(d("PK-E05", format!("a formula of `{}` shows its definition; it is not a derived binding (D-034)", b.name))),
                    }
                }
                _ => return Err(needs("a derived binding, or a label and an expression (D-034)")),
            };
            let refs = rhs
                .refs()
                .into_iter()
                .filter_map(|id| cm.index.get(&id).map(|&i| (id, i)))
                .filter(|(_, i)| !matches!(cm.bindings[*i].ty, Type::Function { .. }))
                .collect();
            CKind::Formula { lhs, rhs, params, refs, live }
        }
        "slider" | "number_input" | "toggle" => {
            let Some(Expr::Ref { r#ref }) = source(rep, 0) else { return Err(needs("the binding it controls")) };
            let idx = *cm.index.get(r#ref).ok_or_else(|| d("PK-E01", format!("unknown binding `{ref}`", ref = r#ref)))?;
            if !cm.bindings[idx].intervenable {
                return Err(d("PK-E03", format!("`{}` is not intervenable, so no control can target it (PK-6.4, D-023)", cm.bindings[idx].name)));
            }
            let (min, max) = match rep.prop("range") {
                Some(Arg::Range { lo, hi }) => (Some(number(cm, lo).map_err(|m| d("PK-E02", m))?), Some(number(cm, hi).map_err(|m| d("PK-E02", m))?)),
                _ => (None, None),
            };
            let step = match prop_expr(rep, "step") {
                Some(s) => Some(number(cm, s).map_err(|m| d("PK-E02", m))?),
                None => None,
            };
            CKind::Control { control: rep.kind.clone(), binding: r#ref.clone(), idx, min, max, step }
        }
        "axes" => CKind::Axes,
        "grid" => CKind::Grid,
        other => return Err(d("PK-E06", format!("the representation kind `{other}` is not implemented by the prototype"))),
    };
    // The declared inverse (PK-5.6): targets must be intervenable, proposals must check with
    // the gesture's value in place.
    if let Some(inv) = &rep.inverse {
        let parts: &[&str] = match rep.kind.as_str() {
            "arrow" => &["head"],
            _ => &[],
        };
        if let Some(p) = &inv.part {
            if !parts.contains(&p.as_str()) {
                return Err(d("PK-E05", format!("`{}` has no part `{p}`", rep.kind)));
            }
        }
        for p in &inv.proposals {
            let i = *cm.index.get(&p.target).ok_or_else(|| d("PK-E01", format!("unknown binding `{}`", p.target)))?;
            if !cm.bindings[i].intervenable {
                return Err(d("PK-E03", format!("the inverse proposes `{}`, which is not intervenable (D-023)", cm.bindings[i].name)));
            }
            ce(&subst(&p.value, &ctx.pointer([0.0, 0.0])), Some(&cm.bindings[i].ty))?;
        }
    }
    Ok(CRep { rep: rep.clone(), kind })
}

fn coords(v: &Value) -> Vec<f64> {
    crate::flat(v)
}

/// Projects a compiled representation on a state of a run (PK-5.1, PK-5.2).
pub fn project(cm: &CModel, ctx: &ViewCtx, r: &CRep, run: &Run, vals: &[Value], t: f64) -> RepFrame {
    let eval = |c: &CExpr| run.eval_state(c, vals, t);
    let status = |s: prismal_kernel::Status| (Shape::Status { status: s.cause.clone() }, format!("{}: not available ({})", r.rep.name.clone().unwrap_or(r.rep.kind.clone()), s.cause));
    let (shape, text) = match &r.kind {
        CKind::Marker { pos, label } => match eval(pos) {
            Ok(v) => {
                let c = coords(&v);
                let unit = if matches!(ctx, ViewCtx::Spatial { .. }) { " m" } else { "" };
                (Shape::Point { at: ctx.to_view(&c) }, format!("{label} at x = {}{unit}, y = {}{unit}", fmt_num(c[0]), fmt_num(c[1])))
            }
            Err(s) => status(s),
        },
        CKind::Arrow { vec, ty, from, px_per_unit, label } => {
            let base = match from.as_ref().map(eval) {
                Some(Ok(v)) => coords(&v),
                Some(Err(s)) => {
                    let (sh, tx) = status(s);
                    return RepFrame { id: r.rep.id.clone(), kind: r.rep.kind.clone(), name: r.rep.name.clone(), shape: sh, text: tx, highlighted: false, valid: None };
                }
                None => vec![0.0, 0.0],
            };
            match eval(vec) {
                Ok(v) => {
                    let c = coords(&v);
                    let from_v = ctx.to_view(&base);
                    let ViewCtx::Spatial { y_up, .. } = ctx else { unreachable!() };
                    let dy = if *y_up { -c[1] } else { c[1] };
                    let to = [from_v[0] + c[0] * px_per_unit, from_v[1] + dy * px_per_unit];
                    let from_text = fmt_value(&Value::Point(prismal_kernel::Arr::from_slice(&base)), &Type::point(""));
                    (Shape::Arrow { from: from_v, to }, format!("arrow {label} = {} from {from_text}", fmt_value(&v, ty)))
                }
                Err(s) => status(s),
            }
        }
        CKind::Segment { a, b } => match (eval(a), eval(b)) {
            (Ok(x), Ok(y)) => (Shape::Segment { from: ctx.to_view(&coords(&x)), to: ctx.to_view(&coords(&y)) }, "segment".into()),
            (Err(s), _) | (_, Err(s)) => status(s),
        },
        CKind::Graph { f, lo, hi, label } => match eval(f) {
            Ok(Value::Func(body)) => {
                let n = 100;
                let mut pts = Vec::with_capacity(n + 1);
                for i in 0..=n {
                    let x = lo + (hi - lo) * i as f64 / n as f64;
                    let args = [Value::Num(x)];
                    let y = body.eval(&Ctx { vals, der: None, t, t0: run.config.t0, args: &args });
                    if let Ok(Value::Num(y)) = y {
                        pts.push([x, y]);
                    }
                }
                (Shape::Polyline { points: pts }, format!("graph of {label} for x from {} to {}", fmt_num(*lo), fmt_num(*hi)))
            }
            Ok(_) => (Shape::Status { status: "not a function".into() }, "graph: not available".into()),
            Err(s) => status(s),
        },
        CKind::Label { value, ty, label } => match eval(value) {
            Ok(v) => {
                let s = fmt_value(&v, ty);
                (Shape::Text { value: s.clone() }, format!("{label} = {s}"))
            }
            Err(s) => status(s),
        },
        CKind::Formula { lhs, rhs, params, refs, live } => {
            let mut symbols = vec![];
            for (id, i) in refs {
                let value = if *live { Some(fmt_binding(cm, *i, &vals[*i])) } else { None };
                symbols.push(Symbol { binding: id.clone(), symbol: symbol(cm, id), value });
            }
            let mut text = format!("{lhs} = {}", print(rhs, cm, params));
            let values: Vec<String> = symbols.iter().filter_map(|s| s.value.as_ref().map(|v| format!("{} = {v}", s.symbol))).collect();
            if !values.is_empty() {
                text = format!("{text}, where {}", values.join(", "));
            }
            (Shape::Formula { lhs: lhs.clone(), rhs: rhs.clone(), symbols }, text)
        }
        CKind::Control { control, binding, idx, min, max, step } => {
            let v = match &vals[*idx] {
                Value::Num(x) => *x,
                Value::Bool(b) => *b as u8 as f64,
                _ => f64::NAN,
            };
            let range = match (min, max) {
                (Some(a), Some(b)) => format!(", from {} to {}", fmt_binding(cm, *idx, &Value::Num(*a)), fmt_binding(cm, *idx, &Value::Num(*b))),
                _ => String::new(),
            };
            let text = format!("{control} for {} = {}{range}", symbol(cm, binding), fmt_binding(cm, *idx, &vals[*idx]));
            (Shape::Control { control: control.clone(), binding: binding.clone(), value: v, min: *min, max: *max, step: *step }, text)
        }
        CKind::Axes => (Shape::Axes, "axes".into()),
        CKind::Grid => (Shape::Grid, "grid".into()),
    };
    RepFrame { id: r.rep.id.clone(), kind: r.rep.kind.clone(), name: r.rep.name.clone(), shape, text, highlighted: false, valid: None }
}

/// The compiled views of a presentation.
#[derive(Clone, Debug)]
pub struct Projector {
    pub views: Vec<(Id, ViewCtx, Vec<CRep>)>,
}

impl Projector {
    pub fn new(cm: &CModel, p: &Presentation) -> Result<Projector, Vec<PDiag>> {
        let mut views = vec![];
        let mut diags = vec![];
        for v in &p.views {
            match ViewCtx::of(cm, v) {
                Ok(ctx) => {
                    let mut reps = vec![];
                    for r in &v.representations {
                        match compile_rep(cm, &ctx, r) {
                            Ok(c) => reps.push(c),
                            Err(d) => diags.extend(d),
                        }
                    }
                    views.push((v.id.clone(), ctx, reps));
                }
                Err(d) => diags.push(d),
            }
        }
        if diags.is_empty() {
            Ok(Projector { views })
        } else {
            Err(diags)
        }
    }

    pub fn ctx(&self, view: Option<&str>) -> ViewCtx {
        view.and_then(|v| self.views.iter().find(|x| x.0 == v)).map(|x| x.1.clone()).unwrap_or(ViewCtx::Panel)
    }

    /// Projects every view, plus extra representations shown by a timeline (by view, or
    /// over the presentation when the view is `None`), on one state: every view shows the
    /// same instant (PK-7.5).
    pub fn frame(&self, cm: &CModel, run: &Run, vals: &[Value], t: f64, extra: &[(Option<Id>, CRep)]) -> (Vec<ViewFrame>, Vec<RepFrame>) {
        let mut out = vec![];
        for (id, ctx, reps) in &self.views {
            let mut rs: Vec<RepFrame> = reps.iter().map(|r| project(cm, ctx, r, run, vals, t)).collect();
            for (_, r) in extra.iter().filter(|(v, _)| v.as_deref() == Some(id.as_str())) {
                rs.push(project(cm, ctx, r, run, vals, t));
            }
            out.push(ViewFrame { id: id.clone(), kind: ctx.kind(), reps: rs });
        }
        let overlay = extra.iter().filter(|(v, _)| v.is_none()).map(|(_, r)| project(cm, &ViewCtx::Panel, r, run, vals, t)).collect();
        (out, overlay)
    }
}

/// Evaluates a constant expression as a value (used for proposals shown in previews).
pub fn value_of(cm: &CModel, e: &Expr) -> Option<Value> {
    constant(cm, e).ok()
}
