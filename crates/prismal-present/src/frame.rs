//! Projection and frame descriptions (PK sections 5 to 7, 11 and 12).
//!
//! A frame description lists, for one presented instant, the visible representations with
//! their resolved properties in view coordinates, their text alternatives, and the
//! captions and announcements due (PK-12.1). It is medium-independent and serializable.
//!
//! View coordinates: in a spatial view, pixels with the model origin at `(0, 0)` and `y`
//! pointing down on screen (a view with `y: up` negates model `y`); in a plot view, the
//! plot's own coordinates; in a panel, none.

use crate::math::{self, MathLayout};
use crate::text::{fmt_binding, fmt_num, fmt_value, print, symbol, unit_text};
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
    /// Every representation in the frame, views first, a group before its members.
    pub fn reps(&self) -> impl Iterator<Item = &RepFrame> {
        fn walk<'a>(r: &'a RepFrame, out: &mut Vec<&'a RepFrame>) {
            out.push(r);
            if let Shape::Group { members } = &r.shape {
                members.iter().for_each(|m| walk(m, out));
            }
        }
        let mut out = vec![];
        self.views.iter().flat_map(|v| v.reps.iter()).chain(self.overlay.iter()).for_each(|r| walk(r, &mut out));
        out.into_iter()
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
    /// A camera set by the timeline (D-042).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<Camera>,
}

/// A view's camera (D-042): the centre in view coordinates, the zoom, and `blend`, how far
/// the renderer moves from its own framing to this centre (1 once a move has ended).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Camera {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 2]>,
    pub zoom: f64,
    pub blend: f64,
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
    /// During a reveal or a hide: opacity from 0 to 1 (PK-8.4, D-042).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// During a `reveal draw`: the fraction of a line or path drawn so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawn: Option<f64>,
    /// The label drawn with a marker, an arrow or a graph: its author name or its source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// For a representation with a declared inverse (PK-5.6): the part it is dragged by,
    /// `body` or the part's name (`head`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag: Option<String>,
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
    /// `layout` places it for any medium (D-046).
    Formula { lhs: String, rhs: Expr, symbols: Vec<Symbol>, layout: MathLayout },
    /// `value`, `min`, `max` and `step` are in coherent SI units; `symbol` is the binding's
    /// display symbol, `display_unit` its display unit (`value / scale` in `text`), else
    /// `unit` names the coherent SI unit.
    Control {
        control: String,
        binding: Id,
        value: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        symbol: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        display_unit: Option<DisplayUnit>,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    /// The source has no value at this instant (PK-3.3).
    Status { status: String },
    /// A closed path through points.
    Polygon { points: Vec<[f64; 2]> },
    /// A model equation typeset from the IR (PK-6.5); `name` is the equation's name.
    Equation { name: String, lhs: Expr, rhs: Expr, symbols: Vec<Symbol>, layout: MathLayout },
    /// A button that requests an event (D-027).
    Button { event: Id, label: String },
    /// Rows of values: the sample instant, then one column per component.
    Table { columns: Vec<String>, rows: Vec<Vec<String>> },
    /// A group's members, already placed by its transform (D-043).
    Group { members: Vec<RepFrame> },
}

/// Applies `f` to every representation in `reps`, a group before its members.
pub fn each_rep_mut(reps: &mut [RepFrame], f: &mut impl FnMut(&mut RepFrame)) {
    for r in reps {
        f(r);
        if let Shape::Group { members } = &mut r.shape {
            each_rep_mut(members, f);
        }
    }
}

/// Removes the representations for which `keep` is false, members of groups included.
pub fn retain_reps(reps: &mut Vec<RepFrame>, keep: &impl Fn(&RepFrame) -> bool) {
    reps.retain(|r| keep(r));
    for r in reps {
        if let Shape::Group { members } = &mut r.shape {
            retain_reps(members, keep);
        }
    }
}

