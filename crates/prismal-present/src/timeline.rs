//! The explanation timeline and its lesson run (PK sections 8 and 9).
//!
//! Playing a timeline is deterministic (PK-8.7): given the model, the lesson run's
//! configuration, the presentation, the medium and the learner's inputs, it produces the
//! beat timings, the time mapping from presentation time to simulation time, every run the
//! lesson used (the lesson run and the learner's branches, each as a sequence of versions),
//! captions, announcements, and refusals. Frames at any presentation instant are projected
//! from that record.
//!
//! Runs are never changed in place: an intervention or request makes a new version of the
//! run, recomputed from its start with the action in its log (RC-11.1). Segments of the
//! time mapping refer to the version they showed, so earlier frames never change.

use crate::frame::{compile_rep, CRep, Frame, Projector, ViewCtx};
use crate::{number, PDiag, Program};
use prismal_ir::present::{Action as TAction, LearnerInput, Presentation, RevealStyle};
use prismal_ir::{Id, Op};
use prismal_kernel::{CModel, Value};
use prismal_runtime::{run, Action, Config, Run, Scheduled};
use crate::frame::{each_rep_mut, retain_reps, Shape};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Medium {
    /// Interactive playback: explore beats open for the learner (D-018).
    Interactive,
    /// Linear media: explore beats use their fallback (PK-9.10).
    Video,
}

/// A learner input at a presentation instant (a learner script, README of the programs).
#[derive(Clone, Debug)]
pub struct Input {
    pub at: f64,
    pub input: LearnerInput,
}

/// One version of a run of the lesson.
#[derive(Clone, Debug)]
pub struct RunVersion {
    /// `lesson`, or `branch n` for the n-th learner branch (RC-12.3).
    pub lineage: String,
    pub config: Config,
    pub run: Run,
}

/// A piece of the time mapping (PK-8.2): from `p0` to `p1` presentation time, run version
/// `run` shows simulation time `s0 + rate (p - p0)`.
#[derive(Clone, Debug, Serialize)]
pub struct Segment {
    pub p0: f64,
    pub p1: f64,
    pub run: usize,
    pub s0: f64,
    pub rate: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BeatTime {
    pub id: Id,
    pub name: String,
    pub start: f64,
    pub end: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Caption {
    pub text: String,
    pub start: f64,
    pub end: f64,
}

/// An event occurrence announced to assistive technology when playback passes it (PK-11.3).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Announcement {
    pub at: f64,
    pub run: usize,
    pub event: String,
    pub t: f64,
}

#[derive(Clone, Debug)]
pub struct Shown {
    pub from: f64,
    pub until: f64,
    pub view: Option<Id>,
    pub rep: CRep,
    /// A reveal: its style and duration (D-042).
    pub reveal: Option<(RevealStyle, f64)>,
}

/// A camera move (D-042): from presentation instant `from`, over `duration`.
#[derive(Clone, Debug)]
pub struct CameraCue {
    pub view: Id,
    pub from: f64,
    pub duration: f64,
    pub center: Option<prismal_kernel::CExpr>,
    pub zoom: Option<f64>,
}

/// The state of a representation `k` of the way through a reveal (D-042): a path is drawn
/// up to `k`, anything else fades in; a group's members are handled one by one.
fn reveal(r: &mut crate::frame::RepFrame, style: RevealStyle, k: f64) {
    let path = matches!(r.shape, Shape::Polyline { .. } | Shape::Polygon { .. } | Shape::Segment { .. } | Shape::Arrow { .. });
    if style == RevealStyle::Draw && path {
        r.drawn = Some(k);
    } else if !matches!(r.shape, Shape::Group { .. }) || style == RevealStyle::Fade {
        r.opacity = Some(k);
    }
}

/// Smooth start and end of an animation (PK-8.4): `3k² - 2k³`.
pub fn ease(k: f64) -> f64 {
    let k = k.clamp(0.0, 1.0);
    k * k * (3.0 - 2.0 * k)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Highlight {
    pub target: Id,
    pub from: f64,
    pub until: f64,
}

/// A learner input the presentation did not act on (PK-9.8).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Refusal {
    pub at: f64,
    pub input: String,
    pub reason: String,
}

/// The record of one playback of a timeline.
pub struct Playback<'a> {
    pub cm: &'a CModel,
    pub pres: &'a Presentation,
    pub projector: Projector,
    pub medium: Medium,
    pub runs: Vec<RunVersion>,
    pub segments: Vec<Segment>,
    pub beats: Vec<BeatTime>,
    pub captions: Vec<Caption>,
    pub announcements: Vec<Announcement>,
    pub shown: Vec<Shown>,
    pub highlights: Vec<Highlight>,
    /// Representations hidden from a presentation instant on (PK-9.2), fading out over a
    /// duration (0 for at once).
    pub hidden: Vec<(Id, f64, f64)>,
    pub cameras: Vec<CameraCue>,
    pub refusals: Vec<Refusal>,
    /// Timeline diagnostics: unsatisfiable waits (PK-9.4), rejected interventions.
    pub diagnostics: Vec<String>,
    /// Elements the medium cannot present and that have no fallback (PK-12.3).
    pub unsupported: Vec<String>,
    /// The run version current at the end of the lesson.
    pub current: usize,
    /// Exact simulation instants at presentation instants where a `wait_until` ended: the
    /// event's located instant, not the mapping's rounded arithmetic (PK-9.3).
    pub pins: Vec<(f64, usize, f64)>,
    pub end: f64,
}

