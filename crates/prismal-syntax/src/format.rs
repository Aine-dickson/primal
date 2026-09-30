//! Canonical printing of the IR as working-syntax text (working syntax section 1.4).
//!
//! The formatter prints from the IR, not from the source: declarations are grouped in blocks
//! by role (const, param, input, state, discrete, derived), then flows, processes, events,
//! equations and constraints; a parameter's range is printed as an interval when it bounds
//! that parameter from both sides, as `where` otherwise; `run rate r` followed by
//! `wait until E` is printed `run rate r until E`; representations are printed inside their
//! views. Author notes (D-036) print as `///` comments before their element. Comments that
//! are not notes are not in the IR and are not printed.
//!
//! Lowering the printed text gives back the same IR up to identities and list order, which
//! `identity::reconcile` restores from the previous IR (D-036, IR-1.7).

use prismal_ir::dim::BASE;
use prismal_ir::present::*;
use prismal_ir::*;
use std::collections::HashMap;
use std::fmt::Write;

const INDENT: &str = "  ";

/// Prints a document in the canonical form.
pub fn format(doc: &Document) -> String {
    let mut parts = vec![];
    for s in &doc.spaces {
        let mut out = String::new();
        notes(&mut out, "", &s.notes);
        let _ = write!(out, "space {} = euclidean({})", s.name, s.dimension);
        parts.push(out);
    }
    for m in &doc.models {
        parts.push(Printer::new(doc, m).model());
    }
    for p in &doc.presentations {
        if let Some(m) = doc.model(&p.model) {
            parts.push(Printer::new(doc, m).presentation(p));
        }
    }
    for r in &doc.runs {
        if let Some(m) = doc.model(&r.model) {
            parts.push(Printer::new(doc, m).run(r));
        }
    }
    let mut s = parts.join("\n\n");
    s.push('\n');
    s
}

fn notes(out: &mut String, ind: &str, notes: &[String]) {
    for n in notes {
        for line in n.lines() {
            if line.is_empty() {
                let _ = writeln!(out, "{ind}///");
            } else {
                let _ = writeln!(out, "{ind}/// {line}");
            }
        }
    }
}

/// The shortest decimal text that reads back as the same number.
pub fn number(x: f64) -> String {
    let plain = format!("{x}");
    let exp = format!("{x:e}");
    if exp.len() < plain.len() {
        exp
    } else {
        plain
    }
}

/// A dimension in base symbols, as authors write it: `L/T^2`, `M L^2/T^2`, `1/L`,
/// `L^(1/2)`; `1` when dimensionless.
pub fn dim(d: &Dim) -> String {
    let (mut num, mut den) = (vec![], vec![]);
    for (i, r) in d.0.iter().enumerate() {
        if r.is_zero() {
            continue;
        }
        let sym = if BASE[i] == "Th" { "Θ" } else { BASE[i] };
        let (n, dd) = (r.n.abs(), r.d);
        let f = match (n, dd) {
            (1, 1) => sym.to_string(),
            (n, 1) => format!("{sym}^{n}"),
            (n, dd) => format!("{sym}^({n}/{dd})"),
        };
        if r.n < 0 {
            den.push(f);
        } else {
            num.push(f);
        }
    }
    let top = if num.is_empty() { "1".to_string() } else { num.join(" ") };
    if den.is_empty() {
        top
    } else {
        format!("{top}/{}", den.join(" "))
    }
}

/// The name of a dimension if it has one (MK-3.3).
fn named(d: &Dim) -> Option<&'static str> {
    const NAMES: [&str; 15] =
        ["Length", "Mass", "Time", "Current", "Amount", "Area", "Volume", "Velocity", "Acceleration", "Frequency", "Momentum", "Force", "Energy", "Power", "Pressure"];
    NAMES.into_iter().find(|n| crate::lower::named_dim(n).as_ref() == Some(d))
}

// Precedence levels of the working syntax's expression grammar (parser.rs).
const P_IF: u8 = 0;
const P_OR: u8 = 1;
const P_AND: u8 = 2;
const P_NOT: u8 = 3;
const P_CMP: u8 = 4;
const P_ADD: u8 = 5;
const P_MUL: u8 = 6;
const P_NEG: u8 = 7;
const P_POW: u8 = 8;
const P_ATOM: u8 = 9;

struct Printer<'a> {
    doc: &'a Document,
    model: &'a Model,
    axes: Vec<String>,
    /// Representation identities to their names, for `highlight`.
    rep_names: HashMap<Id, String>,
}