/// A display unit: a value in coherent SI units shows as `value / scale` followed by `text`.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DisplayUnit {
    pub text: String,
    pub scale: f64,
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
    /// Ranges in coherent SI units, and the dimension of each axis (PK-7.3).
    Plot { x: (f64, f64), y: (f64, f64), dims: (Dim, Dim) },
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
                let dim = |e: &Expr| match compile_expr(cm, e, None) {
                    Ok((_, Type::Quantity { dim })) => Ok(dim),
                    _ => Err(err("PK-E04", "a plot axis range is a pair of numbers or quantities: `x: [0 s, 5 s]` (PK-7.3)".into())),
                };
                let (dx, dy) = (dim(&x[0])?, dim(&y[0])?);
                if dim(&x[1])? != dx || dim(&y[1])? != dy {
                    return Err(err("PK-E04", "both ends of a plot axis range have the same dimension (PK-7.3)".into()));
                }
                Ok(ViewCtx::Plot { x: (n(&x[0])?, n(&x[1])?), y: (n(&y[0])?, n(&y[1])?), dims: (dx, dy) })
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
            ViewCtx::Plot { dims, .. } => tuple(vec![si_num(p[0], &dims.0), si_num(p[1], &dims.1)]),
            ViewCtx::Panel => tuple(vec![prismal_ir::build::lit(p[0]), prismal_ir::build::lit(p[1])]),
        }
    }
}

