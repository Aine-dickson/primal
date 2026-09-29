//! Runs (RC sections 3 to 11): initialization, continuous evolution, event detection and
//! location, event iteration, Zeno detection, failures, interventions and requests.

use crate::solver::{Attempt, Dopri5, Rhs, Rk4, Step};
use prismal_ir::{Expr, Id, Op, Policy, Type};
use prismal_kernel::{check_intervention, compile_expr, distance, CExpr, CModel, COp, CTrigger, Ctx, Dir, Status, Value};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum SolverConfig {
    Dopri5 { rtol: f64, atol: f64, h_max: f64 },
    /// `h` MUST be set for a fixed-step solver (RC section 16); `None` is rejected.
    Rk4 { h: Option<f64> },
}

/// A logged action from outside the model (RC section 11).
#[derive(Clone, Debug)]
pub enum Action {
    /// `set` operations on intervenable bindings (MK-17.2).
    Intervene(Vec<Op>),
    /// A request for an `on request` event (MK-17.2a, D-027).
    Request(Id),
}

#[derive(Clone, Debug)]
pub struct Scheduled {
    pub t: f64,
    pub action: Action,
}

/// Run configuration (RC-3.1, RC section 16).
#[derive(Clone, Debug)]
pub struct Config {
    pub t0: f64,
    pub t_end: f64,
    pub solver: SolverConfig,
    pub t_ref: f64,
    pub eps_t: Option<f64>,
    pub n_micro: u32,
    pub eps_zeno: Option<f64>,
    pub n_zeno: u32,
    pub w_zeno: Option<f64>,
    /// Parameter overrides by binding identity (MK-6.8).
    pub params: Vec<(Id, Expr)>,
    /// The input and intervention log (RC-3.1).
    pub log: Vec<Scheduled>,
}

impl Config {
    /// Defaults of RC section 16, running to `t_end` seconds after `t0 = 0`.
    pub fn until(t_end: f64) -> Config {
        Config {
            t0: 0.0,
            t_end,
            solver: SolverConfig::Dopri5 { rtol: 1e-6, atol: 1e-9, h_max: f64::INFINITY },
            t_ref: 1.0,
            eps_t: None,
            n_micro: 100,
            eps_zeno: None,
            n_zeno: 1000,
            w_zeno: None,
            params: vec![],
            log: vec![],
        }
    }
    pub fn param(mut self, id: &str, value: Expr) -> Config {
        self.params.push((id.into(), value));
        self
    }
    pub fn solver(mut self, s: SolverConfig) -> Config {
        self.solver = s;
        self
    }
    pub fn at(mut self, t: f64, action: Action) -> Config {
        self.log.push(Scheduled { t, action });
        self
    }
    fn eps_t(&self) -> f64 {
        self.eps_t.unwrap_or(1e-10 * self.t_ref)
    }
}

/// Failure categories (RC-10.1) and reported diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Configuration,
    Initialization,
    Model,
    Constraint,
    Equation,
    Conflict,
    Cascade,
    Zeno,
    Computational,
    Intervention,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RunDiag {
    pub category: Category,
    pub message: String,
    pub element: Option<Id>,
    pub t: f64,
    pub n: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunStatus {
    Completed,
    /// Stopped by a failure policy (RC-3.3), with the diagnostic that caused it.
    Stopped(RunDiag),
    /// Initialization or configuration failed; the run did not start (RC-5.2).
    NotStarted(RunDiag),
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub event: usize,
    pub name: String,
    pub t: f64,
    pub n: u32,
    /// True when a Zeno policy replaced the handler (RC-15.1).
    pub zeno: bool,
    pub requested: bool,
}

/// A committed superdense state (RC-4.2).
#[derive(Clone, Debug)]
pub struct Committed {
    pub t: f64,
    pub n: u32,
    pub vals: Vec<Value>,
}

/// An accepted step and the committed state whose discrete values hold during it.
#[derive(Clone, Debug)]
pub struct Segment {
    pub step: Step,
    pub base: usize,
}

/// The result of a run: trajectory, event log, diagnostics and status (RC section 15).
#[derive(Clone, Debug)]
pub struct Run {
    pub model: CModel,
    pub config: Config,
    pub committed: Vec<Committed>,
    pub segments: Vec<Segment>,
    pub log: Vec<LogEntry>,
    pub diagnostics: Vec<RunDiag>,
    /// Interventions rejected by validation (RC-10.5); not run failures.
    pub rejected: Vec<RunDiag>,
    pub status: RunStatus,
}

struct ZenoMonitor {
    last: Option<f64>,
    recent: VecDeque<f64>,
}

struct Engine<'a> {
    cm: &'a CModel,
    cfg: &'a Config,
    vals: Vec<Value>,
    t: f64,
    n: u32,
    refs: Vec<Option<bool>>,
    zeno: Vec<ZenoMonitor>,
    run: Run,
}

