//! Observations of a run (PK section 3). Observing never changes the run (PK-3.2).

use crate::number;
use prismal_ir::present::{Observation, Schedule, Source};
use prismal_ir::Id;
use prismal_kernel::{CModel, Value};
use prismal_runtime::{Run, RunDiag, Scheduled};

/// An event occurrence (RC-15.1).
#[derive(Clone, Debug, PartialEq)]
pub struct EventRec {
    pub event: Id,
    pub name: String,
    pub t: f64,
    pub n: u32,
    pub zeno: bool,
    /// The occurrence's payload (D-050).
    pub payload: Option<prismal_kernel::Value>,
}

/// The data an observation produces (PK-3.5).
#[derive(Clone, Debug)]
pub enum Data {
    Value(Value),
    /// Values with the simulation instants they refer to.
    Series(Vec<(f64, Value)>),
    Events(Vec<EventRec>),
    Diagnostics(Vec<RunDiag>),
    Interventions(Vec<Scheduled>),
}

impl Data {
    pub fn len(&self) -> usize {
        match self {
            Data::Value(_) => 1,
            Data::Series(s) => s.len(),
            Data::Events(e) => e.len(),
            Data::Diagnostics(d) => d.len(),
            Data::Interventions(i) => i.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// The event log of a run as records.
pub fn events(cm: &CModel, run: &Run) -> Vec<EventRec> {
    run.log
        .iter()
        .map(|l| EventRec { event: cm.events[l.event].id.clone(), name: l.name.clone(), t: l.t, n: l.n, zeno: l.zeno, payload: l.payload.clone() })
        .collect()
}

fn window(cm: &CModel, run: &Run, schedule: &Schedule) -> Result<Option<(f64, f64, bool, bool)>, String> {
    Ok(match schedule {
        Schedule::Over { from, to, lo_closed, hi_closed } => {
            let a = number(cm, from)?;
            let b = match to {
                Some(e) => number(cm, e)?,
                None => run.end_time(),
            };
            Some((a, b, *lo_closed, *hi_closed))
        }
        _ => None,
    })
}

fn inside(t: f64, w: Option<(f64, f64, bool, bool)>) -> bool {
    match w {
        None => true,
        Some((a, b, lc, hc)) => (if lc { t >= a } else { t > a }) && (if hc { t <= b } else { t < b }),
    }
}

/// Observes a run. `interventions` is the committed intervention log of the run's session.
pub fn observe(cm: &CModel, run: &Run, obs: &Observation, interventions: &[Scheduled]) -> Result<Data, String> {
    let w = window(cm, run, &obs.schedule)?;
    Ok(match &obs.source {
        Source::EventLog { event, zeno_only } => Data::Events(
            events(cm, run)
                .into_iter()
                .filter(|e| event.as_ref().is_none_or(|x| *x == e.event) && (!zeno_only || e.zeno) && inside(e.t, w))
                .collect(),
        ),
        Source::Diagnostics { element } => Data::Diagnostics(
            run.diagnostics
                .iter()
                .filter(|d| element.as_ref().is_none_or(|x| d.element.as_ref() == Some(x)) && inside(d.t, w))
                .cloned()
                .collect(),
        ),
        Source::InterventionLog => Data::Interventions(interventions.iter().filter(|s| inside(s.t, w)).cloned().collect()),
        Source::Expr { expr } => {
            let c = run.compile(expr);
            let eval = |vals: &[Value], t: f64| run.eval_state(&c, vals, t).map_err(|s| s.cause);
            match &obs.schedule {
                Schedule::Live => {
                    let t = run.end_time();
                    Data::Value(eval(&run.state_at(t), t)?)
                }
                Schedule::At { time } => {
                    let t = number(cm, time)?;
                    if t > run.end_time() {
                        return Err(format!("instant {t} s is after the end of the run"));
                    }
                    Data::Value(eval(&run.state_at(t), t)?)
                }
                Schedule::Every { period } => {
                    let dt = number(cm, period)?;
                    Data::Series(run.every(dt, expr))
                }
                Schedule::On { event, microstep } => {
                    let name = cm.ir.events.iter().find(|e| &e.id == event).map(|e| e.name.clone()).ok_or("unknown event")?;
                    let times = run.times(&name);
                    Data::Series(times.into_iter().zip(run.on(&name, expr, *microstep)).collect())
                }
                Schedule::Over { .. } | Schedule::Run => {
                    let mut out = vec![];
                    for c in run.committed.iter().filter(|c| inside(c.t, w)) {
                        out.push((c.t, eval(&c.vals, c.t)?));
                    }
                    Data::Series(out)
                }
            }
        }
    })
}