/// Reading time of a narration without a stated duration: 0.4 s per word, at least 2 s.
pub fn reading_time(text: &str) -> f64 {
    (text.split_whitespace().count() as f64 * 0.4).max(2.0)
}

struct Player<'a, 'b> {
    pb: Playback<'a>,
    inputs: Vec<(Input, bool)>,
    branches: usize,
    lesson: usize,
    open: Segment,
    _p: std::marker::PhantomData<&'b ()>,
}

impl Player<'_, '_> {
    fn cm(&self) -> &CModel {
        self.pb.cm
    }

    fn num(&mut self, e: &prismal_ir::Expr) -> f64 {
        match number(self.pb.cm, e) {
            Ok(x) => x,
            Err(m) => {
                self.pb.diagnostics.push(m);
                0.0
            }
        }
    }

    fn sim_at(&self, p: f64) -> f64 {
        if let Some(pin) = self.pb.pins.iter().rev().find(|x| x.0 == p && x.1 == self.open.run) {
            return pin.2;
        }
        let s = self.open.s0 + self.open.rate * (p - self.open.p0);
        s.min(self.pb.runs[self.open.run].run.end_time())
    }

    /// Closes the open segment at `p` and opens a new one.
    fn map(&mut self, p: f64, run: usize, s0: f64, rate: f64) {
        let mut seg = std::mem::replace(&mut self.open, Segment { p0: p, p1: f64::NAN, run, s0, rate });
        seg.p1 = p;
        self.close(seg);
    }

    fn close(&mut self, seg: Segment) {
        if seg.rate > 0.0 && seg.p1 > seg.p0 {
            let r = &self.pb.runs[seg.run].run;
            let s1 = (seg.s0 + seg.rate * (seg.p1 - seg.p0)).min(r.end_time());
            for l in r.log.iter().filter(|l| l.t > seg.s0 && l.t <= s1) {
                self.pb.announcements.push(Announcement { at: seg.p0 + (l.t - seg.s0) / seg.rate, run: seg.run, event: l.name.clone(), t: l.t });
            }
        }
        self.pb.segments.push(seg);
    }

    fn cur(&self) -> usize {
        self.pb.current
    }