/// The right-hand side over one segment: discrete state and parameters fixed.
struct SegmentRhs<'a> {
    cm: &'a CModel,
    scratch: Vec<Value>,
    t0: f64,
}

impl Rhs for SegmentRhs<'_> {
    fn f(&mut self, t: f64, y: &[f64], dy: &mut [f64]) -> Result<(), Status> {
        self.cm.load_y(&mut self.scratch, y);
        self.cm.update_derived(&mut self.scratch, t, self.t0)?;
        self.cm.rhs(&self.scratch, t, self.t0, dy)
    }
}

fn sign(x: f64) -> Option<bool> {
    if x > 0.0 {
        Some(true)
    } else if x < 0.0 {
        Some(false)
    } else {
        None
    }
}

fn matches_dir(dir: Dir, from: bool, to: bool) -> bool {
    match dir {
        Dir::Rising => !from && to,
        Dir::Falling => from && !to,
        Dir::Either => from != to,
    }
}

/// Runs a checked model headless with failure policy `stop` (RC-10.4).
pub fn run(cm: &CModel, cfg: Config) -> Run {
    let mut run = Run {
        model: cm.clone(),
        config: cfg.clone(),
        committed: vec![],
        segments: vec![],
        log: vec![],
        diagnostics: vec![],
        rejected: vec![],
        status: RunStatus::Completed,
    };
    let fail = |category, message: String| RunDiag { category, message, element: None, t: cfg.t0, n: 0 };
    if let SolverConfig::Rk4 { h: None } = cfg.solver {
        run.status = RunStatus::NotStarted(fail(Category::Configuration, "the fixed-step solver rk4 requires a step h (RC section 16)".into()));
        return run;
    }
    // Parameter overrides are evaluated as constants of the binding's type.
    let mut overrides = vec![];
    for (id, e) in &cfg.params {
        let Some(&i) = cm.index.get(id) else {
            run.status = RunStatus::NotStarted(fail(Category::Configuration, format!("unknown parameter `{id}`")));
            return run;
        };
        let v = compile_expr(cm, e, Some(&cm.bindings[i].ty))
            .map_err(|d| format!("{d:?}"))
            .and_then(|(c, _)| c.eval(&Ctx { vals: &[], der: None, t: cfg.t0, t0: cfg.t0, args: &[] }).map_err(|s| s.cause));
        match v {
            Ok(v) => overrides.push((i, v)),
            Err(m) => {
                run.status = RunStatus::NotStarted(fail(Category::Configuration, format!("override of `{id}`: {m}")));
                return run;
            }
        }
    }
    let vals = match cm.initial(&overrides, cfg.t0) {
        Ok(v) => v,
        Err(s) => {
            run.status = RunStatus::NotStarted(fail(Category::Initialization, s.cause));
            return run;
        }
    };
    let mut e = Engine {
        cm,
        cfg: &cfg,
        vals,
        t: cfg.t0,
        n: 0,
        refs: vec![None; cm.events.len()],
        zeno: (0..cm.events.len()).map(|_| ZenoMonitor { last: None, recent: VecDeque::new() }).collect(),
        run,
    };
    // RC-5.2: constraints on the initial state.
    match e.check_constraints(&e.vals.clone(), true) {
        Ok(reports) => e.run.diagnostics.extend(reports),
        Err(d) => {
            e.run.status = RunStatus::NotStarted(RunDiag { category: Category::Initialization, ..d });
            return e.run;
        }
    }
    e.commit();
    let start: Vec<usize> = cm.events.iter().enumerate().filter(|(_, ev)| matches!(ev.trigger, Some(CTrigger::Start))).map(|(i, _)| i).collect();
    if let Err(d) = e.iterate(start, vec![]) {
        e.run.status = RunStatus::Stopped(d);
        return e.run;
    }
    e.retake_refs();
    if let Err(d) = e.advance() {
        e.run.status = RunStatus::Stopped(d);
    }
    e.run
}