impl<'a> Printer<'a> {
    fn new(doc: &'a Document, model: &'a Model) -> Printer<'a> {
        let axes = model
            .default_space
            .as_ref()
            .and_then(|s| doc.spaces.iter().find(|x| &x.id == s))
            .map(|s| s.axes.clone())
            .unwrap_or_else(|| vec!["x".into(), "y".into(), "z".into()]);
        Printer { doc, model, axes, rep_names: HashMap::new() }
    }

    fn binding_name(&self, id: &str) -> String {
        self.model.binding(id).map(|b| b.name.clone()).unwrap_or_else(|| id.rsplit('.').next().unwrap_or(id).to_string())
    }

    fn event_name(&self, id: &str) -> String {
        self.model.events.iter().find(|e| e.id == id).map(|e| e.name.clone()).unwrap_or_else(|| id.rsplit('.').next().unwrap_or(id).to_string())
    }

    fn space_name(&self, id: &str) -> String {
        self.doc.spaces.iter().find(|s| s.id == id).map(|s| s.name.clone()).unwrap_or_else(|| id.to_string())
    }

    fn element_name(&self, id: &str) -> String {
        let m = self.model;
        m.equations
            .iter()
            .find(|x| x.id == id)
            .map(|x| x.name.clone())
            .or_else(|| m.constraints.iter().find(|x| x.id == id).map(|x| x.name.clone()))
            .or_else(|| m.events.iter().find(|x| x.id == id).map(|x| x.name.clone()))
            .or_else(|| m.binding(id).map(|x| x.name.clone()))
            .unwrap_or_else(|| id.rsplit('.').next().unwrap_or(id).to_string())
    }

    // ------------------------------------------------------------ types

    fn ty(&self, t: &Type) -> String {
        match t {
            Type::Boolean => "Boolean".into(),
            Type::Integer => "Integer".into(),
            Type::Quantity { dim: d } if d.is_none() => "Real".into(),
            Type::Quantity { dim: d } => named(d).map(str::to_string).unwrap_or_else(|| format!("Quantity<{}>", dim(d))),
            Type::Instant { dim: d } => format!("Instant<{}>", dim(d)),
            Type::Point { space } if Some(space) == self.model.default_space.as_ref() => "Point".into(),
            Type::Point { space } => format!("Point<{}>", self.space_name(space)),
            Type::Vector { space, dim: d } => {
                let dd = named(d).map(str::to_string).unwrap_or_else(|| dim(d));
                if Some(space) == self.model.default_space.as_ref() {
                    format!("Vector<{dd}>")
                } else {
                    format!("Vector<{}, {dd}>", self.space_name(space))
                }
            }
            Type::Tuple { items } => format!("({})", items.iter().map(|i| self.ty(i)).collect::<Vec<_>>().join(", ")),
            Type::Enum { cases } => format!("Enum<{}>", cases.join(", ")),
            Type::Function { result, .. } => self.ty(result),
        }
    }

    // ------------------------------------------------------------ expressions

    fn prec(e: &Expr) -> u8 {
        match e {
            Expr::If { .. } | Expr::Otherwise { .. } => P_IF,
            Expr::Bin { bin, l, r } => match bin {
                BinOp::Or => P_OR,
                BinOp::And => P_AND,
                BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => P_CMP,
                BinOp::Add | BinOp::Sub => P_ADD,
                BinOp::Mul if matches!((&**l, &**r), (Expr::Num { unit: None, num }, Expr::Const { .. }) if *num >= 0.0) => P_MUL,
                BinOp::Mul | BinOp::Div => P_MUL,
                BinOp::Pow => P_POW,
            },
            Expr::Not { .. } => P_NOT,
            Expr::Neg { .. } => P_NEG,
            Expr::Num { num, .. } if *num < 0.0 || num.is_sign_negative() => P_NEG,
            _ => P_ATOM,
        }
    }

    fn wrap(&self, e: &Expr, min: u8, params: &[String]) -> String {
        let s = self.expr_p(e, params);
        if Self::prec(e) < min {
            format!("({s})")
        } else {
            s
        }
    }

    pub fn expr(&self, e: &Expr) -> String {
        self.expr_p(e, &[])
    }

    fn expr_p(&self, e: &Expr, params: &[String]) -> String {
        let w = |x: &Expr, min: u8| self.wrap(x, min, params);
        let p = |x: &Expr| self.expr_p(x, params);
        match e {
            Expr::Num { num, unit } => {
                let n = number(*num);
                match unit {
                    Some(u) => format!("{n} {}", u.text),
                    None => n,
                }
            }
            Expr::Bool { bool } => bool.to_string(),
            Expr::Case { case } => case.clone(),
            Expr::Ref { r#ref } => self.binding_name(r#ref),
            Expr::Param { param } => params.get(*param).cloned().unwrap_or_else(|| format!("x{}", param + 1)),
            Expr::Builtin { builtin } => match builtin {
                Builtin::T => "t".into(),
                Builtin::T0 => "t0".into(),
                Builtin::Elapsed => "elapsed".into(),
            },
            Expr::Const { .. } => "π".into(),
            Expr::Origin { .. } => "origin".into(),
            Expr::Der { der } => format!("der({})", self.binding_name(der)),
            Expr::Neg { neg } => format!("-{}", w(neg, P_POW)),
            Expr::Not { not } => format!("not {}", w(not, P_NOT)),
            Expr::Norm { norm } => format!("|{}|", p(norm)),
            Expr::Bin { bin, l, r } => {
                if let (BinOp::Mul, Expr::Num { num, unit: None }, Expr::Const { .. }) = (bin, &**l, &**r) {
                    if *num >= 0.0 {
                        return format!("{}π", number(*num));
                    }
                }
                let (op, lmin, rmin) = match bin {
                    BinOp::Or => ("or", P_OR, P_AND),
                    BinOp::And => ("and", P_AND, P_NOT),
                    BinOp::Eq => ("==", P_ADD, P_ADD),
                    BinOp::Ne => ("!=", P_ADD, P_ADD),
                    BinOp::Lt => ("<", P_ADD, P_ADD),
                    BinOp::Le => ("<=", P_ADD, P_ADD),
                    BinOp::Gt => (">", P_ADD, P_ADD),
                    BinOp::Ge => (">=", P_ADD, P_ADD),
                    BinOp::Add => ("+", P_ADD, P_MUL),
                    BinOp::Sub => ("-", P_ADD, P_MUL),
                    BinOp::Mul => ("*", P_MUL, P_NEG),
                    BinOp::Div => ("/", P_MUL, P_NEG),
                    BinOp::Pow => {
                        // A number with a unit as a base would read as a unit power (`2 m^2`).
                        let base = match &**l {
                            Expr::Num { unit: Some(_), .. } => format!("({})", p(l)),
                            _ => w(l, P_ATOM),
                        };
                        return format!("{base}^{}", w(r, P_NEG));
                    }
                };
                // A negative literal after a binary operator is written in parentheses.
                let right = match &**r {
                    Expr::Num { num, .. } if num.is_sign_negative() => format!("({})", p(r)),
                    _ => w(r, rmin),
                };
                format!("{} {op} {right}", w(l, lmin))
            }
            Expr::Call { call, args } => {
                let name = match call {
                    Func::Sin => "sin",
                    Func::Cos => "cos",
                    Func::Tan => "tan",
                    Func::Sqrt => "sqrt",
                    Func::Exp => "exp",
                    Func::Log => "log",
                    Func::Abs => "abs",
                    Func::Min => "min",
                    Func::Max => "max",
                    Func::Atan2 => "atan2",
                };
                format!("{name}({})", args.iter().map(p).collect::<Vec<_>>().join(", "))
            }
            Expr::Apply { apply, args } => format!("{}({})", w(apply, P_ATOM), args.iter().map(p).collect::<Vec<_>>().join(", ")),
            Expr::If { r#if, then, r#else } => format!("if {} then {} else {}", p(r#if), p(then), p(r#else)),
            Expr::Tuple { tuple } => format!("({})", tuple.iter().map(p).collect::<Vec<_>>().join(", ")),
            Expr::Comp { comp, axis } => {
                let name = self.axes.get(*axis).cloned().unwrap_or_else(|| axis.to_string());
                format!("{}.{name}", w(comp, P_ATOM))
            }
            Expr::Lambda { lambda } => self.expr_p(&lambda.body, &lambda.names),
            Expr::Otherwise { otherwise, default } => format!("{} otherwise {}", w(otherwise, P_OR), w(default, P_OR)),
        }
    }

    // ------------------------------------------------------------ model

    fn model(&mut self) -> String {
        let m = self.model;
        let mut out = String::new();
        notes(&mut out, "", &m.notes);
        let space = m.default_space.as_ref().map(|s| format!(" in {}", self.space_name(s))).unwrap_or_default();
        let _ = writeln!(out, "model {}{space} {{", m.name);
        let mut sections: Vec<String> = vec![];
        for (role, word) in [
            (Role::Constant, "const"),
            (Role::Parameter, "param"),
            (Role::Input, "input"),
            (Role::Continuous, "state"),
            (Role::Discrete, "discrete"),
            (Role::Derived, "derived"),
        ] {
            let bs: Vec<&Binding> = m.bindings.iter().filter(|b| b.role == role).collect();
            if !bs.is_empty() {
                sections.push(self.block(word, &bs));
            }
        }
        let top: Vec<&Flow> = m.flows.iter().filter(|f| f.process.is_none()).collect();
        if !top.is_empty() {
            let mut s = format!("{INDENT}flow {{\n");
            for f in top {
                let _ = writeln!(s, "{INDENT}{INDENT}{}", self.flow(f));
            }
            let _ = write!(s, "{INDENT}}}");
            sections.push(s);
        }
        for pr in &m.processes {
            let flows: Vec<&Flow> = m.flows.iter().filter(|f| f.process.as_ref() == Some(&pr.id)).collect();
            let events: Vec<&Event> = m.events.iter().filter(|e| e.process.as_ref() == Some(&pr.id)).collect();
            let mut s = String::new();
            notes(&mut s, INDENT, &pr.notes);
            if events.is_empty() && flows.len() == 1 {
                let _ = write!(s, "{INDENT}process {} {{ flow {} }}", pr.name, self.flow(flows[0]));
            } else {
                let _ = writeln!(s, "{INDENT}process {} {{", pr.name);
                for f in flows {
                    let _ = writeln!(s, "{INDENT}{INDENT}flow {}", self.flow(f));
                }
                for e in events {
                    let _ = writeln!(s, "{}", self.event(e, 2));
                }
                let _ = write!(s, "{INDENT}}}");
            }
            sections.push(s);
        }
        let events: Vec<String> = m.events.iter().filter(|e| e.process.is_none()).map(|e| self.event(e, 1)).collect();
        if !events.is_empty() {
            sections.push(events.join("\n"));
        }
        let eqs: Vec<String> = m.equations.iter().map(|q| self.equation(q)).collect();
        if !eqs.is_empty() {
            sections.push(eqs.join("\n"));
        }
        let cons: Vec<String> = m.constraints.iter().filter(|c| !self.is_range(c)).map(|c| self.constraint(c)).collect();
        if !cons.is_empty() {
            sections.push(cons.join("\n"));
        }
        out.push_str(&sections.join("\n\n"));
        out.push_str("\n}");
        out
    }

    /// A parameter's range: a `reject` constraint attached to it and named `<name>_range`.
    fn is_range(&self, c: &Constraint) -> bool {
        match (&c.attached_to, c.policy, &c.tol) {
            (Some(b), Policy::Reject, None) => self.model.binding(b).is_some_and(|x| c.name == format!("{}_range", x.name) && x.role == Role::Parameter),
            _ => false,
        }
    }

    /// `in [lo, hi)` when the condition bounds `x` from both sides, else `where cond`.
    fn range_text(&self, b: &Binding, c: &Constraint) -> String {
        let is_x = |e: &Expr| matches!(e, Expr::Ref { r#ref } if *r#ref == b.id);
        if let Expr::Bin { bin: BinOp::And, l, r } = &c.cond {
            if let (Expr::Bin { bin: lo_op, l: lo, r: x1 }, Expr::Bin { bin: hi_op, l: x2, r: hi }) = (&**l, &**r) {
                let lo_ok = matches!(lo_op, BinOp::Le | BinOp::Lt) && is_x(x1);
                let hi_ok = matches!(hi_op, BinOp::Le | BinOp::Lt) && is_x(x2);
                if lo_ok && hi_ok {
                    let open = if *lo_op == BinOp::Le { "[" } else { "(" };
                    let close = if *hi_op == BinOp::Le { "]" } else { ")" };
                    return format!("in {open}{}, {}{close}", self.expr(lo), self.expr(hi));
                }
            }
        }
        format!("where {}", self.expr(&c.cond))
    }

    fn block(&self, word: &str, bs: &[&Binding]) -> String {
        // Columns: `name:` then the type, then `= value`, then range and modifiers.
        let heads: Vec<(String, String)> = bs.iter().map(|b| self.decl_head(b)).collect();
        let name_w = heads.iter().map(|h| h.0.chars().count()).max().unwrap_or(0);
        let type_w = heads.iter().map(|h| h.1.chars().count()).max().unwrap_or(0);
        let mut s = format!("{INDENT}{word} {{\n");
        for (b, (name, ty)) in bs.iter().zip(&heads) {
            notes(&mut s, &format!("{INDENT}{INDENT}"), &b.notes);
            let mut line = format!("{INDENT}{INDENT}{name:<nw$} {ty}", nw = name_w + 1, name = format!("{name}:"));
            let value = b.def.as_ref().or(b.init.as_ref());
            if let Some(v) = value {
                let pad = type_w.saturating_sub(ty.chars().count());
                let _ = write!(line, "{} = {}", " ".repeat(pad), self.expr(v));
            }
            let mut tail = vec![];
            if let Some(c) = self.model.constraints.iter().find(|c| c.attached_to.as_ref() == Some(&b.id) && self.is_range(c)) {
                tail.push(self.range_text(b, c));
            }
            if b.intervenable == Some(true) {
                tail.push("intervenable".into());
            }
            if b.private {
                tail.push("private".into());
            }
            if let Some(sym) = &b.display.symbol {
                tail.push(format!("symbol \"{sym}\""));
            }
            if let Some(u) = &b.display.unit {
                tail.push(format!("unit {u}"));
            }
            if !tail.is_empty() {
                line.push_str("   ");
                line.push_str(&tail.join("  "));
            }
            let _ = writeln!(s, "{}", line.trim_end());
        }
        let _ = write!(s, "{INDENT}}}");
        s
    }

    /// `name` or `f(x: Real)`, and the type.
    fn decl_head(&self, b: &Binding) -> (String, String) {
        // A dimensionless binding shown in an angle unit reads as an `Angle` (same IR).
        let angle = matches!(&b.ty, Type::Quantity { dim } if dim.is_none()) && matches!(b.display.unit.as_deref(), Some("deg" | "rad" | "rev"));
        if angle {
            return (b.name.clone(), "Angle".into());
        }
        match (&b.ty, &b.def) {
            (Type::Function { params, result }, Some(Expr::Lambda { lambda })) => {
                let names: Vec<String> = (0..params.len()).map(|i| lambda.names.get(i).cloned().unwrap_or_else(|| format!("x{}", i + 1))).collect();
                let ps: Vec<String> = names.iter().zip(params).map(|(n, t)| format!("{n}: {}", self.ty(t))).collect();
                (format!("{}({})", b.name, ps.join(", ")), self.ty(result))
            }
            _ => (b.name.clone(), self.ty(&b.ty)),
        }
    }

    fn flow(&self, f: &Flow) -> String {
        let op = if f.kind == FlowKind::Contribute { "+=" } else { "=" };
        format!("der({}) {op} {}", self.binding_name(&f.target), self.expr(&f.expr))
    }

    fn target(&self, t: &Target) -> String {
        let n = self.binding_name(&t.binding);
        match t.component {
            Some(i) => format!("{n}.{}", self.axes.get(i).cloned().unwrap_or_else(|| i.to_string())),
            None => n,
        }
    }

    fn op(&self, o: &Op) -> String {
        match o {
            Op::Set { target, value } => format!("set {} = {}", self.target(target), self.expr(value)),
            Op::Contribute { target, value } => format!("contribute {} += {}", self.target(target), self.expr(value)),
            Op::Emit { event, payload } => match payload {
                Some(p) => format!("emit {}({})", self.event_name(event), self.expr(p)),
                None => format!("emit {}", self.event_name(event)),
            },
        }
    }

    /// `{ op }` on one line for one operation, a block otherwise.
    fn ops(&self, ops: &[Op], depth: usize) -> String {
        match ops {
            [one] => format!("{{ {} }}", self.op(one)),
            _ => {
                let ind = INDENT.repeat(depth + 1);
                let mut s = "{\n".to_string();
                for o in ops {
                    let _ = writeln!(s, "{ind}{}", self.op(o));
                }
                let _ = write!(s, "{}}}", INDENT.repeat(depth));
                s
            }
        }
    }

    fn trigger(&self, t: &Trigger) -> String {
        match t {
            Trigger::Rising { guard } => format!("rising({})", self.expr(guard)),
            Trigger::Falling { guard } => format!("falling({})", self.expr(guard)),
            Trigger::Crossing { guard } => format!("crossing({})", self.expr(guard)),
            Trigger::At { time } => format!("at {}", self.expr(time)),
            Trigger::Every { period, from } => match from {
                Expr::Builtin { builtin: Builtin::T0 } => format!("every {}", self.expr(period)),
                f => format!("every {} from {}", self.expr(period), self.expr(f)),
            },
            Trigger::On { event } => self.event_name(event),
            Trigger::Start => "start".into(),
            Trigger::Input { binding } => format!("input({})", self.binding_name(binding)),
            Trigger::Request => "request".into(),
            // Not expressible in the working syntax (MK-E13); printed as a crossing of the
            // condition's value so that the text shows what the IR holds.
            Trigger::Level { cond } => format!("crossing({})", self.expr(cond)),
        }
    }

    fn event(&self, e: &Event, depth: usize) -> String {
        let ind = INDENT.repeat(depth);
        let mut s = String::new();
        notes(&mut s, &ind, &e.notes);
        let _ = write!(s, "{ind}event {} on {}", e.name, self.trigger(&e.trigger));
        if let Some(c) = &e.enable {
            let _ = write!(s, " if {}", self.expr(c));
        }
        if !e.handler.is_empty() {
            let _ = write!(s, " {}", self.ops(&e.handler, depth));
        }
        if let Some(z) = &e.zeno {
            match &z.policy {
                ZenoPolicy::Stop => s.push_str(" zeno stop"),
                ZenoPolicy::Settle { ops } => {
                    let _ = write!(s, " zeno settle {}", self.ops(ops, depth));
                }
            }
        }
        s
    }

    fn equation(&self, q: &Equation) -> String {
        let mut s = format!("{INDENT}equation {}: {} == {}", q.name, self.expr(&q.lhs), self.expr(&q.rhs));
        if let Some(t) = &q.tol {
            let _ = write!(s, " checked within {}", self.expr(t));
        }
        s
    }

    fn constraint(&self, c: &Constraint) -> String {
        let mut s = format!("{INDENT}constraint {}: {}", c.name, self.expr(&c.cond));
        if let Some(t) = &c.tol {
            let _ = write!(s, " within {}", self.expr(t));
        }
        match c.policy {
            Policy::Report => {}
            Policy::Reject => s.push_str(" policy reject"),
            Policy::Stop => s.push_str(" policy stop"),
        }
        s
    }

    // ------------------------------------------------------------ presentation

    fn arg(&self, a: &Arg) -> String {
        match a {
            Arg::Expr { expr } => self.expr(expr),
            Arg::Text { text } => format!("\"{text}\""),
            Arg::Range { lo, hi } => format!("[{}, {}]", self.expr(lo), self.expr(hi)),
            Arg::Scale { scale } => format!("{} -> {} px", self.expr(&scale.quantity), number(scale.px)),
            Arg::Word { word } => word.clone(),
            Arg::Sampled { expr, every } => format!("{} every {}", self.expr(expr), self.expr(every)),
            Arg::Element { element } => self.element_name(element),
        }
    }

    fn rep(&self, r: &Rep, depth: usize) -> String {
        let mut args: Vec<String> = r.sources.iter().map(|a| self.arg(a)).collect();
        args.extend(r.props.iter().map(|p| format!("{}: {}", p.name, self.arg(&p.value))));
        let mut s = if args.is_empty() { r.kind.clone() } else { format!("{}({})", r.kind, args.join(", ")) };
        if let Some(n) = &r.name {
            let _ = write!(s, " as {n}");
        }
        if let Some(inv) = &r.inverse {
            let part = inv.part.as_ref().map(|p| format!(" {p}")).unwrap_or_default();
            let bind = if inv.part.as_deref() == Some("head") { "h" } else { "p" };
            let props: Vec<String> =
                inv.proposals.iter().map(|pr| format!("propose {} = {}", self.binding_name(&pr.target), self.expr_p(&pr.value, &[bind.to_string()]))).collect();
            if props.len() == 1 {
                let _ = write!(s, " {{ on {}{part} as {bind} {{ {} }} }}", inv.gesture, props[0]);
            } else {
                let ind = INDENT.repeat(depth + 1);
                let _ = write!(s, " {{\n{ind}on {}{part} as {bind} {{\n", inv.gesture);
                for p in props {
                    let _ = writeln!(s, "{ind}{INDENT}{p}");
                }
                let _ = write!(s, "{ind}}}\n{}}}", INDENT.repeat(depth));
            }
        }
        if !r.members.is_empty() {
            let _ = write!(s, " {}", self.reps_block(&r.members, depth));
        }
        s
    }

    fn reps_block(&self, reps: &[Rep], depth: usize) -> String {
        let ind = INDENT.repeat(depth + 1);
        let mut s = "{\n".to_string();
        for r in reps {
            let _ = writeln!(s, "{ind}{}", self.rep(r, depth + 1));
        }
        let _ = write!(s, "{}}}", INDENT.repeat(depth));
        s
    }

    fn presentation(&mut self, p: &Presentation) -> String {
        for v in &p.views {
            for r in &v.representations {
                self.rep_name(r);
            }
        }
        for a in p.timeline.iter().flat_map(|t| t.scenes.iter().flat_map(|s| s.beats.iter().flat_map(|b| b.actions.iter()))) {
            self.collect_rep_names(a);
        }
        let mut out = String::new();
        notes(&mut out, "", &p.notes);
        let _ = writeln!(out, "presentation {} for {} {{", p.name, self.model.name);
        let mut sections = vec![];
        for v in &p.views {
            let head = match &v.kind {
                ViewKind::Spatial { space, scale, y_up } => format!(
                    "view {}: spatial({}, scale: {} -> {} px, y: {})",
                    v.name,
                    self.space_name(space),
                    self.expr(&scale.quantity),
                    number(scale.px),
                    if *y_up { "up" } else { "down" }
                ),
                ViewKind::Plot { x, y } => format!(
                    "view {}: plot(x: [{}, {}], y: [{}, {}])",
                    v.name,
                    self.expr(&x[0]),
                    self.expr(&x[1]),
                    self.expr(&y[0]),
                    self.expr(&y[1])
                ),
                ViewKind::Panel => format!("panel {}", v.name),
            };
            sections.push(format!("{INDENT}{head} {}", self.reps_block(&v.representations, 1)));
        }
        for perm in &p.permissions {
            sections.push(format!("{INDENT}permit {} {{ {} }}", perm.role, perm.allows.join("; ")));
        }
        if !p.observations.is_empty() {
            let w = p.observations.iter().map(|o| o.name.chars().count()).max().unwrap_or(0);
            let mut s = format!("{INDENT}observe {{\n");
            for o in &p.observations {
                let _ = writeln!(s, "{INDENT}{INDENT}{:<w$} = {}", o.name, self.observation(o).trim_end());
            }
            let _ = write!(s, "{INDENT}}}");
            sections.push(s);
        }
        if let Some(t) = &p.timeline {
            let mut s = format!("{INDENT}timeline {{\n");
            for sc in &t.scenes {
                let _ = writeln!(s, "{INDENT}{INDENT}scene {} {{", sc.name);
                for b in &sc.beats {
                    let _ = writeln!(s, "{}", self.beat(p, b));
                }
                let _ = writeln!(s, "{INDENT}{INDENT}}}");
            }
            let _ = write!(s, "{INDENT}}}");
            sections.push(s);
        }
        out.push_str(&sections.join("\n"));
        out.push_str("\n}");
        out
    }

    /// Records the names of a representation and of a group's members (D-043).
    fn rep_name(&mut self, r: &Rep) {
        if let Some(n) = &r.name {
            self.rep_names.insert(r.id.clone(), n.clone());
        }
        for m in &r.members {
            self.rep_name(m);
        }
    }

    fn collect_rep_names(&mut self, a: &Action) {
        match a {
            Action::Show { reps, .. } | Action::Reveal { reps, .. } => {
                for r in reps {
                    self.rep_name(r);
                }
            }
            Action::Explore { controls, fallback, .. } => {
                for r in controls {
                    if let Some(n) = &r.name {
                        self.rep_names.insert(r.id.clone(), n.clone());
                    }
                }
                for f in fallback {
                    self.collect_rep_names(f);
                }
            }
            Action::Sequence { actions } => actions.iter().for_each(|x| self.collect_rep_names(x)),
            _ => {}
        }
    }

    fn observation(&self, o: &Observation) -> String {
        let source = match &o.source {
            Source::Expr { expr } => self.expr(expr),
            Source::EventLog { event, zeno_only } => {
                let mut s = "event_log".to_string();
                if let Some(e) = event {
                    let _ = write!(s, " of {}", self.event_name(e));
                }
                if *zeno_only {
                    s.push_str(" where zeno_applied");
                }
                s
            }
            Source::Diagnostics { element } => match element {
                Some(e) => format!("diagnostics of {}", self.element_name(e)),
                None => "diagnostics".into(),
            },
            Source::InterventionLog => "intervention_log".into(),
        };
        let schedule = match &o.schedule {
            Schedule::Live => " live".to_string(),
            Schedule::Every { period } => format!(" every {}", self.expr(period)),
            Schedule::At { time } => format!(" at {}", self.expr(time)),
            Schedule::On { event, microstep } => match microstep {
                Some(n) => format!(" on {} microstep {n}", self.event_name(event)),
                None => format!(" on {}", self.event_name(event)),
            },
            Schedule::Over { from, to, lo_closed, hi_closed } => format!(
                " over {}{}, {}{}",
                if *lo_closed { "[" } else { "(" },
                self.expr(from),
                to.as_ref().map(|t| self.expr(t)).unwrap_or_else(|| "t_end".into()),
                if *hi_closed { "]" } else { ")" }
            ),
            Schedule::Run => String::new(),
        };
        format!("{source}{schedule}")
    }

    fn beat(&self, p: &Presentation, b: &Beat) -> String {
        let depth = 3;
        let ind = INDENT.repeat(depth);
        let acts = self.actions(p, &b.actions, depth + 1);
        let one_line = acts.len() <= 3 && acts.iter().all(|a| !a.contains('\n')) && acts.iter().map(|a| a.len()).sum::<usize>() < 70;
        if one_line {
            format!("{ind}beat {} {{ {} }}", b.name, acts.join("; "))
        } else {
            let mut s = format!("{ind}beat {} {{\n", b.name);
            for a in acts {
                let _ = writeln!(s, "{}{a}", INDENT.repeat(depth + 1));
            }
            let _ = write!(s, "{ind}}}");
            s
        }
    }

    /// Actions, with `run rate r` and a following `wait until E` joined (section 1.4).
    fn actions(&self, p: &Presentation, acts: &[Action], depth: usize) -> Vec<String> {
        let mut out = vec![];
        let mut i = 0;
        while i < acts.len() {
            if let (Action::Run { rate }, Some(Action::WaitUntil { event })) = (&acts[i], acts.get(i + 1)) {
                out.push(format!("run rate {} until {}", self.expr(rate), self.event_name(event)));
                i += 2;
                continue;
            }
            out.push(self.action(p, &acts[i], depth));
            i += 1;
        }
        out
    }

    fn action(&self, p: &Presentation, a: &Action, depth: usize) -> String {
        let block = |items: Vec<String>| -> String {
            if items.len() <= 2 && items.iter().all(|x| !x.contains('\n')) && items.iter().map(|x| x.len()).sum::<usize>() < 60 {
                format!("{{ {} }}", items.join("; "))
            } else {
                let mut s = "{\n".to_string();
                for x in items {
                    let _ = writeln!(s, "{}{x}", INDENT.repeat(depth + 1));
                }
                let _ = write!(s, "{}}}", INDENT.repeat(depth));
                s
            }
        };
        match a {
            Action::Show { view: Some(v), reps } => {
                let name = p.views.iter().find(|x| &x.id == v).map(|x| x.name.clone()).unwrap_or_else(|| v.clone());
                format!("in {name} {}", block(reps.iter().map(|r| self.rep(r, depth + 1)).collect()))
            }
            Action::Show { view: None, reps } => reps.iter().map(|r| format!("show {}", self.rep(r, depth))).collect::<Vec<_>>().join("; "),
            Action::Highlight { target } => format!("highlight {}", self.rep_names.get(target).cloned().unwrap_or_else(|| target.clone())),
            Action::Hide { target, duration } => {
                let name = self.rep_names.get(target).cloned().unwrap_or_else(|| target.clone());
                match duration {
                    Some(d) => format!("hide {name} for {}", self.expr(d)),
                    None => format!("hide {name}"),
                }
            }
            Action::Reveal { view, style, duration, reps } => {
                let mut s = format!("reveal {}", if *style == RevealStyle::Fade { "fade" } else { "draw" });
                if let Some(d) = duration {
                    let _ = write!(s, " for {}", self.expr(d));
                }
                if let Some(v) = view {
                    let name = p.views.iter().find(|x| &x.id == v).map(|x| x.name.clone()).unwrap_or_else(|| v.clone());
                    let _ = write!(s, " in {name}");
                }
                let _ = write!(s, " {}", block(reps.iter().map(|r| self.rep(r, depth + 1)).collect()));
                s
            }
            Action::Camera { view, center, zoom, duration } => {
                let name = p.views.iter().find(|x| &x.id == view).map(|x| x.name.clone()).unwrap_or_else(|| view.clone());
                let mut s = format!("camera {name}");
                if let Some(c) = center {
                    let _ = write!(s, " to {}", self.expr(c));
                }
                if let Some(z) = zoom {
                    let _ = write!(s, " zoom {}", self.expr(z));
                }
                if let Some(d) = duration {
                    let _ = write!(s, " for {}", self.expr(d));
                }
                s
            }
            Action::Narrate { text, duration } => match duration {
                Some(d) => format!("narrate \"{text}\" for {}", self.expr(d)),
                None => format!("narrate \"{text}\""),
            },
            Action::Run { rate } => format!("run rate {}", self.expr(rate)),
            Action::Hold => "hold".into(),
            Action::Seek { time } => format!("seek {}", self.expr(time)),
            Action::Reset => "reset".into(),
            Action::Branch => "branch".into(),
            Action::Intervene { ops } => format!("intervene {}", block(ops.iter().map(|o| self.op(o)).collect())),
            Action::Request { event } => format!("request {}", self.event_name(event)),
            Action::Wait { duration } => format!("wait {}", self.expr(duration)),
            Action::WaitUntil { event } => format!("wait until {}", self.event_name(event)),
            Action::Explore { limit, keep, controls, fallback } => {
                let mut s = "explore".to_string();
                if let Some(l) = limit {
                    let _ = write!(s, " limit {}", self.expr(l));
                }
                if !keep.is_empty() {
                    let _ = write!(s, " keep {}", keep.iter().map(|k| self.binding_name(k)).collect::<Vec<_>>().join(", "));
                }
                let _ = write!(s, " {}", self.reps_block(controls, depth));
                if !fallback.is_empty() {
                    let _ = write!(s, " fallback {}", block(self.actions(p, fallback, depth + 1)));
                }
                s
            }
            Action::Sequence { actions } => format!("sequence {}", block(self.actions(p, actions, depth + 1))),
        }
    }

    // ------------------------------------------------------------ runs

    fn run(&self, r: &RunCase) -> String {
        let pres = r.presentation.as_ref().and_then(|p| self.doc.presentation(p));
        let mut s = format!("run {} of {}", r.name, self.model.name);
        if let Some(p) = pres {
            let _ = write!(s, " with {}", p.name);
        }
        s.push_str(" {\n");
        let ind2 = INDENT.repeat(2);
        if !r.params.is_empty() {
            let items: Vec<String> = r.params.iter().map(|o| format!("{} = {}", self.binding_name(&o.binding), self.expr(&o.value))).collect();
            let _ = writeln!(s, "{INDENT}param {{ {} }}", items.join("; "));
        }
        let mut cfg = vec![];
        if let Some(sv) = &r.config.solver {
            cfg.push(format!("solver = {sv}"));
        }
        for (k, v) in [("h", &r.config.h), ("rtol", &r.config.rtol), ("atol", &r.config.atol)] {
            if let Some(v) = v {
                cfg.push(format!("{k} = {}", self.expr(v)));
            }
        }
        if !cfg.is_empty() {
            let _ = writeln!(s, "{INDENT}config {{ {} }}", cfg.join("; "));
        }
        if let Some(e) = &r.end {
            let _ = writeln!(s, "{INDENT}until {}", self.expr(e));
        }
        if !r.learner.is_empty() {
            let _ = writeln!(s, "{INDENT}learner {{");
            for st in &r.learner {
                let input = match &st.input {
                    LearnerInput::Continue => "continue".to_string(),
                    LearnerInput::SetControl { control, binding, value } => format!("set {control} {} = {}", self.binding_name(binding), self.expr(value)),
                };
                let _ = writeln!(s, "{ind2}at {}: {input}", self.expr(&st.at));
            }
            let _ = writeln!(s, "{INDENT}}}");
        }
        if !r.expectations.is_empty() {
            let _ = writeln!(s, "{INDENT}expect {{");
            for e in &r.expectations {
                let _ = writeln!(s, "{ind2}{}", self.expectation(pres, &e.check));
            }
            let _ = writeln!(s, "{INDENT}}}");
        }
        s.push('}');
        s
    }

    fn subject(&self, pres: Option<&Presentation>, sub: &Subject) -> String {
        let beat_name = |id: &str| {
            pres.and_then(|p| p.timeline.as_ref())
                .and_then(|t| t.scenes.iter().flat_map(|s| &s.beats).find(|b| b.id == id))
                .map(|b| b.name.clone())
                .unwrap_or_else(|| id.to_string())
        };
        match sub {
            Subject::Observation { observation, index } => {
                let name = pres
                    .and_then(|p| p.observations.iter().find(|o| &o.id == observation))
                    .map(|o| o.name.clone())
                    .unwrap_or_else(|| observation.rsplit('.').next().unwrap_or(observation).to_string());
                match index {
                    Some(k) => format!("{name}[{k}]"),
                    None => name,
                }
            }
            Subject::On { expr, event, microstep } => match microstep {
                Some(n) => format!("({} on {} microstep {n})", self.expr(expr), self.event_name(event)),
                None => format!("({} on {})", self.expr(expr), self.event_name(event)),
            },
            Subject::BeatStart { beat } => format!("start of {}", beat_name(beat)),
            Subject::BeatEnd { beat } => format!("end of {}", beat_name(beat)),
        }
    }

    fn expectation(&self, pres: Option<&Presentation>, c: &Check) -> String {
        match c {
            Check::Equal { subject, expected, tolerance } => {
                let rhs = match expected {
                    Operand::Value { expr } => self.expr(expr),
                    Operand::Subject { subject } => self.subject(pres, subject),
                    Operand::List { items } => format!(
                        "[{}]",
                        items
                            .iter()
                            .map(|i| match i {
                                Item::Event { event } => self.event_name(event),
                                Item::Value { expr } => self.expr(expr),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                };
                let tol = match tolerance {
                    Tolerance::Exact => "exactly".to_string(),
                    Tolerance::Abs { tol } => format!("within {}", self.expr(tol)),
                    Tolerance::Rel { tol } => format!("within rel {}", self.expr(tol)),
                };
                format!("{} == {rhs} {tol}", self.subject(pres, subject))
            }
            Check::Within { subject, lo, hi, lo_closed, hi_closed } => format!(
                "{} in {}{}, {}{}",
                self.subject(pres, subject),
                if *lo_closed { "[" } else { "(" },
                self.expr(lo),
                self.expr(hi),
                if *hi_closed { "]" } else { ")" }
            ),
            Check::Outcome { outcome } => match outcome {
                Outcome::InitializationFails => "initialization fails".into(),
                Outcome::ConfigurationRejected => "configuration rejected".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_round_trip() {
        for x in [0.0, 1.0, 9.81, 1e-9, 2.5e3, 0.451523640985731, 1e21, 123456789.125, 1e-300] {
            let s = number(x);
            assert_eq!(s.parse::<f64>().unwrap(), x, "{s}");
        }
        assert_eq!(number(1e-9), "1e-9");
        assert_eq!(number(9.81), "9.81");
        assert_eq!(number(2.0), "2");
    }
}