    /// Applies an intervention or request to the current run at the displayed instant: a
    /// new version of that run (RC-11.1).
    fn modify(&mut self, p: f64, action: Action) {
        let s = self.sim_at(p);
        let old = &self.pb.runs[self.cur()];
        let mut cfg = old.config.clone();
        cfg.log.push(Scheduled { t: s, action });
        let new = run(self.cm(), cfg.clone());
        if new.rejected.len() > old.run.rejected.len() {
            let d = new.rejected.last().unwrap();
            self.pb.diagnostics.push(format!("at {p} s: rejected: {}", d.message));
            return;
        }
        let before = old.run.log.iter().filter(|l| l.t == s).count();
        for l in new.log.iter().filter(|l| l.t == s).skip(before) {
            self.pb.announcements.push(Announcement { at: p, run: self.pb.runs.len(), event: l.name.clone(), t: l.t });
        }
        let lineage = old.lineage.clone();
        self.pb.runs.push(RunVersion { lineage, config: cfg, run: new });
        self.pb.current = self.pb.runs.len() - 1;
        if lineage_is(&self.pb.runs[self.pb.current], "lesson") {
            self.lesson = self.pb.current;
        }
        let rate = self.open.rate;
        self.map(p, self.pb.current, s, rate);
    }

    fn branch(&mut self, p: f64) -> usize {
        self.branches += 1;
        let s = self.sim_at(p);
        let v = RunVersion { lineage: format!("branch {}", self.branches), ..self.pb.runs[self.cur()].clone() };
        self.pb.runs.push(v);
        self.pb.current = self.pb.runs.len() - 1;
        let rate = self.open.rate;
        self.map(p, self.pb.current, s, rate);
        self.pb.current
    }

    /// Run-directing actions take effect at `p`, before presentation time passes.
    fn direct(&mut self, a: &TAction, p: f64) {
        match a {
            TAction::Hold => {
                let s = self.sim_at(p);
                self.map(p, self.cur(), s, 0.0);
            }
            TAction::Run { rate } => {
                let r = self.num(rate);
                let s = self.sim_at(p);
                self.map(p, self.cur(), s, r);
            }
            TAction::Seek { time } => {
                let s = self.num(time);
                let rate = self.open.rate;
                self.map(p, self.cur(), s, rate);
            }
            TAction::Reset => {
                let t0 = self.pb.runs[self.cur()].config.t0;
                let rate = self.open.rate;
                self.map(p, self.cur(), t0, rate);
            }
            TAction::Branch => {
                self.branch(p);
            }
            TAction::Intervene { ops } => self.modify(p, Action::Intervene(ops.clone())),
            TAction::Request { event } => self.modify(p, Action::Request(event.clone())),
            _ => unreachable!("not run-directing"),
        }
    }

    /// Actions that start together at `p` (a beat, or a fallback): run-directing ones first,
    /// in written order (PK-9.2a, D-033). Returns when the last one ends.
    fn group(&mut self, acts: &[TAction], p: f64, beat: &str) -> f64 {
        for a in acts.iter().filter(|a| a.is_run_directing()) {
            self.direct(a, p);
        }
        let mut end = p;
        for a in acts.iter().filter(|a| !a.is_run_directing()) {
            end = end.max(self.timed(a, p, beat));
        }
        end
    }

