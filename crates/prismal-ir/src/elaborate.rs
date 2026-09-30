//! Elaboration of contained objects and collections (MK sections 7 and 8, D-055).
//!
//! A model with parts is expanded into a flat model: each member of each part becomes the
//! bindings, processes, functions, flows, events, equations and constraints of its object
//! type, with identities `declaration@path` and names `path.name`, where the path of a member
//! is `ball`, `row[2]` or, nested, `cart.wheels[1]` (MK-7.6). Member expressions (`ball.pos`,
//! `row[2].pos`), aggregates (`sum(b.m for b in row)`), container flows over members
//! (`for b in row { der(b.vel) += ... }`) and representations repeated per member are
//! replaced by what they denote. The result is an ordinary model, checked and run as any
//! other; elaboration only moves and renames what was written.
//!
//! Membership is fixed in v0: a part holds one object, or `count` members.

use crate::present::{Action, Arg, Check, Inverse, Item, LearnerInput, Observation, Operand, Presentation, Prop, Rep, RunCase, Schedule, Source, Subject, Tolerance, ViewKind};
use crate::{Agg, BinOp, Builtin, Document, Each, Event, Expr, Flow, Id, Model, Op, Target, Trigger, Zeno, ZenoPolicy};

/// A problem found while elaborating; `element` is the identity of what was written.
#[derive(Clone, Debug, PartialEq)]
pub struct Diag {
    pub code: &'static str,
    pub message: String,
    pub element: Id,
}

/// The identity of the element declared as `id` in the member at `path`.
pub fn flat(id: &str, path: &str) -> Id {
    if path.is_empty() {
        id.to_string()
    } else {
        format!("{id}@{path}")
    }
}

/// The identity of the declaration an elaborated element comes from.
pub fn declaration(id: &str) -> &str {
    id.split('@').next().unwrap_or(id)
}

fn join(a: &str, b: &str) -> String {
    if a.is_empty() {
        b.to_string()
    } else {
        format!("{a}.{b}")
    }
}

/// A member: its path and its object type.
#[derive(Clone)]
struct Inst<'a> {
    path: String,
    ty: &'a Model,
}

/// Where the identities of an expression resolve: the object type whose declarations it
/// refers to, the member's path (empty for the model itself), loop variables, and the
/// member number in the overrides of a collection.
#[derive(Clone)]
struct Scope<'a> {
    ty: &'a Model,
    path: String,
    vars: Vec<(String, Inst<'a>)>,
    index: Option<u32>,
}

impl<'a> Scope<'a> {
    fn root(m: &'a Model) -> Scope<'a> {
        Scope { ty: m, path: String::new(), vars: vec![], index: None }
    }
    fn with_var(&self, var: &str, inst: Inst<'a>) -> Scope<'a> {
        let mut s = self.clone();
        s.vars.push((var.to_string(), inst));
        s
    }
    fn local(&self, id: &str) -> Id {
        flat(id, &self.path)
    }
}

struct Cx<'a> {
    root: &'a Model,
    diags: Vec<Diag>,
    element: Id,
}

impl<'a> Cx<'a> {
    fn err(&mut self, code: &'static str, message: String) {
        let d = Diag { code, message, element: self.element.clone() };
        if !self.diags.contains(&d) {
            self.diags.push(d);
        }
    }

    fn object(&mut self, id: &str) -> Option<&'a Model> {
        let o = self.root.objects.iter().find(|o| o.id == id);
        if o.is_none() {
            self.err("MK-E26", format!("unknown object type `{id}`"));
        }
        o
    }

    /// The members of the part `id` of the object type in `s`: one for a contained object,
    /// `count` for a collection.
    fn members(&mut self, s: &Scope<'a>, id: &str) -> Option<(Vec<Inst<'a>>, bool)> {
        let Some(p) = s.ty.part(id) else {
            self.err("MK-E26", format!("unknown part `{id}`"));
            return None;
        };
        let ty = self.object(&p.object)?;
        Some(match p.count {
            None => (vec![Inst { path: join(&s.path, &p.name), ty }], false),
            Some(n) => ((1..=n).map(|k| Inst { path: join(&s.path, &format!("{}[{k}]", p.name)), ty }).collect(), true),
        })
    }

