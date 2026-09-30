//! Lowering of spaces and models to the IR (docs/spec/04-ir.md).
//!
//! Identities follow declaration paths (D-036), as the builder API does: `Model.name` for
//! bindings, `Model.event.name`, `Model.process.name`, `Model.equation.name`,
//! `Model.constraint.name`, and `Model.flow.n` for the n-th flow in source order.
//! Presentations and runs are not lowered: their IR is specified with the presentation
//! prototype (04-ir section 7).

use crate::ast::{self, Bound, ExprKind, Member, Modifier, RoleWord, TypeExpr};
use crate::{Diag, Span};
use prismal_ir::build;
use prismal_ir::*;
use std::collections::{BTreeMap, HashMap};

/// Where each IR element was written.
pub type SourceMap = BTreeMap<Id, Span>;

/// Named dimensions (MK-3.3, R-48) and base dimension symbols (MK-3.1).
pub fn named_dim(name: &str) -> Option<Dim> {
    let d = |s: &str| Dim::parse(s).expect("valid dimension");
    Some(match name {
        "1" => Dim::NONE,
        "L" => d("L"),
        "M" => d("M"),
        "T" => d("T"),
        "I" => d("I"),
        "Θ" | "Th" => d("Th"),
        "N" => d("N"),
        "J" => d("J"),
        "Length" => d("L"),
        "Mass" => d("M"),
        "Time" => d("T"),
        "Current" => d("I"),
        "Amount" => d("N"),
        "Area" => d("L^2"),
        "Volume" => d("L^3"),
        "Velocity" => d("L/T"),
        "Acceleration" => d("L/T^2"),
        "Frequency" => d("1/T"),
        "Momentum" => d("M L/T"),
        "Force" => d("M L/T^2"),
        "Energy" => d("M L^2/T^2"),
        "Power" => d("M L^2/T^3"),
        "Pressure" => d("M/L/T^2"),
        _ => return None,
    })
}

const FUNCS: [(&str, Func, usize); 10] = [
    ("sin", Func::Sin, 1),
    ("cos", Func::Cos, 1),
    ("tan", Func::Tan, 1),
    ("sqrt", Func::Sqrt, 1),
    ("exp", Func::Exp, 1),
    ("log", Func::Log, 1),
    ("abs", Func::Abs, 1),
    ("min", Func::Min, 2),
    ("max", Func::Max, 2),
    ("atan2", Func::Atan2, 2),
];

pub fn lower(file: &ast::File) -> (Document, SourceMap, Vec<Diag>) {
    let mut diags = vec![];
    let mut map = SourceMap::new();
    let mut spaces: Vec<Space> = vec![];
    for it in &file.items {
        if let ast::Item::Space(s) = it {
            if spaces.iter().any(|x| x.name == s.name.text) {
                diags.push(Diag::new("SX-E09", format!("space `{}` declared twice", s.name.text), s.name.span));
                continue;
            }
            if let Some(sp) = lower_space(s, &mut diags) {
                map.insert(sp.id.clone(), s.span);
                spaces.push(sp);
            }
        }
    }
    let mut models: Vec<Model> = vec![];
    for it in &file.items {
        if let ast::Item::Model(m) = it {
            if models.iter().any(|x| x.name == m.name.text) {
                diags.push(Diag::new("SX-E09", format!("model `{}` declared twice", m.name.text), m.name.span));
                continue;
            }
            let mut cx = ModelCx::new(m, &spaces, &mut diags, &mut map);
            let model = cx.lower(m);
            models.push(model);
        }
    }
    let mut presentations = vec![];
    for it in &file.items {
        if let ast::Item::Presentation(p) = it {
            if presentations.iter().any(|x: &prismal_ir::present::Presentation| x.name == p.name.text) {
                diags.push(Diag::new("SX-E09", format!("presentation `{}` declared twice", p.name.text), p.name.span));
                continue;
            }
            if let Some(pr) = crate::lower_present::presentation(p, &models, &spaces, &mut diags, &mut map) {
                presentations.push(pr);
            }
        }
    }
    let mut runs = vec![];
    for it in &file.items {
        if let ast::Item::Run(r) = it {
            if runs.iter().any(|x: &prismal_ir::present::RunCase| x.name == r.name.text) {
                diags.push(Diag::new("SX-E09", format!("run `{}` declared twice", r.name.text), r.name.span));
                continue;
            }
            if let Some(rc) = crate::lower_present::run(r, &models, &spaces, &presentations, &mut diags, &mut map) {
                runs.push(rc);
            }
        }
    }
    let mut doc = Document::new(spaces, models);
    doc.presentations = presentations;
    doc.runs = runs;
    (doc, map, diags)
}