    fn timed(&mut self, a: &TAction, p: f64, beat: &str) -> f64 {
        match a {
            TAction::Show { view, reps } => {
                let ctx = self.pb.projector.ctx(view.as_deref());
                for r in reps {
                    match compile_rep(self.pb.cm, &ctx, r) {
                        Ok(c) => self.pb.shown.push(Shown { from: p, until: f64::INFINITY, view: view.clone(), rep: c, reveal: None }),
                        Err(d) => self.pb.diagnostics.extend(d.into_iter().map(|x| x.to_string())),
                    }
                }
                p
            }
            TAction::Highlight { target } => {
                self.pb.highlights.push(Highlight { target: target.clone(), from: p, until: f64::NAN });
                p
            }
            TAction::Hide { target, duration } => {
                let d = duration.as_ref().map(|d| self.num(d)).unwrap_or(0.0);
                self.pb.hidden.push((target.clone(), p, d));
                p + d
            }
            TAction::Reveal { view, style, duration, reps } => {
                let d = duration.as_ref().map(|d| self.num(d)).unwrap_or(1.0);
                let ctx = self.pb.projector.ctx(view.as_deref());
                for r in reps {
                    match compile_rep(self.pb.cm, &ctx, r) {
                        Ok(c) => self.pb.shown.push(Shown { from: p, until: f64::INFINITY, view: view.clone(), rep: c, reveal: Some((*style, d)) }),
                        Err(e) => self.pb.diagnostics.extend(e.into_iter().map(|x| x.to_string())),
                    }
                }
                p + d
            }
            TAction::Camera { view, center, zoom, duration } => {
                let d = duration.as_ref().map(|d| self.num(d)).unwrap_or(1.0);
                let z = zoom.as_ref().map(|z| self.num(z));
                let c = match (center, self.pb.projector.ctx(Some(view))) {
                    (Some(e), ViewCtx::Spatial { space, .. }) => match prismal_kernel::compile_expr(self.pb.cm, e, Some(&prismal_ir::Type::Point { space })) {
                        Ok((ce, _)) => Some(ce),
                        Err(ds) => {
                            self.pb.diagnostics.extend(ds.iter().map(|x| x.to_string()));
                            None
                        }
                    },
                    _ => None,
                };
                self.pb.cameras.push(CameraCue { view: view.clone(), from: p, duration: d, center: c, zoom: z });
                p + d
            }
            TAction::Narrate { text, duration } => {
                let d = match duration {
                    Some(e) => self.num(e),
                    None => reading_time(text),
                };
                self.pb.captions.push(Caption { text: text.clone(), start: p, end: p + d });
                p + d
            }
            TAction::Wait { duration } => p + self.num(duration),
            TAction::WaitUntil { event } => {
                let name = self.pb.cm.ir.events.iter().find(|e| &e.id == event).map(|e| e.name.clone()).unwrap_or_default();
                let s = self.sim_at(p);
                let rate = self.open.rate;
                let r = &self.pb.runs[self.cur()].run;
                if rate <= 0.0 {
                    self.pb.diagnostics.push(format!("{beat}: waits for `{name}` while the simulation holds; the wait cannot be satisfied (PK-9.4)"));
                    return p;
                }
                match r.log.iter().find(|l| l.name == name && l.t > s) {
                    Some(l) => {
                        let end = p + (l.t - s) / rate;
                        let (run, t) = (self.open.run, l.t);
                        self.pb.pins.push((end, run, t));
                        end
                    }
                    None => {
                        let end = r.end_time();
                        self.pb.diagnostics.push(format!("{beat}: `{name}` does not occur before the run ends at {end} s (PK-9.4)"));
                        p + (end - s).max(0.0) / rate
                    }
                }
            }
            TAction::Sequence { actions } => {
                let mut q = p;
                for x in actions {
                    if x.is_run_directing() {
                        self.direct(x, q);
                    } else {
                        q = self.timed(x, q, beat);
                    }
                }
                q
            }
            TAction::Explore { limit, keep, controls, fallback } => match self.pb.medium {
                Medium::Video => {
                    if fallback.is_empty() {
                        self.pb.unsupported.push(format!("{beat}: an explore beat without a fallback cannot be shown in linear media (PK-9.10, PK-12.3)"));
                        p
                    } else {
                        self.group(fallback, p, beat)
                    }
                }
                Medium::Interactive => self.explore(p, limit.as_ref(), keep, controls, beat),
            },
            other => {
                self.direct(other, p);
                p
            }
        }
    }