impl<'a> Engine<'a> {
    fn diag(&self, category: Category, message: String, element: Option<&str>) -> RunDiag {
        RunDiag { category, message, element: element.map(|s| s.to_string()), t: self.t, n: self.n }
    }

    fn ctx<'v>(&self, vals: &'v [Value]) -> Ctx<'v> {
        Ctx { vals, der: None, t: self.t, t0: self.cfg.t0, args: &[] }
    }

    fn commit(&mut self) {
        self.run.committed.push(Committed { t: self.t, n: self.n, vals: self.vals.clone() });
    }

    fn guard_value(&self, vals: &[Value], ei: usize) -> Result<Option<f64>, Status> {
        match &self.cm.events[ei].trigger {
            Some(CTrigger::Crossing { guard, .. }) => Ok(Some(guard.eval(&self.ctx(vals))?.num())),
            _ => Ok(None),
        }
    }

    /// RC-5.4, RC-8.4: sign references from the current committed state.
    fn retake_refs(&mut self) {
        for ei in 0..self.cm.events.len() {
            if let Ok(Some(g)) = self.guard_value(&self.vals.clone(), ei) {
                if let Some(s) = sign(g) {
                    self.refs[ei] = Some(s);
                }
            }
        }
    }

    /// Checks constraints (MK-12.3). Returns reports, or the failure of a `reject` or `stop`
    /// constraint.
    fn check_constraints(&self, vals: &[Value], _initial: bool) -> Result<Vec<RunDiag>, RunDiag> {
        let mut reports = vec![];
        for k in &self.cm.constraints {
            let c = self.ctx(vals);
            let ok = if let Some((l, r, tol)) = &k.within {
                match (l.eval(&c), r.eval(&c)) {
                    (Ok(a), Ok(b)) => distance(&a, &b).map(|d| d <= *tol).unwrap_or(false),
                    _ => false,
                }
            } else {
                matches!(k.cond.as_ref().unwrap().eval(&c), Ok(Value::Bool(true)))
            };
            if ok {
                continue;
            }
            let values: Vec<String> = self
                .cm
                .ir
                .constraints
                .iter()
                .find(|x| x.id == k.id)
                .map(|x| x.cond.refs())
                .unwrap_or_default()
                .iter()
                .map(|id| format!("{} = {}", self.cm.bindings[self.cm.idx(id)].name, vals[self.cm.idx(id)]))
                .collect();
            let d = self.diag(Category::Constraint, format!("constraint `{}` violated ({})", k.name, values.join(", ")), Some(&k.id));
            match k.policy {
                Policy::Report => reports.push(d),
                Policy::Reject | Policy::Stop => return Err(d),
            }
        }
        Ok(reports)
    }

    /// Equation checks (MK-11.2): residuals above the tolerance are reported.
    fn check_equations(&mut self) {
        let der = self.cm.der_values(&self.vals, self.t, self.cfg.t0).ok();
        for q in &self.cm.equations {
            let Some(tol) = q.check_tol else { continue };
            let c = Ctx { vals: &self.vals, der: der.as_deref(), t: self.t, t0: self.cfg.t0, args: &[] };
            let res = match (q.lhs.eval(&c), q.rhs.eval(&c)) {
                (Ok(a), Ok(b)) => distance(&a, &b),
                _ => None,
            };
            if res.map(|r| r > tol).unwrap_or(true) {
                let d = self.diag(Category::Equation, format!("check equation `{}` residual {:?} exceeds {}", q.name, res, tol), Some(&q.id));
                self.run.diagnostics.push(d);
            }
        }
    }

    fn after_step_checks(&mut self) -> Result<(), RunDiag> {
        let reports = self.check_constraints(&self.vals.clone(), false)?;
        self.run.diagnostics.extend(reports);
        self.check_equations();
        Ok(())
    }

    /// Continuous evolution with event detection until `t_end` (RC sections 6, 7).
    fn advance(&mut self) -> Result<(), RunDiag> {
        let cm = self.cm;
        if cm.is_static {
            // RC-12.4: a static model has a single instant; only interventions apply.
            let actions: Vec<Scheduled> = self.cfg.log.clone();
            for s in actions {
                self.intervene(&s.action)?;
            }
            return Ok(());
        }
        let eps_t = self.cfg.eps_t();
        let n_y = cm.n_y();
        let mut y = vec![0.0; n_y];
        let mut h: Option<f64> = None;
        let mut pending: Vec<Scheduled> = self.cfg.log.clone();
        pending.sort_by(|a, b| a.t.total_cmp(&b.t));
        let mut pending: VecDeque<Scheduled> = pending.into();
        loop {
            // Logged actions due now apply after the model's own iteration (RC-8.7, RC-11.1).
            let mut acted = false;
            while pending.front().map(|s| s.t <= self.t).unwrap_or(false) {
                if self.run.committed.last().map(|c| c.t != self.t).unwrap_or(true) {
                    self.n = 0;
                    self.commit();
                }
                let s = pending.pop_front().unwrap();
                self.intervene(&s.action)?;
                acted = true;
            }
            if acted {
                self.retake_refs();
                h = None;
            }
            if self.t >= self.cfg.t_end {
                break;
            }
            cm.store_y(&self.vals, &mut y);
            let next_stop = pending.front().map(|s| s.t.min(self.cfg.t_end)).unwrap_or(self.cfg.t_end);
            let base = self.run.committed.len() - 1;
            let mut rhs = SegmentRhs { cm, scratch: self.vals.clone(), t0: self.cfg.t0 };
            let span = next_stop - self.t;
            // One accepted step (RC-6.7: never past the next stop).
            let (y1, step, h_next) = match &self.cfg.solver {
                SolverConfig::Rk4 { h: Some(hf) } => {
                    let hs = if self.t + hf >= next_stop - 1e-12 * hf { span } else { *hf };
                    let (y1, step) = Rk4::step(&mut rhs, self.t, &y, hs).map_err(|s| self.diag(Category::Model, s.cause, None))?;
                    (y1, step, None)
                }
                SolverConfig::Rk4 { h: None } => unreachable!(),
                SolverConfig::Dopri5 { rtol, atol, h_max } => {
                    let solver = Dopri5 { rtol: *rtol, atol: *atol, h_max: *h_max };
                    let mut hh = match h {
                        Some(x) => x,
                        None => solver.initial_step(&mut rhs, self.t, &y, span).map_err(|s| self.diag(Category::Model, s.cause, None))?,
                    };
                    loop {
                        let land = self.t + hh >= next_stop - 1e-12 * hh.max(1e-300);
                        let hs = if land { span } else { hh };
                        let h_min = 1e-14 * self.t.abs().max(1.0);
                        if hs < h_min {
                            return Err(self.diag(Category::Computational, format!("step size {hs} below the minimum at t = {}", self.t), None));
                        }
                        match solver.attempt(&mut rhs, self.t, &y, hs) {
                            Ok(Attempt::Accepted { y1, step, h_next }) => break (y1, step, Some(h_next)),
                            Ok(Attempt::Rejected { h_next }) => hh = h_next,
                            // RC-6.8: a status in a flow rejects the step; retry smaller.
                            Err(s) => {
                                hh = hs * 0.25;
                                if hh < h_min {
                                    return Err(self.diag(Category::Model, s.cause, None));
                                }
                            }
                        }
                    }
                }
            };
            h = h_next;
            let t1 = if step.t1 >= next_stop - 1e-12 * (step.t1 - step.t0) { next_stop } else { step.t1 };
            let mut step = step;
            step.t1 = t1;

            // Event detection on the dense output (RC-7.1 to RC-7.7).
            let mut found: Vec<(usize, f64)> = vec![];
            let mut new_refs = self.refs.clone();
            for ei in 0..cm.events.len() {
                let Some(CTrigger::Crossing { dir, .. }) = &cm.events[ei].trigger else { continue };
                let mut r = self.refs[ei];
                let mut prev = self.t;
                for k in 1..=4 {
                    let tk = if k == 4 { t1 } else { self.t + (t1 - self.t) * k as f64 / 4.0 };
                    let g = self.guard_at(&step, &rhs.scratch, tk, ei).map_err(|s| self.diag(Category::Model, s.cause, None))?;
                    if let Some(s) = sign(g) {
                        match r {
                            Some(rs) if rs != s => {
                                if matches_dir(*dir, rs, s) {
                                    let th = self.locate(&step, &rhs.scratch, ei, prev, tk, s, eps_t)?;
                                    found.push((ei, th));
                                    break;
                                }
                                r = Some(s);
                            }
                            None => r = Some(s),
                            _ => {}
                        }
                        prev = tk;
                    }
                }
                new_refs[ei] = r;
            }

            if !found.is_empty() {
                let te = found.iter().map(|x| x.1).fold(f64::INFINITY, f64::min);
                let due: Vec<usize> = found.iter().filter(|x| x.1 <= te + eps_t).map(|x| x.0).collect();
                step.t1 = te;
                let mut ye = vec![0.0; n_y];
                step.eval(te, &mut ye);
                self.run.segments.push(Segment { step, base });
                self.t = te;
                self.n = 0;
                cm.load_y(&mut self.vals, &ye);
                cm.update_derived(&mut self.vals, self.t, self.cfg.t0).map_err(|s| self.diag(Category::Model, s.cause, None))?;
                self.commit();
                self.after_step_checks()?;
                self.iterate(due, vec![])?;
                self.retake_refs();
                h = None;
                continue;
            }
            self.run.segments.push(Segment { step, base });
            self.t = t1;
            self.n = 0;
            cm.load_y(&mut self.vals, &y1);
            cm.update_derived(&mut self.vals, self.t, self.cfg.t0).map_err(|s| self.diag(Category::Model, s.cause, None))?;
            self.refs = new_refs;
            self.after_step_checks()?;
        }
        let last = self.run.committed.last().unwrap();
        if last.t != self.t {
            self.commit();
        }
        Ok(())
    }

    fn guard_at(&self, step: &Step, scratch: &[Value], t: f64, ei: usize) -> Result<f64, Status> {
        let mut y = vec![0.0; self.cm.n_y()];
        step.eval(t, &mut y);
        let mut vals = scratch.to_vec();
        self.cm.load_y(&mut vals, &y);
        self.cm.update_derived(&mut vals, t, self.cfg.t0)?;
        let c = Ctx { vals: &vals, der: None, t, t0: self.cfg.t0, args: &[] };
        match &self.cm.events[ei].trigger {
            Some(CTrigger::Crossing { guard, .. }) => Ok(guard.eval(&c)?.num()),
            _ => unreachable!(),
        }
    }

    /// Bisection on the dense output until the bracket is at most `eps_t` wide (RC-7.5).
    /// Returns `t_hi`, the far end of the bracket (RC-7.6).
    fn locate(&self, step: &Step, scratch: &[Value], ei: usize, mut lo: f64, mut hi: f64, new_sign: bool, eps_t: f64) -> Result<f64, RunDiag> {
        while hi - lo > eps_t {
            let mid = 0.5 * (lo + hi);
            if mid <= lo || mid >= hi {
                break;
            }
            let g = self.guard_at(step, scratch, mid, ei).map_err(|s| self.diag(Category::Model, s.cause, None))?;
            if sign(g) == Some(new_sign) {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        Ok(hi)
    }

    fn zeno_detected(&mut self, ei: usize) -> bool {
        let ev = &self.cm.events[ei];
        let Some(z) = &ev.zeno else { return false };
        let eps = z.eps.or(self.cfg.eps_zeno).unwrap_or(1e-6 * self.cfg.t_ref);
        let n = z.n.unwrap_or(self.cfg.n_zeno) as usize;
        let w = z.window.or(self.cfg.w_zeno).unwrap_or(self.cfg.t_ref);
        let m = &mut self.zeno[ei];
        let t = self.t;
        while m.recent.front().map(|&x| x < t - w).unwrap_or(false) {
            m.recent.pop_front();
        }
        let close = m.last.map(|l| t != l && t - l < eps).unwrap_or(false);
        close || m.recent.len() + 1 > n
    }

    fn record_occurrence(&mut self, ei: usize) {
        let m = &mut self.zeno[ei];
        if m.last != Some(self.t) {
            m.last = Some(self.t);
            m.recent.push_back(self.t);
        }
    }

    /// Event iteration at the current event time (RC section 8.1).
    fn iterate(&mut self, mut due: Vec<usize>, mut requested: Vec<usize>) -> Result<(), RunDiag> {
        loop {
            // Step 2: enabling conditions.
            let vals = self.vals.clone();
            due.retain(|&ei| match &self.cm.events[ei].enable {
                Some(c) => matches!(c.eval(&self.ctx(&vals)), Ok(Value::Bool(true))),
                None => true,
            });
            due.sort();
            due.dedup();
            if due.is_empty() && requested.is_empty() {
                return Ok(());
            }
            if self.n >= self.cfg.n_micro {
                return Err(self.diag(Category::Cascade, format!("more than {} microsteps at t = {}", self.cfg.n_micro, self.t), None));
            }
            // Zeno policies replace handlers at an accumulating occurrence (RC-9.3).
            let mut handled: Vec<(usize, bool, bool)> = vec![];
            let mut ops: Vec<COp> = vec![];
            for &ei in &due {
                let is_crossing = matches!(self.cm.events[ei].trigger, Some(CTrigger::Crossing { .. }));
                if is_crossing && self.zeno_detected(ei) {
                    match self.cm.events[ei].zeno.as_ref().unwrap().settle.clone() {
                        None => {
                            return Err(self.diag(Category::Zeno, format!("event accumulation detected for `{}`", self.cm.events[ei].name), Some(&self.cm.events[ei].id)));
                        }
                        Some(settle) => {
                            ops.extend(settle);
                            handled.push((ei, true, false));
                        }
                    }
                } else {
                    ops.extend(self.cm.events[ei].handler.clone());
                    handled.push((ei, false, false));
                }
            }
            for &ei in &requested {
                ops.extend(self.cm.events[ei].handler.clone());
                handled.push((ei, false, true));
            }
            let emitted = self.transition(&ops, false)?;
            for &(ei, z, rq) in &handled {
                if matches!(self.cm.events[ei].trigger, Some(CTrigger::Crossing { .. })) {
                    self.record_occurrence(ei);
                }
                self.run.log.push(LogEntry { event: ei, name: self.cm.events[ei].name.clone(), t: self.t, n: self.n, zeno: z, requested: rq });
            }
            // Next microstep: emitted events (MK-15.10) and crossings caused by jumps (RC-8.5).
            let mut next: Vec<usize> = emitted;
            for ei in 0..self.cm.events.len() {
                if let Some(CTrigger::Crossing { dir, .. }) = &self.cm.events[ei].trigger {
                    let before = self.run.committed[self.run.committed.len() - 2].vals.clone();
                    let (a, b) = match (self.guard_value(&before, ei), self.guard_value(&self.vals.clone(), ei)) {
                        (Ok(Some(a)), Ok(Some(b))) => (a, b),
                        _ => continue,
                    };
                    if let (Some(sa), Some(sb)) = (sign(a), sign(b)) {
                        if sa != sb && matches_dir(*dir, sa, sb) {
                            next.push(ei);
                        }
                    }
                }
            }
            due = next;
            requested = vec![];
        }
    }

    /// One transition (MK-16.3): conflicts, proposed state, constraints, commit.
    /// Returns the events emitted. For interventions, a `reject` violation is returned as an
    /// error with category `Intervention`.
    fn transition(&mut self, ops: &[COp], intervention: bool) -> Result<Vec<usize>, RunDiag> {
        let mut targets: Vec<(usize, Option<usize>)> = vec![];
        let mut proposed = self.vals.clone();
        let mut emitted = vec![];
        for op in ops {
            match op {
                COp::Set { binding, component, value } => {
                    if targets.iter().any(|(b, c)| b == binding && (c.is_none() || component.is_none() || c == component)) {
                        return Err(self.diag(Category::Conflict, format!("two operations on `{}` in one transition", self.cm.bindings[*binding].name), None));
                    }
                    targets.push((*binding, *component));
                    let v = value.eval(&self.ctx(&self.vals)).map_err(|s| self.diag(Category::Model, s.cause, None))?;
                    match component {
                        None => proposed[*binding] = v,
                        Some(k) => {
                            let mut a = proposed[*binding].arr();
                            a.v[*k] = v.num();
                            proposed[*binding] = match proposed[*binding] {
                                Value::Point(_) => Value::Point(a),
                                _ => Value::Vec(a),
                            };
                        }
                    }
                }
                COp::Emit { event } => emitted.push(*event),
            }
        }
        self.cm.update_derived(&mut proposed, self.t, self.cfg.t0).map_err(|s| self.diag(Category::Model, s.cause, None))?;
        match self.check_constraints(&proposed, false) {
            Ok(reports) => self.run.diagnostics.extend(reports),
            Err(d) if intervention => return Err(RunDiag { category: Category::Intervention, ..d }),
            Err(d) => return Err(d),
        }
        self.vals = proposed;
        self.n += 1;
        self.commit();
        Ok(emitted)
    }

    /// Applies one logged action after the model's own iteration at this instant (RC-8.7, RC-11.1).
    fn intervene(&mut self, action: &Action) -> Result<(), RunDiag> {
        match action {
            Action::Intervene(ops) => {
                let cops = match check_intervention(self.cm, ops) {
                    Ok(c) => c,
                    Err(d) => {
                        let msg = d.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("; ");
                        let rd = self.diag(Category::Intervention, msg, None);
                        self.run.rejected.push(rd);
                        return Ok(());
                    }
                };
                match self.transition(&cops, true) {
                    Ok(emitted) => self.iterate(emitted, vec![]),
                    Err(d) if d.category == Category::Intervention => {
                        self.run.rejected.push(d);
                        Ok(())
                    }
                    Err(d) => Err(d),
                }
            }
            Action::Request(id) => {
                let Some(ei) = self.cm.events.iter().position(|e| &e.id == id) else {
                    let rd = self.diag(Category::Intervention, format!("unknown event `{id}`"), None);
                    self.run.rejected.push(rd);
                    return Ok(());
                };
                if !matches!(self.cm.events[ei].trigger, Some(CTrigger::Request)) {
                    let rd = self.diag(Category::Intervention, format!("event `{}` is not requestable (D-027)", self.cm.events[ei].name), Some(id));
                    self.run.rejected.push(rd);
                    return Ok(());
                }
                self.iterate(vec![], vec![ei])
            }
        }
    }
}

// ------------------------------------------------------------------ observation (RC section 15)

impl Run {
    /// Compiles an observation expression over this run's model.
    pub fn compile(&self, e: &Expr) -> CExpr {
        compile_expr(&self.model, e, None).unwrap_or_else(|d| panic!("observation does not check: {d:?}")).0
    }

    fn eval_on(&self, c: &CExpr, vals: &[Value], t: f64) -> Result<Value, Status> {
        let der = self.model.der_values(vals, t, self.config.t0).ok();
        c.eval(&Ctx { vals, der: der.as_deref(), t, t0: self.config.t0, args: &[] })
    }

    /// The state at time `t`: the final microstep at an event time (RC-4.3), otherwise the
    /// dense output (RC-15.1).
    pub fn state_at(&self, t: f64) -> Vec<Value> {
        if let Some(c) = self.committed.iter().rev().find(|c| c.t == t) {
            return c.vals.clone();
        }
        let seg = self
            .segments
            .iter()
            .find(|s| s.step.t0 <= t && t <= s.step.t1)
            .unwrap_or_else(|| panic!("time {t} is outside the run"));
        let mut y = vec![0.0; self.model.n_y()];
        seg.step.eval(t, &mut y);
        let mut vals = self.committed[seg.base].vals.clone();
        self.model.load_y(&mut vals, &y);
        self.model.update_derived(&mut vals, t, self.config.t0).expect("derived values");
        vals
    }

    /// `expr at(τ)` (PK-3.1).
    pub fn at(&self, t: f64, e: &Expr) -> Value {
        let c = self.compile(e);
        self.eval_on(&c, &self.state_at(t), t).expect("observation value")
    }

    /// `expr on(E)` at the final microstep of each occurrence, or at a given microstep (PK-3.1).
    pub fn on(&self, event: &str, e: &Expr, microstep: Option<u32>) -> Vec<Value> {
        let c = self.compile(e);
        self.log
            .iter()
            .filter(|l| self.model.events[l.event].name == event)
            .map(|l| {
                let state = match microstep {
                    Some(n) => self.committed.iter().find(|s| s.t == l.t && s.n == n).expect("microstep"),
                    None => self.committed.iter().rev().find(|s| s.t == l.t).expect("event instant"),
                };
                self.eval_on(&c, &state.vals, l.t).expect("observation value")
            })
            .collect()
    }

    /// `expr every(Δ)` from `t0` to the end of the run (PK-3.1).
    pub fn every(&self, dt: f64, e: &Expr) -> Vec<(f64, Value)> {
        let c = self.compile(e);
        let end = self.committed.last().map(|s| s.t).unwrap_or(self.config.t0);
        let mut out = vec![];
        let mut k = 0u64;
        loop {
            let t = self.config.t0 + k as f64 * dt;
            if t > end {
                break;
            }
            out.push((t, self.eval_on(&c, &self.state_at(t), t).expect("observation value")));
            k += 1;
        }
        out
    }

    /// Times of each occurrence of an event.
    pub fn times(&self, event: &str) -> Vec<f64> {
        self.log.iter().filter(|l| l.name == event).map(|l| l.t).collect()
    }

    /// The value of a binding in the final state.
    pub fn final_value(&self, binding: &str) -> Value {
        self.committed.last().unwrap().vals[self.model.idx(binding)].clone()
    }
}

/// The type of an observation expression.
pub fn observation_type(cm: &CModel, e: &Expr) -> Type {
    compile_expr(cm, e, None).expect("checks").1
}