/// Lowers an expression in the scope of a model already in `doc`: names resolve to its
/// bindings and events, components to its default space. Used for run overrides and
/// observations until presentations and runs have their IR (04-ir section 7).
pub fn lower_expr(doc: &Document, model: &str, e: &ast::Expr) -> Result<Expr, Vec<Diag>> {
    let m = doc.models.iter().find(|m| m.name == model).ok_or_else(|| vec![Diag::new("SX-E03", format!("unknown model `{model}`"), e.span)])?;
    let mut diags = vec![];
    let mut map = SourceMap::new();
    let mut cx = ModelCx::scope(m, &doc.spaces, &mut diags, &mut map);
    let out = cx.expr(e, &[]);
    if diags.is_empty() {
        Ok(out)
    } else {
        Err(diags)
    }
}

fn lower_space(s: &ast::SpaceDecl, diags: &mut Vec<Diag>) -> Option<Space> {
    if s.kind.text != "euclidean" {
        diags.push(Diag::new("SX-E06", format!("space kind `{}`: v0 has only `euclidean(n)`", s.kind.text), s.kind.span));
        return None;
    }
    let n = match s.args.as_slice() {
        [ast::Arg { name: None, value: ast::Expr { kind: ExprKind::Num(n, None), .. }, every: None }] if (1.0..=3.0).contains(n) && n.fract() == 0.0 => *n as u32,
        _ => {
            diags.push(Diag::new("SX-E02", "a space is declared `euclidean(n)` with n = 1, 2 or 3", s.span));
            return None;
        }
    };
    let mut sp = build::space(&s.name.text, n);
    sp.notes = s.notes.clone();
    Some(sp)
}

pub(crate) struct ModelCx<'a> {
    pub(crate) name: String,
    space: Option<&'a Space>,
    spaces: &'a [Space],
    pub(crate) bindings: HashMap<String, Id>,
    pub(crate) events: HashMap<String, Id>,
    /// Declared enumerations by name: identity and cases (D-049).
    pub(crate) enums: HashMap<String, (Id, Vec<String>)>,
    /// Declared functions by name (D-048).
    pub(crate) functions: HashMap<String, Id>,
    pub(crate) diags: &'a mut Vec<Diag>,
    pub(crate) map: &'a mut SourceMap,
    flows: usize,
    model: Model,
}