    /// An explore beat (PK-9.8, PK-9.9, D-025): the lesson run is branched at the displayed
    /// instant; the learner's inputs act on the branch through the declared controls.
    fn explore(&mut self, p: f64, limit: Option<&prismal_ir::Expr>, keep: &[Id], controls: &[prismal_ir::present::Rep], beat: &str) -> f64 {
        let back_to = self.cur();
        let s_branch = self.sim_at(p);
        let rate = self.open.rate;
        self.branch(p);
        let limit = limit.map(|l| self.num(l));
        let deadline = limit.map(|l| p + l).unwrap_or(f64::INFINITY);
        let mut end = None;
        for i in 0..self.inputs.len() {
            let (inp, used) = &self.inputs[i];
            if *used || inp.at < p {
                continue;
            }
            if inp.at > deadline {
                break;
            }
            let inp = inp.clone();
            self.inputs[i].1 = true;
            match &inp.input {
                LearnerInput::Continue => {
                    end = Some(inp.at);
                    break;
                }
                LearnerInput::SetControl { control, binding, value } => {
                    let offered = controls.iter().any(|c| c.kind == *control && matches!(c.sources.first(), Some(prismal_ir::present::Arg::Expr { expr: prismal_ir::Expr::Ref { r#ref } }) if r#ref == binding));
                    if !offered {
                        self.pb.refusals.push(Refusal { at: inp.at, input: describe(&inp.input), reason: format!("{beat} offers no {control} for `{binding}` (PK-9.8)") });
                        continue;
                    }
                    let op = Op::Set { target: prismal_ir::Target { binding: binding.clone(), component: None }, value: value.clone() };
                    self.modify(inp.at, Action::Intervene(vec![op]));
                }
            }
        }
        let end = match end {
            Some(e) => e,
            None if deadline.is_finite() => deadline,
            None => {
                self.pb.diagnostics.push(format!("{beat}: the learner never continues and the beat has no limit; it ends at once"));
                p
            }
        };
        for c in controls {
            match compile_rep(self.pb.cm, &ViewCtx::Panel, c) {
                Ok(cr) => self.pb.shown.push(Shown { from: p, until: end, view: None, rep: cr, reveal: None }),
                Err(d) => self.pb.diagnostics.extend(d.into_iter().map(|x| x.to_string())),
            }
        }
        if keep.is_empty() {
            // Return to the lesson run at the instant of the branch (PK-9.9).
            let lesson = if lineage_is(&self.pb.runs[back_to], "lesson") { self.lesson } else { back_to };
            self.pb.current = lesson;
            self.map(end, lesson, s_branch, rate);
        }
        end
    }
}

fn lineage_is(v: &RunVersion, name: &str) -> bool {
    v.lineage == name
}

fn describe(i: &LearnerInput) -> String {
    match i {
        LearnerInput::Continue => "continue".into(),
        LearnerInput::SetControl { control, binding, .. } => format!("set {control} {binding}"),
    }
}

/// Plays a presentation's timeline over a lesson run configured by `base` (PK-9.5).
pub fn play<'a>(prog: &'a Program, presentation: &str, base: Config, medium: Medium, inputs: Vec<Input>) -> Result<Playback<'a>, Vec<PDiag>> {
    let pres = prog.presentation(presentation);
    let cm = prog.model(&pres.model);
    let projector = Projector::new(cm, pres)?;
    let Some(tl) = &pres.timeline else {
        return Err(vec![PDiag { code: "PK-E01", message: "the presentation has no timeline".into(), element: pres.id.clone() }]);
    };
    let lesson = run(cm, base.clone());
    let t0 = base.t0;
    let mut inputs: Vec<(Input, bool)> = inputs.into_iter().map(|i| (i, false)).collect();
    inputs.sort_by(|a, b| a.0.at.total_cmp(&b.0.at));
    let pb = Playback {
        cm,
        pres,
        projector,
        medium,
        runs: vec![RunVersion { lineage: "lesson".into(), config: base, run: lesson }],
        segments: vec![],
        beats: vec![],
        captions: vec![],
        announcements: vec![],
        shown: vec![],
        highlights: vec![],
        hidden: vec![],
        cameras: vec![],
        refusals: vec![],
        diagnostics: vec![],
        unsupported: vec![],
        current: 0,
        pins: vec![],
        end: 0.0,
    };
    // The lesson starts holding at t0 (PK-8.2).
    let mut pl = Player { pb, inputs, branches: 0, lesson: 0, open: Segment { p0: 0.0, p1: f64::NAN, run: 0, s0: t0, rate: 0.0 }, _p: Default::default() };
    let mut p = 0.0;
    for b in tl.scenes.iter().flat_map(|s| &s.beats) {
        let start = p;
        let n_hl = pl.pb.highlights.len();
        p = pl.group(&b.actions, p, &b.name);
        for h in &mut pl.pb.highlights[n_hl..] {
            h.until = p;
        }
        pl.pb.beats.push(BeatTime { id: b.id.clone(), name: b.name.clone(), start, end: p });
    }
    let last = std::mem::replace(&mut pl.open, Segment { p0: p, p1: p, run: 0, s0: 0.0, rate: 0.0 });
    pl.close(Segment { p1: p, ..last });
    // Inputs no explore beat took are refused: outside explore beats the learner has no
    // model actions (PK-9.8).
    for (inp, used) in &pl.inputs {
        if !used {
            let reason = match inp.input {
                LearnerInput::Continue => "no continue point at this instant".into(),
                LearnerInput::SetControl { .. } => "outside an explore beat the learner has no model actions (PK-9.8, D-025)".to_string(),
            };
            pl.pb.refusals.push(Refusal { at: inp.at, input: describe(&inp.input), reason });
        }
    }
    pl.pb.end = p;
    Ok(pl.pb)
}

impl Playback<'_> {
    pub fn beat(&self, name: &str) -> &BeatTime {
        self.beats.iter().find(|b| b.name == name || b.id == name).unwrap_or_else(|| panic!("no beat {name}"))
    }

    /// The last version of a lineage (`lesson`, `branch 1`).
    pub fn lineage(&self, name: &str) -> Option<&RunVersion> {
        self.runs.iter().rev().find(|v| v.lineage == name)
    }

    /// The segment of the time mapping shown at presentation instant `p`.
    pub fn segment_at(&self, p: f64) -> &Segment {
        self.segments.iter().rev().find(|s| s.p0 <= p).unwrap_or(&self.segments[0])
    }

    /// The run version and simulation instant shown at presentation instant `p` (PK-8.2).
    pub fn sim_at(&self, p: f64) -> (usize, f64) {
        let s = self.segment_at(p);
        if let Some(pin) = self.pins.iter().rev().find(|x| x.0 == p && x.1 == s.run) {
            return (s.run, pin.2);
        }
        let r = &self.runs[s.run].run;
        (s.run, (s.s0 + s.rate * (p - s.p0)).min(r.end_time()))
    }

    /// The frame description at presentation instant `p`; announcements are those due in
    /// `(p - dt, p]` (PK-12.1).
    pub fn frame(&self, p: f64, dt: f64) -> Frame {
        let (ri, t) = self.sim_at(p);
        let v = &self.runs[ri];
        let vals: Vec<Value> = v.run.state_at(t);
        let extra: Vec<(Option<Id>, CRep)> = self.shown.iter().filter(|s| s.from <= p && p < s.until).map(|s| (s.view.clone(), s.rep.clone())).collect();
        let (mut views, mut overlay) = self.projector.frame(self.cm, &v.run, &vals, t, &extra);
        // Hidden representations: gone after their fade, fading during it.
        // Members of groups are found by the same rules (D-043).
        let gone: Vec<&Id> = self.hidden.iter().filter(|h| h.1 + h.2 <= p).map(|h| &h.0).collect();
        let keep = |r: &crate::frame::RepFrame| !gone.contains(&&r.id);
        for v in views.iter_mut() {
            retain_reps(&mut v.reps, &keep);
        }
        retain_reps(&mut overlay, &keep);
        let mut animate = |r: &mut crate::frame::RepFrame| {
            if let Some(h) = self.hidden.iter().find(|h| h.0 == r.id && h.1 <= p && p < h.1 + h.2) {
                r.opacity = Some(1.0 - ease((p - h.1) / h.2));
            }
            if let Some(s) = self.shown.iter().find(|s| s.rep.rep.id == r.id && s.from <= p) {
                if let Some((style, d)) = s.reveal {
                    let k = if d > 0.0 { ease((p - s.from) / d) } else { 1.0 };
                    if k < 1.0 {
                        match (&mut r.shape, style) {
                            // A group drawn: its paths are drawn, its markers fade in.
                            (Shape::Group { members }, RevealStyle::Draw) => each_rep_mut(members, &mut |m| reveal(m, style, k)),
                            _ => reveal(r, style, k),
                        }
                    }
                }
            }
        };
        for v in views.iter_mut() {
            each_rep_mut(&mut v.reps, &mut animate);
        }
        each_rep_mut(&mut overlay, &mut animate);
        // Cameras (D-042): each move starts from where the previous one left the camera.
        for vf in views.iter_mut() {
            let ctx = self.projector.ctx(Some(&vf.id));
            let mut center: Option<[f64; 2]> = None;
            let mut zoom = 1.0;
            let mut blend = 1.0;
            let mut any = false;
            for c in self.cameras.iter().filter(|c| c.view == vf.id && c.from <= p) {
                any = true;
                let target = c.center.as_ref().and_then(|ce| v.run.eval_state(ce, &vals, t).ok()).map(|pt| ctx.to_view(&crate::flat(&pt))).or(center);
                let tz = c.zoom.unwrap_or(zoom);
                let k = if c.duration > 0.0 { ease((p - c.from) / c.duration) } else { 1.0 };
                zoom += (tz - zoom) * k;
                match (center, target) {
                    (Some(a), Some(b)) => {
                        center = Some([a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k]);
                        blend = 1.0;
                    }
                    (None, Some(b)) => {
                        center = Some(b);
                        blend = k;
                    }
                    _ => {}
                }
            }
            if any {
                vf.camera = Some(crate::frame::Camera { center, zoom, blend });
            }
        }
        for h in self.highlights.iter().filter(|h| h.from <= p && p < h.until) {
            let mut mark = |r: &mut crate::frame::RepFrame| {
                if r.id == h.target {
                    r.highlighted = true;
                }
            };
            for v in views.iter_mut() {
                each_rep_mut(&mut v.reps, &mut mark);
            }
            each_rep_mut(&mut overlay, &mut mark);
        }
        Frame {
            time: p,
            run: v.lineage.clone(),
            t,
            views,
            overlay,
            captions: self.captions.iter().filter(|c| c.start <= p && p < c.end).map(|c| c.text.clone()).collect(),
            announcements: self.announcements.iter().filter(|a| a.at > p - dt && a.at <= p).map(|a| a.event.clone()).collect(),
        }
    }

    /// Frame descriptions at `fps` frames per presentation second, from 0 to the end.
    pub fn export(&self, fps: f64) -> Vec<Frame> {
        let n = (self.end * fps).floor() as usize;
        (0..=n).map(|k| self.frame(k as f64 / fps, 1.0 / fps)).collect()
    }

    /// The values of an `on(E)` observation at each occurrence playback passed, with the
    /// presentation instant of each.
    pub fn observed(&self, observation: &str) -> Vec<(f64, Value)> {
        let Some(o) = self.pres.observations.iter().find(|o| o.name == observation) else { return vec![] };
        let (prismal_ir::present::Source::Expr { expr }, prismal_ir::present::Schedule::On { event, .. }) = (&o.source, &o.schedule) else { return vec![] };
        let name = self.cm.ir.events.iter().find(|e| &e.id == event).map(|e| e.name.clone()).unwrap_or_default();
        self.announcements
            .iter()
            .filter(|a| a.event == name)
            .map(|a| {
                let r = &self.runs[a.run].run;
                let c = r.compile(expr);
                (a.at, r.eval_state(&c, &r.state_at(a.t), a.t).expect("observation value"))
            })
            .collect()
    }
}