/// A number in coherent SI units of a dimension, as a literal (bare when dimensionless).
fn si_num(x: f64, dim: &Dim) -> Expr {
    if dim.is_none() {
        prismal_ir::build::lit(x)
    } else {
        Expr::Num { num: x, unit: Some(prismal_ir::Unit { text: crate::text::unit_text(dim), scale: 1.0, offset: 0.0, dim: *dim }) }
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
    /// The path of a point, sampled every `every` seconds from the run's start (PK-6.3).
    Trace { pos: CExpr, every: f64, label: String },
    /// A scalar against elapsed time, sampled every `every` seconds (PK-6.3).
    Series { value: CExpr, every: f64, label: String },
    Label { value: CExpr, ty: Type, label: String },
    Formula { lhs: String, rhs: Expr, params: Vec<String>, refs: Vec<(Id, usize)>, live: bool },
    Control { control: String, binding: Id, idx: usize, min: Option<f64>, max: Option<f64>, step: Option<f64> },
    Poly { points: Vec<CExpr>, closed: bool },
    Equation { name: String, lhs: Expr, rhs: Expr, refs: Vec<(Id, usize)>, live: bool },
    Button { event: Id, label: String },
    Table { value: CExpr, tys: Vec<Type>, every: f64, columns: Vec<String> },
    Axes,
    Grid,
    /// Members drawn with a shared transform: placed at `at`, turned by `rotate`, scaled by
    /// `scale` (D-043).
    Group { at: Option<CExpr>, rotate: Option<CExpr>, scale: Option<CExpr>, members: Vec<CRep> },
}

/// A group's transform in model coordinates (D-043): `p -> at + k R(angle) p` for points,
/// `v -> k R(angle) v` for vectors.
#[derive(Clone, Copy, Debug)]
pub struct Tf {
    at: [f64; 2],
    angle: f64,
    k: f64,
}

impl Tf {
    pub const ID: Tf = Tf { at: [0.0, 0.0], angle: 0.0, k: 1.0 };

    fn turn(&self, a: &[f64]) -> [f64; 2] {
        let (x, y) = (a.first().copied().unwrap_or(0.0), a.get(1).copied().unwrap_or(0.0));
        let (s, c) = self.angle.sin_cos();
        [self.k * (c * x - s * y), self.k * (s * x + c * y)]
    }

    /// A value in the group's frame as a value in the view's space.
    fn apply(&self, v: Value) -> Value {
        if self.at == [0.0, 0.0] && self.angle == 0.0 && self.k == 1.0 {
            return v;
        }
        match v {
            Value::Point(a) => {
                let r = self.turn(a.as_slice());
                Value::Point(prismal_kernel::Arr::from_slice(&[self.at[0] + r[0], self.at[1] + r[1]]))
            }
            Value::Vec(a) => Value::Vec(prismal_kernel::Arr::from_slice(&self.turn(a.as_slice()))),
            other => other,
        }
    }
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
                // A pair of plot coordinates with the dimensions of the plot's axes (PK-7.3).
                (Type::Tuple { items }, ViewCtx::Plot { dims, .. }) => {
                    items.len() == 2 && items[0] == (Type::Quantity { dim: dims.0 }) && items[1] == (Type::Quantity { dim: dims.1 })
                }
                _ => false,
            };
            if !ok {
                return Err(needs("a point of the view's space, or a pair of plot coordinates in the units of the plot's axes"));
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
        "trace" | "series_plot" => {
            let Some(Arg::Sampled { expr: e, every }) = rep.sources.first() else {
                return Err(needs("a sampled source: `pos every 0.02 s`"));
            };
            let dt = match compile_expr(cm, every, None) {
                Ok((_, Type::Quantity { dim })) if dim == Dim::time() => number(cm, every).map_err(|m| d("PK-E02", m))?,
                _ => return Err(d("PK-E04", "a sampling interval is a duration: `every 0.02 s`".into())),
            };
            if !(dt > 0.0) {
                return Err(d("PK-E05", "a sampling interval is positive".into()));
            }
            if rep.kind == "trace" {
                let ViewCtx::Spatial { space, .. } = ctx else { return Err(d("PK-E05", "a trace belongs in a spatial view".into())) };
                let (c, _) = ce(e, Some(&Type::Point { space: space.clone() }))?;
                CKind::Trace { pos: c, every: dt, label: label(e) }
            } else {
                let ViewCtx::Plot { dims, .. } = ctx else { return Err(d("PK-E05", "a series plot belongs in a plot view".into())) };
                let (c, t) = ce(e, None)?;
                let Type::Quantity { dim } = t else { return Err(needs("a number or a quantity")) };
                if dims.0 != Dim::time() {
                    return Err(d("PK-E04", "a series plot draws against elapsed time: its plot's x axis is a time range, `x: [0 s, 5 s]` (PK-7.3)".into()));
                }
                if dim != dims.1 {
                    return Err(d("PK-E04", format!("the series has dimension {dim}, the plot's y axis {} (PK-7.3)", dims.1)));
                }
                CKind::Series { value: c, every: dt, label: label(e) }
            }
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
                // A declared function shows its definition: `kinetic(m, v) = 0.5 m v²` (D-048).
                (Some(Arg::Expr { expr: Expr::Fn { r#fn } }), None) => {
                    let f = cm.ir.function(r#fn).ok_or_else(|| d("PK-E01", format!("unknown function `{}`", r#fn)))?;
                    let names = f.param_names();
                    (format!("{}({})", f.name, names.join(", ")), f.body.clone(), names)
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
        "polyline" | "polygon" => {
            let ViewCtx::Spatial { space, .. } = ctx else { return Err(d("PK-E05", format!("a {} belongs in a spatial view", rep.kind))) };
            let p = Type::Point { space: space.clone() };
            let mut points = vec![];
            for a in &rep.sources {
                let Arg::Expr { expr } = a else { return Err(needs("points")) };
                points.push(ce(expr, Some(&p))?.0);
            }
            if points.len() < 2 {
                return Err(needs("at least two points"));
            }
            CKind::Poly { points, closed: rep.kind == "polygon" }
        }
        "equation" => {
            let Some(Arg::Element { element }) = rep.sources.first() else { return Err(needs("the name of a model equation")) };
            let q = cm.ir.equations.iter().find(|q| &q.id == element).ok_or_else(|| needs("the name of a model equation"))?;
            let live = matches!(prop_expr(rep, "live"), Some(Expr::Bool { bool: true }));
            let mut refs: Vec<(Id, usize)> = vec![];
            for id in q.lhs.refs().into_iter().chain(q.rhs.refs()) {
                if let Some(&i) = cm.index.get(&id) {
                    if !refs.iter().any(|r| r.0 == id) && !matches!(cm.bindings[i].ty, Type::Function { .. }) {
                        refs.push((id, i));
                    }
                }
            }
            CKind::Equation { name: q.name.clone(), lhs: q.lhs.clone(), rhs: q.rhs.clone(), refs, live }
        }
        "button" => {
            let msg = "the name of an event declared `on request` (D-027)";
            let Some(Arg::Element { element }) = rep.sources.first() else { return Err(needs(msg)) };
            let Some(ev) = cm.ir.events.iter().find(|e| &e.id == element) else { return Err(needs(msg)) };
            if !matches!(ev.trigger, prismal_ir::Trigger::Request) {
                return Err(d("PK-E03", format!("`{}` is not declared `on request`, so no button can request it (D-027)", ev.name)));
            }
            let label = match rep.prop("label") {
                Some(Arg::Text { text }) => text.clone(),
                _ => ev.name.clone(),
            };
            CKind::Button { event: element.clone(), label }
        }
        "table" => {
            let Some(Arg::Sampled { expr: e, every }) = rep.sources.first() else {
                return Err(needs("a sampled source: `table((pos.x, pos.y) every 0.5 s)`"));
            };
            let dt = match compile_expr(cm, every, None) {
                Ok((_, Type::Quantity { dim })) if dim == Dim::time() => number(cm, every).map_err(|m| d("PK-E02", m))?,
                _ => return Err(d("PK-E04", "a sampling interval is a duration: `every 0.5 s`".into())),
            };
            if !(dt > 0.0) {
                return Err(d("PK-E05", "a sampling interval is positive".into()));
            }
            let (c, t) = ce(e, None)?;
            let (tys, columns) = match (&t, e) {
                (Type::Tuple { items }, Expr::Tuple { tuple }) => (items.clone(), tuple.iter().map(|x| print(x, cm, &[])).collect()),
                _ => (vec![t.clone()], vec![print(e, cm, &[])]),
            };
            CKind::Table { value: c, tys, every: dt, columns }
        }
        "group" => {
            let ViewCtx::Spatial { space, .. } = ctx else { return Err(d("PK-E05", "a group belongs in a spatial view (PK-6.3b)".into())) };
            if !rep.sources.is_empty() || rep.props.iter().any(|p| !["at", "rotate", "scale"].contains(&p.name.as_str())) {
                return Err(d("PK-E05", "a group takes `at`, `rotate` and `scale`; its members are written in its block (PK-6.3b)".into()));
            }
            let at = match prop_expr(rep, "at") {
                Some(e) => Some(ce(e, Some(&Type::Point { space: space.clone() }))?.0),
                None => None,
            };
            let real = |name: &str| -> Result<Option<CExpr>, Vec<PDiag>> {
                let Some(e) = prop_expr(rep, name) else { return Ok(None) };
                match ce(e, None)? {
                    (c, Type::Quantity { dim }) if dim.is_none() => Ok(Some(c)),
                    _ => Err(d("PK-E04", format!("a group's `{name}` is a number{} (PK-6.3b)", if name == "rotate" { " or an angle" } else { "" }))),
                }
            };
            let (rotate, scale) = (real("rotate")?, real("scale")?);
            if prop_expr(rep, "scale").is_some_and(|e| number(cm, e).is_ok_and(|k| k <= 0.0)) {
                return Err(d("PK-E02", "a group's scale is positive (PK-6.3b)".into()));
            }
            if rep.members.is_empty() {
                return Err(needs("members"));
            }
            let mut members = vec![];
            let mut diags = vec![];
            for m in &rep.members {
                if !["marker", "arrow", "segment", "polyline", "polygon", "group"].contains(&m.kind.as_str()) {
                    diags.push(PDiag {
                        code: "PK-E05",
                        message: format!("a group holds markers, arrows, segments, polylines, polygons and groups, not `{}` (PK-6.3b)", m.kind),
                        element: m.id.clone(),
                    });
                    continue;
                }
                if m.inverse.is_some() {
                    diags.push(PDiag { code: "PK-E06", message: "a drag on a member of a group is not implemented by the prototype".into(), element: m.id.clone() });
                    continue;
                }
                match compile_rep(cm, ctx, m) {
                    Ok(c) => members.push(c),
                    Err(e) => diags.extend(e),
                }
            }
            if !diags.is_empty() {
                return Err(diags);
            }
            CKind::Group { at, rotate, scale, members }
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
    project_in(cm, ctx, r, run, vals, t, Tf::ID)
}

/// Projects a representation whose values are in the frame of a group (D-043).
fn project_in(cm: &CModel, ctx: &ViewCtx, r: &CRep, run: &Run, vals: &[Value], t: f64, tf: Tf) -> RepFrame {
    let eval = |c: &CExpr| run.eval_state(c, vals, t).map(|v| tf.apply(v));
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
                    return RepFrame { id: r.rep.id.clone(), kind: r.rep.kind.clone(), name: r.rep.name.clone(), shape: sh, text: tx, highlighted: false, valid: None, opacity: None, drawn: None, label: None, drag: None };
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
                    let y = body.eval(&Ctx { vals, der: None, t, t0: run.config.t0, args: &args, payloads: &[] });
                    if let Ok(Value::Num(y)) = y {
                        pts.push([x, y]);
                    }
                }
                (Shape::Polyline { points: pts }, format!("graph of {label} for x from {} to {}", fmt_num(*lo), fmt_num(*hi)))
            }
            Ok(_) => (Shape::Status { status: "not a function".into() }, "graph: not available".into()),
            Err(s) => status(s),
        },
        CKind::Trace { pos, every, label } => {
            let pts: Vec<[f64; 2]> = samples(run, t, *every).filter_map(|s| run.eval_state(pos, &run.state_at(s), s).ok()).map(|v| ctx.to_view(&coords(&v))).collect();
            let text = match (pts.first(), pts.last()) {
                (Some(_), Some(_)) => format!("trace of {label}: {} samples every {} s up to t = {} s", pts.len(), fmt_num(*every), fmt_num(t - run.config.t0)),
                _ => format!("trace of {label}: no samples"),
            };
            (Shape::Polyline { points: pts }, text)
        }
        CKind::Series { value, every, label } => {
            let t0 = run.config.t0;
            let pts: Vec<[f64; 2]> = samples(run, t, *every)
                .filter_map(|s| match run.eval_state(value, &run.state_at(s), s) {
                    Ok(Value::Num(y)) => Some([s - t0, y]),
                    _ => None,
                })
                .collect();
            let text = match pts.last() {
                Some(p) => format!("series of {label} against elapsed time: {} samples, last {} at {} s", pts.len(), fmt_num(p[1]), fmt_num(p[0])),
                None => format!("series of {label}: no samples"),
            };
            (Shape::Polyline { points: pts }, text)
        }
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
            let layout = math::layout(&math::formula(lhs, rhs, cm, params));
            (Shape::Formula { lhs: lhs.clone(), rhs: rhs.clone(), symbols, layout }, text)
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
            let display_unit = cm.ir.binding(binding).and_then(|b| b.display.unit.as_ref()).and_then(|u| prismal_ir::Unit::parse(u).ok()).map(|u| DisplayUnit { text: u.text, scale: u.scale });
            let unit = match (&display_unit, &cm.bindings[*idx].ty) {
                (None, Type::Quantity { dim }) if !dim.is_none() => Some(unit_text(dim)),
                _ => None,
            };
            let shape = Shape::Control { control: control.clone(), binding: binding.clone(), value: v, min: *min, max: *max, step: *step, symbol: symbol(cm, binding), display_unit, unit };
            (shape, text)
        }
        CKind::Poly { points, closed } => {
            let mut pts = vec![];
            for p in points {
                match eval(p) {
                    Ok(v) => pts.push(ctx.to_view(&coords(&v))),
                    Err(s) => {
                        let (sh, tx) = status(s);
                        return RepFrame { id: r.rep.id.clone(), kind: r.rep.kind.clone(), name: r.rep.name.clone(), shape: sh, text: tx, highlighted: false, valid: None, opacity: None, drawn: None, label: None, drag: None };
                    }
                }
            }
            let text = format!("{} through {} points", r.rep.kind, pts.len());
            (if *closed { Shape::Polygon { points: pts } } else { Shape::Polyline { points: pts } }, text)
        }
        CKind::Equation { name, lhs, rhs, refs, live } => {
            let symbols: Vec<Symbol> = refs
                .iter()
                .map(|(id, i)| Symbol { binding: id.clone(), symbol: symbol(cm, id), value: if *live { Some(crate::text::fmt_binding(cm, *i, &vals[*i])) } else { None } })
                .collect();
            let mut text = format!("equation {name}: {} = {}", print(lhs, cm, &[]), print(rhs, cm, &[]));
            let values: Vec<String> = symbols.iter().filter_map(|s| s.value.as_ref().map(|v| format!("{} = {v}", s.symbol))).collect();
            if !values.is_empty() {
                text = format!("{text}, where {}", values.join(", "));
            }
            let layout = math::layout(&math::equation(lhs, rhs, cm));
            (Shape::Equation { name: name.clone(), lhs: lhs.clone(), rhs: rhs.clone(), symbols, layout }, text)
        }
        CKind::Button { event, label } => {
            let name = cm.ir.events.iter().find(|e| &e.id == event).map(|e| e.name.clone()).unwrap_or_default();
            (Shape::Button { event: event.clone(), label: label.clone() }, format!("button {label}: requests {name}"))
        }
        CKind::Table { value, tys, every, columns } => {
            let t0 = run.config.t0;
            let mut rows = vec![];
            for s in samples(run, t, *every) {
                let Ok(v) = run.eval_state(value, &run.state_at(s), s) else { continue };
                let mut row = vec![fmt_num(s - t0)];
                match (&v, tys.len()) {
                    (Value::Tuple(items), n) if n > 1 => row.extend(items.iter().zip(tys).map(|(x, ty)| fmt_value(x, ty))),
                    _ => row.push(fmt_value(&v, &tys[0])),
                }
                rows.push(row);
            }
            let mut cols = vec!["t (s)".to_string()];
            cols.extend(columns.iter().cloned());
            let text = format!("table of {} with {} rows, every {} s", columns.join(", "), rows.len(), fmt_num(*every));
            (Shape::Table { columns: cols, rows }, text)
        }
        CKind::Group { at, rotate, scale, members } => {
            let num = |c: &Option<CExpr>, default: f64| match c.as_ref().map(eval) {
                Some(Ok(Value::Num(x))) => Ok(x),
                Some(Ok(_)) | None => Ok(default),
                Some(Err(s)) => Err(s),
            };
            // `at` is evaluated in the enclosing frame, so nested groups compose.
            let at = match at.as_ref().map(eval) {
                Some(Ok(v)) => Ok(coords(&v)),
                Some(Err(s)) => Err(s),
                None => Ok(tf.at.to_vec()),
            };
            match (at, num(rotate, 0.0), num(scale, 1.0)) {
                (Ok(a), Ok(angle), Ok(k)) => {
                    let inner = Tf { at: [a[0], a[1]], angle: tf.angle + angle, k: tf.k * k };
                    let ms: Vec<RepFrame> = members.iter().map(|m| project_in(cm, ctx, m, run, vals, t, inner)).collect();
                    let name = r.rep.name.clone().unwrap_or("group".into());
                    let text = format!("{name}: {}", ms.iter().map(|m| m.text.as_str()).collect::<Vec<_>>().join("; "));
                    (Shape::Group { members: ms }, text)
                }
                (Err(s), _, _) | (_, Err(s), _) | (_, _, Err(s)) => status(s),
            }
        }
        CKind::Axes => (Shape::Axes, "axes".into()),
        CKind::Grid => (Shape::Grid, "grid".into()),
    };
    let label = match &r.kind {
        CKind::Marker { label, .. } | CKind::Arrow { label, .. } | CKind::Graph { label, .. } => Some(label.clone()),
        _ => None,
    };
    let drag = r.rep.inverse.as_ref().map(|inv| inv.part.clone().unwrap_or_else(|| "body".into()));
    RepFrame { id: r.rep.id.clone(), kind: r.rep.kind.clone(), name: r.rep.name.clone(), shape, text, highlighted: false, valid: None, opacity: None, drawn: None, label, drag }
}

/// Sample instants `t0, t0 + dt, ...` up to `t`, and `t` itself (PK-6.3, PK-7.5: a trace
/// ends at the instant shown).
fn samples(run: &Run, t: f64, dt: f64) -> impl Iterator<Item = f64> {
    let t0 = run.config.t0;
    let end = t.min(run.end_time());
    let n = ((end - t0) / dt).floor().max(0.0) as usize;
    (0..=n).map(move |k| t0 + k as f64 * dt).filter(move |s| *s < end).chain(std::iter::once(end))
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
            out.push(ViewFrame { id: id.clone(), kind: ctx.kind(), reps: rs, camera: None });
        }
        let overlay = extra.iter().filter(|(v, _)| v.is_none()).map(|(_, r)| project(cm, &ViewCtx::Panel, r, run, vals, t)).collect();
        (out, overlay)
    }
}

/// Evaluates a constant expression as a value (used for proposals shown in previews).
pub fn value_of(cm: &CModel, e: &Expr) -> Option<Value> {
    constant(cm, e).ok()
}