impl<'a> ModelCx<'a> {
    /// The scope of a lowered model, for expressions written outside it (presentations, runs).
    pub(crate) fn scope(m: &Model, spaces: &'a [Space], diags: &'a mut Vec<Diag>, map: &'a mut SourceMap) -> ModelCx<'a> {
        ModelCx {
            name: m.name.clone(),
            space: m.default_space.as_ref().and_then(|s| spaces.iter().find(|x| &x.id == s)),
            spaces,
            bindings: m.bindings.iter().map(|b| (b.name.clone(), b.id.clone())).collect(),
            events: m.events.iter().map(|ev| (ev.name.clone(), ev.id.clone())).collect(),
            enums: m.enums.iter().map(|e| (e.name.clone(), (e.id.clone(), e.cases.clone()))).collect(),
            functions: m.functions.iter().map(|f| (f.name.clone(), f.id.clone())).collect(),
            diags,
            map,
            flows: 0,
            model: build::ModelBuilder::new(&m.name).finish(),
        }
    }

    pub(crate) fn space_by_name(&self, name: &str) -> Option<Id> {
        self.spaces.iter().find(|s| s.name == name).map(|s| s.id.clone())
    }

    fn new(m: &ast::ModelDecl, spaces: &'a [Space], diags: &'a mut Vec<Diag>, map: &'a mut SourceMap) -> ModelCx<'a> {
        let name = m.name.text.clone();
        let space = match &m.space {
            Some(s) => match spaces.iter().find(|x| x.name == s.text) {
                Some(sp) => Some(sp),
                None => {
                    diags.push(Diag::new("SX-E03", format!("unknown space `{}`", s.text), s.span));
                    None
                }
            },
            None => None,
        };
        let model = build::ModelBuilder::new(&name).finish();
        ModelCx { name, space, spaces, bindings: HashMap::new(), events: HashMap::new(), enums: HashMap::new(), functions: HashMap::new(), diags, map, flows: 0, model }
    }

    pub(crate) fn err(&mut self, code: &'static str, msg: impl Into<String>, span: Span) {
        self.diags.push(Diag::new(code, msg, span));
    }

    fn lower(&mut self, m: &ast::ModelDecl) -> Model {
        self.model.default_space = self.space.map(|s| s.id.clone());
        self.model.notes = m.notes.clone();
        self.map.insert(self.model.id.clone(), m.span);
        // Declared names first, so that declarations may refer to later ones.
        let mut event_names: HashMap<String, Span> = HashMap::new();
        for mem in &m.members {
            match mem {
                Member::Decl(d) => {
                    self.bindings.insert(d.name.text.clone(), format!("{}.{}", self.name, d.name.text));
                }
                Member::Event(e) => self.declare_event(e, &mut event_names),
                Member::Process(p) => {
                    for e in &p.events {
                        self.declare_event(e, &mut event_names);
                    }
                }
                Member::Enum(e) => {
                    let id = format!("{}.enum.{}", self.name, e.name.text);
                    let cases = e.cases.iter().map(|c| c.text.clone()).collect();
                    if self.enums.insert(e.name.text.clone(), (id, cases)).is_some() {
                        self.err("SX-E09", format!("enumeration `{}` declared twice", e.name.text), e.name.span);
                    }
                }
                Member::Fn(f) => {
                    let id = format!("{}.fn.{}", self.name, f.name.text);
                    if self.functions.insert(f.name.text.clone(), id).is_some() {
                        self.err("SX-E09", format!("function `{}` declared twice", f.name.text), f.name.span);
                    }
                }
                _ => {}
            }
        }
        // One namespace for bindings, functions and cases: a name means one thing.
        for mem in &m.members {
            match mem {
                Member::Fn(f) if self.bindings.contains_key(&f.name.text) => {
                    self.err("SX-E09", format!("`{}` is both a binding and a function", f.name.text), f.name.span)
                }
                Member::Enum(e) => {
                    for c in &e.cases {
                        if self.bindings.contains_key(&c.text) || self.functions.contains_key(&c.text) {
                            self.err("SX-E09", format!("case `{}` has the name of a binding or function", c.text), c.span);
                        }
                    }
                }
                _ => {}
            }
        }
        let mut processes: HashMap<String, Span> = HashMap::new();
        let mut names: HashMap<String, Span> = HashMap::new();
        for mem in &m.members {
            match mem {
                Member::Decl(d) => self.decl(d),
                Member::Fn(f) => self.function(f),
                Member::Enum(e) => {
                    let (id, cases) = self.enums[&e.name.text].clone();
                    self.map.insert(id.clone(), e.span);
                    if !self.model.enums.iter().any(|x| x.id == id) {
                        self.model.enums.push(EnumDecl { id, name: e.name.text.clone(), cases, notes: e.notes.clone() });
                    }
                }
                Member::Flow(f) => self.flow(f, None),
                Member::Process(p) => {
                    if processes.insert(p.name.text.clone(), p.span).is_some() {
                        self.err("SX-E09", format!("process `{}` declared twice", p.name.text), p.name.span);
                    }
                    // MK-14.1: the IR carries the process kind; the working syntax has no
                    // spelling for it, so a process with flows is continuous, otherwise discrete.
                    let kind = if p.flows.is_empty() { ProcessKind::Discrete } else { ProcessKind::Continuous };
                    let id = format!("{}.process.{}", self.name, p.name.text);
                    self.map.insert(id.clone(), p.span);
                    self.model.processes.push(Process { id: id.clone(), name: p.name.text.clone(), kind, notes: p.notes.clone() });
                    for f in &p.flows {
                        self.flow(f, Some(&id));
                    }
                    for e in &p.events {
                        self.event(e, Some(&id));
                    }
                }
                Member::Event(e) => self.event(e, None),
                Member::Equation(eq) => {
                    if names.insert(format!("equation {}", eq.name.text), eq.span).is_some() {
                        self.err("SX-E09", format!("equation `{}` declared twice", eq.name.text), eq.name.span);
                    }
                    let lhs = self.expr(&eq.lhs, &[]);
                    let rhs = self.expr(&eq.rhs, &[]);
                    let tol = eq.checked.as_ref().map(|t| self.expr(t, &[]));
                    let role = if tol.is_some() { EquationRole::Check } else { EquationRole::Display };
                    let id = format!("{}.equation.{}", self.name, eq.name.text);
                    self.map.insert(id.clone(), eq.span);
                    self.model.equations.push(Equation { id, name: eq.name.text.clone(), lhs, rhs, role, tol });
                }
                Member::Constraint(c) => {
                    let name = match &c.name {
                        Some(n) => n.text.clone(),
                        None => format!("c{}", self.model.constraints.len() + 1),
                    };
                    let cond = self.expr(&c.cond, &[]);
                    let tol = c.within.as_ref().map(|t| self.expr(t, &[]));
                    // MK-12.4: constraints other than parameter ranges default to `report`.
                    let policy = match c.policy.as_ref().map(|p| p.text.as_str()) {
                        None | Some("report") => Policy::Report,
                        Some("reject") => Policy::Reject,
                        Some("stop") => Policy::Stop,
                        Some(other) => {
                            let span = c.policy.as_ref().unwrap().span;
                            self.err("SX-E02", format!("unknown policy `{other}`: expected `reject`, `report` or `stop`"), span);
                            Policy::Report
                        }
                    };
                    self.push_constraint(&name, cond, tol, policy, None, c.span);
                }
                Member::Object(n) => self.err("SX-E06", "contained object types are not in the v0 IR", n.span),
            }
        }
        std::mem::replace(&mut self.model, build::ModelBuilder::new("").finish())
    }

    /// A declared function (MK-10.3, D-048): its parameters are lambda parameters of its body.
    fn function(&mut self, f: &ast::FnDecl) {
        let id = format!("{}.fn.{}", self.name, f.name.text);
        let locals: Vec<String> = f.params.iter().map(|(n, _)| n.text.clone()).collect();
        let params = f.params.iter().map(|(n, t)| FnParam { name: n.text.clone(), ty: self.ty(t) }).collect();
        let result = self.ty(&f.result);
        let body = self.expr(&f.body, &locals);
        self.map.insert(id.clone(), f.span);
        if !self.model.functions.iter().any(|x| x.id == id) {
            self.model.functions.push(FunctionDecl { id, name: f.name.text.clone(), params, result, body, notes: f.notes.clone() });
        }
    }

    fn declare_event(&mut self, e: &ast::EventDecl, seen: &mut HashMap<String, Span>) {
        if seen.insert(e.name.text.clone(), e.span).is_some() {
            self.err("SX-E09", format!("event `{}` declared twice", e.name.text), e.name.span);
        }
        self.events.insert(e.name.text.clone(), format!("{}.event.{}", self.name, e.name.text));
    }

    fn push_constraint(&mut self, name: &str, cond: Expr, tol: Option<Expr>, policy: Policy, attached_to: Option<Id>, span: Span) {
        let id = format!("{}.constraint.{}", self.name, name);
        if self.model.constraints.iter().any(|c| c.id == id) {
            self.err("SX-E09", format!("constraint `{name}` declared twice"), span);
        }
        self.map.insert(id.clone(), span);
        self.model.constraints.push(Constraint { id, name: name.into(), cond, tol, policy, attached_to });
    }

    // ------------------------------------------------------------ declarations

    fn decl(&mut self, d: &ast::Decl) {
        let id = format!("{}.{}", self.name, d.name.text);
        let role = match d.role {
            RoleWord::Const => Role::Constant,
            RoleWord::Param => Role::Parameter,
            RoleWord::Input => Role::Input,
            RoleWord::State => Role::Continuous,
            RoleWord::Discrete => Role::Discrete,
            RoleWord::Derived => Role::Derived,
        };
        let mut ty = self.ty(&d.ty);
        let (mut init, mut def) = (None, None);
        let locals: Vec<String> = d.params.iter().flatten().map(|(n, _)| n.text.clone()).collect();
        let value = d.value.as_ref().map(|v| self.expr(v, &locals));
        if let Some(ps) = &d.params {
            if role != Role::Derived {
                self.err("SX-E08", "only derived bindings may have parameters (MK-10.7); use `derived`", d.name.span);
            }
            let params: Vec<Type> = ps.iter().map(|(_, t)| self.ty(t)).collect();
            ty = Type::func(params.clone(), ty);
            let names: Vec<&str> = locals.iter().map(|s| s.as_str()).collect();
            def = value.map(|body| build::lambda_named(&names, params, body));
        } else if role == Role::Derived {
            def = value;
        } else if role == Role::Input {
            if value.is_some() {
                self.err("SX-E08", "an input has no initial definition; its value comes from the environment", d.span);
            }
        } else {
            init = value;
        }
        let mut b = Binding {
            id: id.clone(),
            name: d.name.text.clone(),
            role,
            ty,
            init,
            def,
            intervenable: None,
            private: false,
            display: Display::default(),
            notes: d.notes.clone(),
        };
        for m in &d.modifiers {
            match m {
                Modifier::Intervenable => b.intervenable = Some(true),
                Modifier::Private => b.private = true,
                Modifier::Symbol(s) => b.display.symbol = Some(s.clone()),
                Modifier::Unit(u) => match Unit::parse(&u.text) {
                    Ok(_) => b.display.unit = Some(u.text.clone()),
                    Err(e) => self.err("SX-E05", e, u.span),
                },
            }
        }
        self.map.insert(id.clone(), d.span);
        self.model.bindings.push(b);
        // `where cond` and `in I`: a `reject` constraint attached to the binding (MK-12.4).
        if let Some(range) = &d.range {
            let (cond, span) = match range {
                ast::Range::Where(e) => (self.expr(e, &[]), e.span),
                ast::Range::In(i) => (self.interval(build::r(&id), i, &[]), i.span),
            };
            if role != Role::Parameter {
                self.err("SX-E08", "a range (`in` or `where`) is written on a parameter (MK-12.4); use `constraint`", span);
            }
            self.push_constraint(&format!("{}_range", d.name.text), cond, None, Policy::Reject, Some(id), span);
        }
    }

    fn flow(&mut self, f: &ast::FlowStmt, process: Option<&Id>) {
        let target = self.binding(&f.target);
        let expr = self.expr(&f.expr, &[]);
        self.flows += 1;
        let id = format!("{}.flow.{}", self.name, self.flows);
        self.map.insert(id.clone(), f.span);
        let kind = if f.contribute { FlowKind::Contribute } else { FlowKind::Define };
        self.model.flows.push(Flow { id, target, kind, expr, process: process.cloned() });
    }

    fn event(&mut self, e: &ast::EventDecl, process: Option<&Id>) {
        let trigger = match &e.trigger {
            ast::TriggerExpr::Rising(g) => Trigger::Rising { guard: self.expr(g, &[]) },
            ast::TriggerExpr::Falling(g) => Trigger::Falling { guard: self.expr(g, &[]) },
            ast::TriggerExpr::Crossing(g) => Trigger::Crossing { guard: self.expr(g, &[]) },
            ast::TriggerExpr::At(t) => Trigger::At { time: self.expr(t, &[]) },
            ast::TriggerExpr::Every(p, from) => Trigger::Every {
                period: self.expr(p, &[]),
                from: match from {
                    Some(f) => self.expr(f, &[]),
                    None => build::t0(),
                },
            },
            ast::TriggerExpr::Start => Trigger::Start,
            ast::TriggerExpr::Input(i) => Trigger::Input { binding: self.binding(i) },
            ast::TriggerExpr::Request(payload) => {
                if let Some((n, _)) = payload {
                    self.err("SX-E06", "event payloads are not yet implemented (MK-15.1)", n.span);
                }
                Trigger::Request
            }
            ast::TriggerExpr::On(n) => Trigger::On { event: self.event_id(n) },
        };
        let enable = e.enable.as_ref().map(|c| self.expr(c, &[]));
        let handler = e.handler.iter().map(|o| self.op(o)).collect();
        let zeno = e.zeno.as_ref().map(|z| Zeno {
            policy: match z {
                ast::ZenoClause::Stop => ZenoPolicy::Stop,
                ast::ZenoClause::Settle(ops) => ZenoPolicy::Settle { ops: ops.iter().map(|o| self.op(o)).collect() },
            },
            eps: None,
            n: None,
            window: None,
        });
        let id = format!("{}.event.{}", self.name, e.name.text);
        self.map.insert(id.clone(), e.span);
        self.model.events.push(Event {
            id,
            name: e.name.text.clone(),
            trigger,
            enable,
            handler,
            zeno,
            process: process.cloned(),
            notes: e.notes.clone(),
        });
    }

    pub(crate) fn op(&mut self, o: &ast::OpStmt) -> Op {
        match o {
            ast::OpStmt::Set { target, value, .. } => Op::Set { target: self.target(target), value: self.expr(value, &[]) },
            ast::OpStmt::Contribute { target, value, .. } => Op::Contribute { target: self.target(target), value: self.expr(value, &[]) },
            ast::OpStmt::Emit { event, payload, .. } => {
                let payload = payload.as_ref().map(|p| self.expr(p, &[]));
                Op::Emit { event: self.event_id(event), payload }
            }
        }
    }

    fn target(&mut self, p: &ast::Path) -> Target {
        let binding = self.binding(&p.name);
        let component = p.component.as_ref().map(|c| self.axis(c));
        Target { binding, component }
    }

    pub(crate) fn binding(&mut self, n: &ast::Name) -> Id {
        match self.bindings.get(&n.text) {
            Some(id) => id.clone(),
            None => {
                self.err("SX-E03", format!("unknown binding `{}` in model `{}`", n.text, self.name), n.span);
                format!("{}.{}", self.name, n.text)
            }
        }
    }

    pub(crate) fn event_id(&mut self, n: &ast::Name) -> Id {
        match self.events.get(&n.text) {
            Some(id) => id.clone(),
            None => {
                self.err("SX-E03", format!("unknown event `{}` in model `{}`", n.text, self.name), n.span);
                format!("{}.event.{}", self.name, n.text)
            }
        }
    }

    fn axis(&mut self, n: &ast::Name) -> usize {
        let axes: Vec<String> = match self.space {
            Some(s) => s.axes.clone(),
            None => ["x", "y", "z"].iter().map(|s| s.to_string()).collect(),
        };
        match axes.iter().position(|a| *a == n.text) {
            Some(i) => i,
            None => {
                self.err("SX-E03", format!("no component `{}`: the axes are {}", n.text, axes.join(", ")), n.span);
                0
            }
        }
    }

    // ------------------------------------------------------------ types

    fn space_id(&mut self, span: Span) -> Id {
        match self.space {
            Some(s) => s.id.clone(),
            None => {
                self.err("SX-E08", format!("model `{}` has no default space: write `model {} in Space`, or name the space", self.name, self.name), span);
                String::new()
            }
        }
    }

    fn dim(&mut self, d: &ast::DimExpr) -> Dim {
        let mut out = Dim::NONE;
        for f in &d.factors {
            match named_dim(&f.name.text) {
                Some(base) => out = out.mul(&base.pow(Ratio::new(f.num, f.den))),
                None => self.err("SX-E04", format!("unknown dimension `{}`", f.name.text), f.name.span),
            }
        }
        out
    }

    fn ty(&mut self, t: &TypeExpr) -> Type {
        let (name, args, span) = match t {
            TypeExpr::Tuple { items, .. } => return Type::Tuple { items: items.iter().map(|i| self.ty(i)).collect() },
            TypeExpr::Named { name, args, span } => (name, args, *span),
        };
        let n = name.text.as_str();
        let arity = |cx: &mut Self, want: &[usize]| {
            if !want.contains(&args.len()) {
                cx.err("SX-E04", format!("`{n}` takes {} type argument(s)", want.iter().map(|w| w.to_string()).collect::<Vec<_>>().join(" or ")), span);
                false
            } else {
                true
            }
        };
        let space_arg = |cx: &mut Self, a: &ast::DimExpr| -> Option<Id> {
            match a.factors.as_slice() {
                [f] if f.num == 1 && f.den == 1 => cx.spaces.iter().find(|s| s.name == f.name.text).map(|s| s.id.clone()),
                _ => None,
            }
        };
        match n {
            "Real" | "Angle" if arity(self, &[0]) => Type::real(),
            "Boolean" if arity(self, &[0]) => Type::Boolean,
            "Integer" if arity(self, &[0]) => Type::Integer,
            "Quantity" if arity(self, &[1]) => Type::Quantity { dim: self.dim(&args[0]) },
            "Instant" if arity(self, &[1]) => Type::Instant { dim: self.dim(&args[0]) },
            "Point" if arity(self, &[0, 1]) => match args.first() {
                None => Type::Point { space: self.space_id(span) },
                Some(a) => match space_arg(self, a) {
                    Some(s) => Type::Point { space: s },
                    None => {
                        self.err("SX-E03", "unknown space in `Point<...>`", a.span);
                        Type::Point { space: String::new() }
                    }
                },
            },
            "Vector" if arity(self, &[1, 2]) => {
                if args.len() == 2 {
                    let space = match space_arg(self, &args[0]) {
                        Some(s) => s,
                        None => {
                            self.err("SX-E03", "unknown space in `Vector<Space, D>`", args[0].span);
                            String::new()
                        }
                    };
                    Type::Vector { space, dim: self.dim(&args[1]) }
                } else {
                    let space = self.space_id(span);
                    Type::Vector { space, dim: self.dim(&args[0]) }
                }
            }
            "Real" | "Angle" | "Boolean" | "Integer" | "Quantity" | "Instant" | "Point" | "Vector" => Type::real(),
            _ if args.is_empty() && self.enums.contains_key(n) => {
                let (id, cases) = self.enums[n].clone();
                Type::Enum { r#enum: id, cases }
            }
            _ => match named_dim(n) {
                Some(dim) if args.is_empty() => Type::Quantity { dim },
                _ => {
                    self.err("SX-E04", format!("unknown type `{n}`"), name.span);
                    Type::real()
                }
            },
        }
    }

    // ------------------------------------------------------------ expressions

    fn interval(&mut self, x: Expr, i: &ast::Interval, locals: &[String]) -> Expr {
        let lo = match &i.lo {
            Bound::Inf => None,
            Bound::Value(e) => Some(self.expr(e, locals)),
        };
        let hi = match &i.hi {
            Bound::Inf => None,
            Bound::Value(e) => Some(self.expr(e, locals)),
        };
        let lo_cmp = |lo: Expr| if i.lo_closed { build::le(lo, x.clone()) } else { build::lt(lo, x.clone()) };
        let hi_cmp = |hi: Expr| if i.hi_closed { build::le(x.clone(), hi) } else { build::lt(x.clone(), hi) };
        match (lo, hi) {
            (Some(l), Some(h)) => build::and(lo_cmp(l), hi_cmp(h)),
            // One-sided: the value on the left, as `where x >= lo` writes it, so that both
            // spellings give one IR (working syntax section 1.4).
            (Some(l), None) => {
                if i.lo_closed {
                    build::ge(x.clone(), l)
                } else {
                    build::gt(x.clone(), l)
                }
            }
            (None, Some(h)) => hi_cmp(h),
            (None, None) => {
                self.err("SX-E08", "an interval needs at least one finite bound", i.span);
                build::boolean(true)
            }
        }
    }

    fn placeholder() -> Expr {
        build::lit(0.0)
    }

    pub(crate) fn expr(&mut self, e: &ast::Expr, locals: &[String]) -> Expr {
        use ast::BinOp as B;
        match &e.kind {
            ExprKind::Num(v, unit) => match unit {
                None => build::lit(*v),
                Some(u) => match Unit::parse(&u.text) {
                    Ok(unit) => Expr::Num { num: *v, unit: Some(unit) },
                    Err(msg) => {
                        self.err("SX-E05", format!("{msg} (in a model)"), u.span);
                        Self::placeholder()
                    }
                },
            },
            ExprKind::Bool(b) => build::boolean(*b),
            ExprKind::Name(n) => self.name(n, e.span, locals),
            // A minus sign on a number literal writes a negative literal (IR-1.1).
            ExprKind::Neg(x) => match self.expr(x, locals) {
                Expr::Num { num, unit } if matches!(x.kind, ExprKind::Num(..)) => Expr::Num { num: -num, unit },
                e => -e,
            },
            ExprKind::Not(x) => build::not(self.expr(x, locals)),
            ExprKind::Norm(x) => build::norm(self.expr(x, locals)),
            ExprKind::Binary(op, l, r) => {
                let (l, r) = (self.expr(l, locals), self.expr(r, locals));
                let op = match op {
                    B::Add => BinOp::Add,
                    B::Sub => BinOp::Sub,
                    B::Mul => BinOp::Mul,
                    B::Div => BinOp::Div,
                    B::Pow => BinOp::Pow,
                    B::And => BinOp::And,
                    B::Or => BinOp::Or,
                };
                Expr::bin(op, l, r)
            }
            ExprKind::Compare(first, rest) => {
                // Chained comparisons lower to `and` of pairwise comparisons (04-ir section 6).
                let mut left = self.expr(first, locals);
                let mut out: Option<Expr> = None;
                for (op, r) in rest {
                    let right = self.expr(r, locals);
                    let op = match op {
                        ast::CmpOp::Eq => BinOp::Eq,
                        ast::CmpOp::Ne => BinOp::Ne,
                        ast::CmpOp::Lt => BinOp::Lt,
                        ast::CmpOp::Le => BinOp::Le,
                        ast::CmpOp::Gt => BinOp::Gt,
                        ast::CmpOp::Ge => BinOp::Ge,
                    };
                    let c = Expr::bin(op, left, right.clone());
                    out = Some(match out {
                        None => c,
                        Some(prev) => build::and(prev, c),
                    });
                    left = right;
                }
                out.expect("a comparison has at least one operator")
            }
            ExprKind::In(x, i) => {
                let x = self.expr(x, locals);
                self.interval(x, i, locals)
            }
            ExprKind::If(c, a, b) => build::ite(self.expr(c, locals), self.expr(a, locals), self.expr(b, locals)),
            ExprKind::Otherwise(a, b) => Expr::Otherwise { otherwise: Box::new(self.expr(a, locals)), default: Box::new(self.expr(b, locals)) },
            ExprKind::Call(f, args) => self.call(f, args, e.span, locals),
            ExprKind::Field(x, n) if matches!(&x.kind, ExprKind::Name(e) if self.enums.contains_key(e) && !self.bindings.contains_key(e) && !locals.contains(e)) => {
                let ExprKind::Name(e) = &x.kind else { unreachable!() };
                if !self.enums[e].1.contains(&n.text) {
                    self.err("SX-E03", format!("`{}` is not a case of `{e}`", n.text), n.span);
                }
                Expr::Case { case: n.text.clone() }
            }
            ExprKind::Field(x, n) => {
                let x = self.expr(x, locals);
                let axis = self.axis(n);
                build::comp(x, axis)
            }
            ExprKind::Tuple(items) => build::tuple(items.iter().map(|i| self.expr(i, locals)).collect()),
            ExprKind::Str(_) => {
                self.err("SX-E08", "text belongs to presentations, not to a model", e.span);
                Self::placeholder()
            }
            ExprKind::Index(..) => {
                self.err("SX-E06", "indexing needs collections, which the v0 IR does not have", e.span);
                Self::placeholder()
            }
            ExprKind::Match(x, arms) => {
                let scrutinee = self.expr(x, locals);
                let arms = arms.iter().map(|(c, v)| Arm { case: c.text.clone(), value: self.expr(v, locals) }).collect();
                Expr::Match { r#match: Box::new(scrutinee), arms }
            }
            ExprKind::List(_) | ExprKind::Map(..) | ExprKind::On(..) | ExprKind::BeatTime { .. } => {
                self.err("SX-E08", "this form belongs to presentations and runs, not to a model", e.span);
                Self::placeholder()
            }
        }
    }

    fn name(&mut self, n: &str, span: Span, locals: &[String]) -> Expr {
        if let Some(i) = locals.iter().position(|l| l == n) {
            return build::param(i);
        }
        if let Some(id) = self.bindings.get(n) {
            return build::r(id);
        }
        if let Some(id) = self.functions.get(n) {
            return Expr::Fn { r#fn: id.clone() };
        }
        // A case of a declared enumeration; its type comes from the context (D-049).
        if self.enums.values().any(|(_, cases)| cases.iter().any(|c| c == n)) {
            return Expr::Case { case: n.to_string() };
        }
        match n {
            "t" => build::t(),
            "t0" => build::t0(),
            "elapsed" => build::elapsed(),
            "π" | "pi" => build::pi(),
            "origin" => {
                let s = self.space_id(span);
                build::origin(&s)
            }
            "inf" => {
                self.err("SX-E08", "`inf` is only an interval bound", span);
                Self::placeholder()
            }
            _ => {
                let hint = if self.events.contains_key(n) { " (an event is not a value)" } else { "" };
                self.err("SX-E03", format!("unknown name `{n}` in model `{}`{hint}", self.name), span);
                Self::placeholder()
            }
        }
    }

    fn call(&mut self, f: &ast::Expr, args: &[ast::Arg], span: Span, locals: &[String]) -> Expr {
        for a in args {
            if let Some(n) = &a.name {
                self.err("SX-E08", "named arguments belong to representations, not to model expressions", n.span);
            }
            if let Some(e) = &a.every {
                self.err("SX-E08", "a sampled source (`every`) belongs to a representation, not to a model expression", e.span);
            }
        }
        let name = match &f.kind {
            ExprKind::Name(n) => n.as_str(),
            _ => {
                let callee = self.expr(f, locals);
                let args = args.iter().map(|a| self.expr(&a.value, locals)).collect();
                return build::apply(callee, args);
            }
        };
        if name == "der" && !locals.iter().any(|l| l == name) && !self.bindings.contains_key(name) {
            return match args {
                [ast::Arg { value: ast::Expr { kind: ExprKind::Name(x), span: xs }, .. }] => {
                    let id = self.binding(&ast::Name { text: x.clone(), span: *xs });
                    Expr::Der { der: id }
                }
                _ => {
                    self.err("SX-E08", "`der` takes the name of one continuous state", span);
                    Self::placeholder()
                }
            };
        }
        let lowered: Vec<Expr> = args.iter().map(|a| self.expr(&a.value, locals)).collect();
        if locals.iter().any(|l| l == name) || self.bindings.contains_key(name) || self.functions.contains_key(name) {
            let callee = self.name(name, f.span, locals);
            return build::apply(callee, lowered);
        }
        if let Some((_, func, n)) = FUNCS.iter().find(|(s, _, _)| *s == name) {
            if lowered.len() != *n {
                self.err("SX-E08", format!("`{name}` takes {n} argument(s), found {}", lowered.len()), span);
            }
            return build::call(*func, lowered);
        }
        self.err("SX-E03", format!("unknown function `{name}`"), f.span);
        Self::placeholder()
    }
}