    /// The member an expression selects: a loop variable, a contained object, or a member of
    /// a collection by a constant number.
    fn member(&mut self, s: &Scope<'a>, e: &Expr) -> Option<Inst<'a>> {
        match e {
            Expr::Var { var } => match s.vars.iter().rev().find(|(v, _)| v == var) {
                Some((_, i)) => Some(i.clone()),
                None => {
                    self.err("MK-E26", format!("`{var}` is not a member here: it is named only inside its `for` or aggregate"));
                    None
                }
            },
            Expr::Part { part } => {
                let (ms, many) = self.members(s, part)?;
                if many {
                    self.err("MK-E26", format!("`{}` is a collection: select a member, `{}[1]`", part_name(s.ty, part), part_name(s.ty, part)));
                    return None;
                }
                ms.into_iter().next()
            }
            Expr::Item { item, index } => {
                let (ms, many) = self.members(s, item)?;
                if !many {
                    self.err("MK-E26", format!("`{}` is one object, not a collection", part_name(s.ty, item)));
                    return None;
                }
                let k = match self.constant_index(s, index) {
                    Some(k) => k,
                    None => {
                        self.err("MK-E26", format!("a member of `{}` is selected by a constant number", part_name(s.ty, item)));
                        return None;
                    }
                };
                if k < 1 || k as usize > ms.len() {
                    self.err("MK-E26", format!("`{}` has members 1 to {}, not {k}", part_name(s.ty, item), ms.len()));
                    return None;
                }
                ms.into_iter().nth(k as usize - 1)
            }
            _ => None,
        }
    }

    fn constant_index(&mut self, s: &Scope<'a>, e: &Expr) -> Option<i64> {
        match e {
            Expr::Num { num, unit: None } if num.fract() == 0.0 => Some(*num as i64),
            Expr::Builtin { builtin: Builtin::Index } => s.index.map(|k| k as i64),
            Expr::Neg { neg } => self.constant_index(s, neg).map(|k| -k),
            Expr::Bin { bin, l, r } => {
                let (a, b) = (self.constant_index(s, l)?, self.constant_index(s, r)?);
                match bin {
                    BinOp::Add => Some(a + b),
                    BinOp::Sub => Some(a - b),
                    BinOp::Mul => Some(a * b),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn is_member(e: &Expr) -> bool {
        matches!(e, Expr::Var { .. } | Expr::Part { .. } | Expr::Item { .. })
    }

    /// An expression with members and aggregates replaced, and identities declared in the
    /// scope's object type moved to its member.
    fn expr(&mut self, s: &Scope<'a>, e: &Expr) -> Expr {
        let b = |cx: &mut Self, x: &Expr| Box::new(cx.expr(s, x));
        match e {
            Expr::Ref { r#ref } => Expr::Ref { r#ref: s.local(r#ref) },
            Expr::Der { der } => Expr::Der { der: s.local(der) },
            Expr::Fn { r#fn } => Expr::Fn { r#fn: s.local(r#fn) },
            Expr::Payload { payload } => Expr::Payload { payload: s.local(payload) },
            Expr::Builtin { builtin: Builtin::Index } => match s.index {
                Some(k) => Expr::Num { num: k as f64, unit: None },
                None => e.clone(),
            },
            Expr::Field { field, of } => {
                let Some(inst) = self.member(s, of) else { return Expr::Num { num: 0.0, unit: None } };
                if inst.ty.binding(field).is_none() {
                    self.err("MK-E26", format!("an object `{}` has no binding `{}`", inst.ty.name, field.rsplit('.').next().unwrap_or(field)));
                }
                Expr::Ref { r#ref: flat(field, &inst.path) }
            }
            Expr::Part { .. } | Expr::Item { .. } | Expr::Var { .. } => {
                self.err("MK-E26", "a member is not a value: read one of its bindings, `b.pos`".into());
                Expr::Num { num: 0.0, unit: None }
            }
            // Members compare by identity, which elaboration knows: `o != b` is a constant.
            Expr::Bin { bin: bin @ (BinOp::Eq | BinOp::Ne), l, r } if Self::is_member(l) && Self::is_member(r) => {
                let (a, c) = (self.member(s, l), self.member(s, r));
                let same = matches!((&a, &c), (Some(x), Some(y)) if x.path == y.path);
                Expr::Bool { bool: same == (*bin == BinOp::Eq) }
            }
            Expr::Aggregate { aggregate, var, over, body, filter } => self.aggregate(s, *aggregate, var, over, body.as_deref(), filter.as_deref()),
            Expr::Neg { neg } => Expr::Neg { neg: b(self, neg) },
            Expr::Not { not } => Expr::Not { not: b(self, not) },
            Expr::Norm { norm } => Expr::Norm { norm: b(self, norm) },
            Expr::Bin { bin, l, r } => Expr::Bin { bin: *bin, l: b(self, l), r: b(self, r) },
            Expr::Call { call, args } => Expr::Call { call: *call, args: args.iter().map(|a| self.expr(s, a)).collect() },
            Expr::Apply { apply, args } => Expr::Apply { apply: b(self, apply), args: args.iter().map(|a| self.expr(s, a)).collect() },
            Expr::If { r#if, then, r#else } => Expr::If { r#if: b(self, r#if), then: b(self, then), r#else: b(self, r#else) },
            Expr::Tuple { tuple } => Expr::Tuple { tuple: tuple.iter().map(|a| self.expr(s, a)).collect() },
            Expr::Comp { comp, axis } => Expr::Comp { comp: b(self, comp), axis: *axis },
            Expr::Lambda { lambda } => {
                let mut l = lambda.clone();
                l.body = b(self, &lambda.body);
                Expr::Lambda { lambda: l }
            }
            Expr::Otherwise { otherwise, default } => Expr::Otherwise { otherwise: b(self, otherwise), default: b(self, default) },
            Expr::Match { r#match, arms } => Expr::Match {
                r#match: b(self, r#match),
                arms: arms.iter().map(|a| crate::Arm { case: a.case.clone(), value: self.expr(s, &a.value) }).collect(),
            },
            Expr::Num { .. } | Expr::Bool { .. } | Expr::Case { .. } | Expr::Param { .. } | Expr::Builtin { .. } | Expr::Const { .. } | Expr::Origin { .. } => e.clone(),
        }
    }

    fn aggregate(&mut self, s: &Scope<'a>, agg: Agg, var: &str, over: &str, body: Option<&Expr>, filter: Option<&Expr>) -> Expr {
        let zero = Expr::Num { num: 0.0, unit: None };
        let Some((members, many)) = self.members(s, over) else { return zero };
        if !many {
            self.err("MK-E26", format!("`{}` is one object; an aggregate goes over a collection", part_name(s.ty, over)));
            return zero;
        }
        let mut terms: Vec<Expr> = vec![];
        for inst in members {
            let inner = s.with_var(var, inst);
            let cond = filter.map(|f| self.expr(&inner, f));
            if matches!(cond, Some(Expr::Bool { bool: false })) {
                continue;
            }
            let dynamic = cond.filter(|c| !matches!(c, Expr::Bool { bool: true }));
            let value = match body {
                Some(e) => self.expr(&inner, e),
                None => Expr::Num { num: 1.0, unit: None },
            };
            terms.push(match (agg, dynamic) {
                (_, None) => value,
                (Agg::Sum | Agg::Count, Some(c)) => Expr::If { r#if: Box::new(c), then: Box::new(value), r#else: Box::new(zero.clone()) },
                (Agg::Any, Some(c)) => Expr::bin(BinOp::And, c, value),
                (Agg::All, Some(c)) => Expr::bin(BinOp::Or, Expr::Not { not: Box::new(c) }, value),
                (Agg::Min | Agg::Max, Some(_)) => {
                    self.err("MK-E26", format!("a filter of `{}` compares members only (`if o != b`)", agg.name()));
                    return zero;
                }
            });
        }
        let fold = |terms: Vec<Expr>, f: &dyn Fn(Expr, Expr) -> Expr| terms.into_iter().reduce(|a, b| f(a, b));
        match agg {
            Agg::Sum | Agg::Count => fold(terms, &|a, b| Expr::bin(BinOp::Add, a, b)).unwrap_or(zero),
            Agg::Any => fold(terms, &|a, b| Expr::bin(BinOp::Or, a, b)).unwrap_or(Expr::Bool { bool: false }),
            Agg::All => fold(terms, &|a, b| Expr::bin(BinOp::And, a, b)).unwrap_or(Expr::Bool { bool: true }),
            Agg::Min | Agg::Max => {
                let func = if agg == Agg::Min { crate::Func::Min } else { crate::Func::Max };
                match fold(terms, &|a, b| Expr::Call { call: func, args: vec![a, b] }) {
                    Some(e) => e,
                    None => {
                        self.err("MK-E26", format!("`{}` over no member has no value", agg.name()));
                        zero
                    }
                }
            }
        }
    }

    fn target(&mut self, s: &Scope<'a>, t: &Target) -> Target {
        Target { binding: s.local(&t.binding), component: t.component }
    }

    fn op(&mut self, s: &Scope<'a>, o: &Op) -> Op {
        match o {
            Op::Set { target, value } => Op::Set { target: self.target(s, target), value: self.expr(s, value) },
            Op::Contribute { target, value } => Op::Contribute { target: self.target(s, target), value: self.expr(s, value) },
            Op::Emit { event, payload } => Op::Emit { event: s.local(event), payload: payload.as_ref().map(|p| self.expr(s, p)) },
        }
    }

    fn event(&mut self, s: &Scope<'a>, e: &Event) -> Event {
        self.element = e.id.clone();
        let trigger = match &e.trigger {
            Trigger::Rising { guard } => Trigger::Rising { guard: self.expr(s, guard) },
            Trigger::Falling { guard } => Trigger::Falling { guard: self.expr(s, guard) },
            Trigger::Crossing { guard } => Trigger::Crossing { guard: self.expr(s, guard) },
            Trigger::At { time } => Trigger::At { time: self.expr(s, time) },
            Trigger::Every { period, from } => Trigger::Every { period: self.expr(s, period), from: self.expr(s, from) },
            Trigger::On { event } => Trigger::On { event: s.local(event) },
            Trigger::Input { binding } => Trigger::Input { binding: s.local(binding) },
            Trigger::Level { cond } => Trigger::Level { cond: self.expr(s, cond) },
            Trigger::Start => Trigger::Start,
            Trigger::Request => Trigger::Request,
        };
        Event {
            id: s.local(&e.id),
            name: join(&s.path, &e.name),
            trigger,
            enable: e.enable.as_ref().map(|c| self.expr(s, c)),
            handler: e.handler.iter().map(|o| self.op(s, o)).collect(),
            zeno: e.zeno.as_ref().map(|z| Zeno {
                policy: match &z.policy {
                    ZenoPolicy::Stop => ZenoPolicy::Stop,
                    ZenoPolicy::Settle { ops } => ZenoPolicy::Settle { ops: ops.iter().map(|o| self.op(s, o)).collect() },
                },
                ..z.clone()
            }),
            process: e.process.as_ref().map(|p| s.local(p)),
            payload: e.payload.clone(),
            notes: e.notes.clone(),
        }
    }

    /// The flows of `f` in scope `s`: one, or one per member of its loop.
    fn flows(&mut self, s: &Scope<'a>, f: &Flow, out: &mut Vec<Flow>) {
        self.element = f.id.clone();
        let one = |cx: &mut Self, scope: &Scope<'a>, id: Id, out: &mut Vec<Flow>| {
            let target = match &f.member {
                None => scope.local(&f.target),
                Some(m) => match cx.member(scope, m) {
                    Some(inst) => {
                        if inst.ty.binding(&f.target).is_none() {
                            cx.err("MK-E26", format!("an object `{}` has no binding `{}`", inst.ty.name, f.target.rsplit('.').next().unwrap_or(&f.target)));
                        }
                        flat(&f.target, &inst.path)
                    }
                    None => return,
                },
            };
            let expr = cx.expr(scope, &f.expr);
            out.push(Flow { id, target, kind: f.kind, expr, process: f.process.as_ref().map(|p| s.local(p)), member: None, each: None });
        };
        match &f.each {
            None => one(self, s, s.local(&f.id), out),
            Some(Each { var, over }) => {
                let Some((members, _)) = self.members(s, over) else { return };
                for inst in members {
                    let id = flat(&f.id, &inst.path);
                    let inner = s.with_var(var, inst);
                    one(self, &inner, id, out);
                }
            }
        }
    }

    /// Everything a scope's object type declares, moved to the scope's member, then its parts.
    fn body(&mut self, s: &Scope<'a>, overrides: &[(Id, Expr)], out: &mut Model, depth: usize) {
        let ty = s.ty;
        for b in &ty.bindings {
            self.element = b.id.clone();
            let mut nb = b.clone();
            nb.id = s.local(&b.id);
            nb.name = join(&s.path, &b.name);
            nb.init = b.init.as_ref().map(|e| self.expr(s, e));
            nb.def = b.def.as_ref().map(|e| self.expr(s, e));
            if let Some((_, v)) = overrides.iter().find(|(id, _)| id == &b.id) {
                match b.role {
                    // MK-7.11: a connection makes the input follow an expression of the container.
                    crate::Role::Input => {
                        nb.role = crate::Role::Derived;
                        nb.init = None;
                        nb.def = Some(v.clone());
                    }
                    crate::Role::Derived => self.err("MK-E26", format!("`{}` is derived: it cannot be given a value when the object is declared", b.name)),
                    _ => nb.init = Some(v.clone()),
                }
            }
            out.bindings.push(nb);
        }
        for f in &ty.functions {
            self.element = f.id.clone();
            let mut nf = f.clone();
            nf.id = s.local(&f.id);
            nf.name = join(&s.path, &f.name);
            nf.body = self.expr(s, &f.body);
            out.functions.push(nf);
        }
        for p in &ty.processes {
            let mut np = p.clone();
            np.id = s.local(&p.id);
            np.name = join(&s.path, &p.name);
            out.processes.push(np);
        }
        for f in &ty.flows {
            self.flows(s, f, &mut out.flows);
        }
        for e in &ty.events {
            let ne = self.event(s, e);
            out.events.push(ne);
        }
        for q in &ty.equations {
            self.element = q.id.clone();
            let mut nq = q.clone();
            nq.id = s.local(&q.id);
            nq.name = join(&s.path, &q.name);
            nq.lhs = self.expr(s, &q.lhs);
            nq.rhs = self.expr(s, &q.rhs);
            nq.tol = q.tol.as_ref().map(|t| self.expr(s, t));
            out.equations.push(nq);
        }
        for c in &ty.constraints {
            self.element = c.id.clone();
            let mut nc = c.clone();
            nc.id = s.local(&c.id);
            nc.name = join(&s.path, &c.name);
            nc.cond = self.expr(s, &c.cond);
            nc.tol = c.tol.as_ref().map(|t| self.expr(s, t));
            nc.attached_to = c.attached_to.as_ref().map(|a| s.local(a));
            out.constraints.push(nc);
        }
        for p in &ty.parts {
            self.element = p.id.clone();
            if depth > 16 {
                self.err("MK-E26", format!("objects contain each other without end, through `{}`", p.name));
                return;
            }
            let Some((members, _)) = self.members(s, &p.id) else { continue };
            for (k, inst) in members.into_iter().enumerate() {
                // Overrides are read in the container's scope, with `index` the member's number.
                let mut outer = s.clone();
                outer.index = p.count.map(|_| k as u32 + 1);
                let ov: Vec<(Id, Expr)> = p.overrides.iter().map(|o| (o.binding.clone(), self.expr(&outer, &o.value))).collect();
                for (id, _) in &ov {
                    if inst.ty.binding(id).is_none() {
                        self.err("MK-E26", format!("an object `{}` has no binding `{}`", inst.ty.name, id.rsplit('.').next().unwrap_or(id)));
                    }
                }
                let inner = Scope { ty: inst.ty, path: inst.path.clone(), vars: vec![], index: None };
                self.body(&inner, &ov, out, depth + 1);
            }
        }
    }
}

fn part_name(ty: &Model, id: &str) -> String {
    ty.part(id).map(|p| p.name.clone()).unwrap_or_else(|| id.rsplit('.').next().unwrap_or(id).to_string())
}

/// The flat model of `m`: `m` itself when it has no parts.
pub fn model(m: &Model) -> Result<Model, Vec<Diag>> {
    if !m.has_parts() && m.objects.is_empty() {
        return Ok(m.clone());
    }
    let mut cx = Cx { root: m, diags: vec![], element: m.id.clone() };
    let mut out = Model { bindings: vec![], processes: vec![], flows: vec![], events: vec![], equations: vec![], constraints: vec![], functions: vec![], objects: vec![], parts: vec![], ..m.clone() };
    // Enumerations are types: one declaration serves every member (MK-2.2).
    for o in &m.objects {
        out.enums.extend(o.enums.iter().cloned());
    }
    cx.body(&Scope::root(m), &[], &mut out, 0);
    if cx.diags.is_empty() {
        Ok(out)
    } else {
        Err(cx.diags)
    }
}

// ---------------------------------------------------------------- presentations and runs

struct PCx<'a, 'b> {
    cx: &'b mut Cx<'a>,
    root: Scope<'a>,
    /// Representations repeated per member: the identity written and its members'.
    families: Vec<(Id, Vec<Id>)>,
}

impl<'a> PCx<'a, '_> {
    fn e(&mut self, s: &Scope<'a>, e: &Expr) -> Expr {
        self.cx.expr(s, e)
    }

    fn arg(&mut self, s: &Scope<'a>, a: &Arg) -> Arg {
        match a {
            Arg::Expr { expr } => Arg::Expr { expr: self.e(s, expr) },
            Arg::Range { lo, hi } => Arg::Range { lo: self.e(s, lo), hi: self.e(s, hi) },
            Arg::Sampled { expr, every } => Arg::Sampled { expr: self.e(s, expr), every: self.e(s, every) },
            Arg::Scale { scale } => Arg::Scale { scale: crate::present::Scale { quantity: self.e(s, &scale.quantity), px: scale.px } },
            Arg::Text { .. } | Arg::Word { .. } | Arg::Element { .. } => a.clone(),
        }
    }

    fn reps(&mut self, s: &Scope<'a>, reps: &[Rep]) -> Vec<Rep> {
        let mut out = vec![];
        for r in reps {
            self.cx.element = r.id.clone();
            match &r.each {
                None => out.push(self.rep(s, r, None)),
                Some(Each { var, over }) => {
                    let Some((members, _)) = self.cx.members(s, over) else { continue };
                    let mut ids = vec![];
                    for (k, inst) in members.into_iter().enumerate() {
                        let inner = s.with_var(var, inst);
                        let nr = self.rep(&inner, r, Some(k + 1));
                        ids.push(nr.id.clone());
                        out.push(nr);
                    }
                    self.families.push((r.id.clone(), ids));
                }
            }
        }
        out
    }

    fn rep(&mut self, s: &Scope<'a>, r: &Rep, k: Option<usize>) -> Rep {
        let suffix = |x: &str| match k {
            Some(k) => format!("{x}[{k}]"),
            None => x.to_string(),
        };
        Rep {
            each: None,
            id: suffix(&r.id),
            name: r.name.as_deref().map(suffix),
            kind: r.kind.clone(),
            sources: r.sources.iter().map(|a| self.arg(s, a)).collect(),
            props: r.props.iter().map(|p| Prop { name: p.name.clone(), value: self.arg(s, &p.value) }).collect(),
            inverse: r.inverse.as_ref().map(|inv| Inverse {
                gesture: inv.gesture.clone(),
                part: inv.part.clone(),
                proposals: inv.proposals.iter().map(|p| crate::present::Proposal { target: p.target.clone(), value: self.e(s, &p.value) }).collect(),
            }),
            members: self.reps(s, &r.members),
        }
    }

    fn actions(&mut self, acts: &[Action]) -> Vec<Action> {
        let s = self.root.clone();
        let mut out = vec![];
        for a in acts {
            let ex = |p: &mut Self, e: &Option<Expr>| e.as_ref().map(|x| p.e(&s, x));
            let na = match a {
                Action::Show { view, reps } => Action::Show { view: view.clone(), reps: self.reps(&s, reps) },
                Action::Reveal { view, style, duration, reps } => Action::Reveal { view: view.clone(), style: *style, duration: ex(self, duration), reps: self.reps(&s, reps) },
                Action::Camera { view, center, zoom, duration } => Action::Camera { view: view.clone(), center: ex(self, center), zoom: ex(self, zoom), duration: ex(self, duration) },
                Action::Narrate { text, duration } => Action::Narrate { text: text.clone(), duration: ex(self, duration) },
                Action::Run { rate } => Action::Run { rate: self.e(&s, rate) },
                Action::Seek { time } => Action::Seek { time: self.e(&s, time) },
                Action::Intervene { ops } => Action::Intervene { ops: ops.iter().map(|o| self.cx.op(&s, o)).collect() },
                Action::Request { event, payload } => Action::Request { event: event.clone(), payload: ex(self, payload) },
                Action::Wait { duration } => Action::Wait { duration: self.e(&s, duration) },
                Action::Explore { limit, keep, controls, fallback } => Action::Explore { limit: ex(self, limit), keep: keep.clone(), controls: self.reps(&s, controls), fallback: self.actions(fallback) },
                Action::Sequence { actions } => Action::Sequence { actions: self.actions(actions) },
                // A family of representations is highlighted or hidden member by member.
                Action::Highlight { target } | Action::Hide { target, .. } if self.families.iter().any(|(id, _)| id == target) => {
                    let ids = self.families.iter().find(|(id, _)| id == target).unwrap().1.clone();
                    for id in ids {
                        out.push(match a {
                            Action::Highlight { .. } => Action::Highlight { target: id },
                            Action::Hide { duration, .. } => Action::Hide { target: id, duration: ex(self, duration) },
                            _ => unreachable!(),
                        });
                    }
                    continue;
                }
                Action::Hide { target, duration } => Action::Hide { target: target.clone(), duration: ex(self, duration) },
                other => other.clone(),
            };
            out.push(na);
        }
        out
    }
}

fn presentation<'a>(cx: &mut Cx<'a>, root: &'a Model, p: &Presentation) -> Presentation {
    let mut pc = PCx { cx, root: Scope::root(root), families: vec![] };
    let s = pc.root.clone();
    let mut out = p.clone();
    out.observations = p
        .observations
        .iter()
        .map(|o| {
            pc.cx.element = o.id.clone();
            Observation {
                source: match &o.source {
                    Source::Expr { expr } => Source::Expr { expr: pc.e(&s, expr) },
                    other => other.clone(),
                },
                schedule: match &o.schedule {
                    Schedule::Every { period } => Schedule::Every { period: pc.e(&s, period) },
                    Schedule::At { time } => Schedule::At { time: pc.e(&s, time) },
                    Schedule::Over { from, to, lo_closed, hi_closed } => Schedule::Over { from: pc.e(&s, from), to: to.as_ref().map(|t| pc.e(&s, t)), lo_closed: *lo_closed, hi_closed: *hi_closed },
                    other => other.clone(),
                },
                ..o.clone()
            }
        })
        .collect();
    for v in &mut out.views {
        pc.cx.element = v.id.clone();
        v.kind = match &v.kind {
            ViewKind::Spatial { space, scale, y_up } => ViewKind::Spatial { space: space.clone(), scale: crate::present::Scale { quantity: pc.e(&s, &scale.quantity), px: scale.px }, y_up: *y_up },
            ViewKind::Plot { x, y } => ViewKind::Plot { x: [pc.e(&s, &x[0]), pc.e(&s, &x[1])], y: [pc.e(&s, &y[0]), pc.e(&s, &y[1])] },
            ViewKind::Panel => ViewKind::Panel,
        };
        v.representations = pc.reps(&s, &v.representations);
    }
    if let Some(t) = &mut out.timeline {
        for sc in &mut t.scenes {
            for b in &mut sc.beats {
                pc.cx.element = b.id.clone();
                b.actions = pc.actions(&b.actions);
            }
        }
    }
    out
}

fn run<'a>(cx: &mut Cx<'a>, root: &'a Model, r: &RunCase) -> RunCase {
    let s = Scope::root(root);
    cx.element = r.id.clone();
    let mut out = r.clone();
    for o in &mut out.params {
        o.value = cx.expr(&s, &o.value);
    }
    for i in &mut out.inputs {
        i.value = cx.expr(&s, &i.value);
        i.at = i.at.as_ref().map(|a| cx.expr(&s, a));
    }
    out.end = r.end.as_ref().map(|e| cx.expr(&s, e));
    for l in &mut out.learner {
        l.at = cx.expr(&s, &l.at);
        if let LearnerInput::SetControl { value, .. } = &mut l.input {
            *value = cx.expr(&s, value);
        }
    }
    let subject = |cx: &mut Cx<'a>, sub: &Subject| match sub {
        Subject::On { expr, event, microstep } => Subject::On { expr: cx.expr(&s, expr), event: event.clone(), microstep: *microstep },
        other => other.clone(),
    };
    for x in &mut out.expectations {
        cx.element = x.id.clone();
        x.check = match &x.check {
            Check::Equal { subject: sub, expected, tolerance } => Check::Equal {
                subject: subject(cx, sub),
                expected: match expected {
                    Operand::Value { expr } => Operand::Value { expr: cx.expr(&s, expr) },
                    Operand::Subject { subject: o } => Operand::Subject { subject: subject(cx, o) },
                    Operand::List { items } => Operand::List {
                        items: items
                            .iter()
                            .map(|i| match i {
                                Item::Value { expr } => Item::Value { expr: cx.expr(&s, expr) },
                                other => other.clone(),
                            })
                            .collect(),
                    },
                },
                tolerance: match tolerance {
                    Tolerance::Abs { tol } => Tolerance::Abs { tol: cx.expr(&s, tol) },
                    Tolerance::Rel { tol } => Tolerance::Rel { tol: cx.expr(&s, tol) },
                    Tolerance::Exact => Tolerance::Exact,
                },
            },
            Check::Within { subject: sub, lo, hi, lo_closed, hi_closed } => Check::Within { subject: subject(cx, sub), lo: cx.expr(&s, lo), hi: cx.expr(&s, hi), lo_closed: *lo_closed, hi_closed: *hi_closed },
            other => other.clone(),
        };
    }
    out
}

/// Whether anything in the document needs elaboration.
pub fn needed(d: &Document) -> bool {
    d.models.iter().any(|m| m.has_parts() || !m.objects.is_empty())
}

/// The document with every model elaborated, and the presentations and runs of models with
/// parts elaborated against them. A document without parts is returned unchanged.
pub fn document(d: &Document) -> Result<Document, Vec<Diag>> {
    if !needed(d) {
        return Ok(d.clone());
    }
    let mut diags = vec![];
    let mut out = d.clone();
    out.models = vec![];
    for m in &d.models {
        match model(m) {
            Ok(fm) => out.models.push(fm),
            Err(ds) => {
                diags.extend(ds);
                out.models.push(m.clone());
            }
        }
    }
    for (i, p) in d.presentations.iter().enumerate() {
        if let Some(m) = d.model(&p.model).filter(|m| m.has_parts() || !m.objects.is_empty()) {
            let mut cx = Cx { root: m, diags: vec![], element: p.id.clone() };
            out.presentations[i] = presentation(&mut cx, m, p);
            diags.extend(cx.diags);
        }
    }
    for (i, r) in d.runs.iter().enumerate() {
        if let Some(m) = d.model(&r.model).filter(|m| m.has_parts() || !m.objects.is_empty()) {
            let mut cx = Cx { root: m, diags: vec![], element: r.id.clone() };
            out.runs[i] = run(&mut cx, m, r);
            diags.extend(cx.diags);
        }
    }
    if diags.is_empty() {
        Ok(out)
    } else {
        Err(diags)
    }
}
