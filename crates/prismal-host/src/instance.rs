//! An open presentation (HI section 4): an interactive session or a lesson playback, driven
//! by the host's clock and the learner's inputs, answering with layouts, frame descriptions
//! (PK-12.1) and observations as JSON.
//!
//! A presentation opens in one of two modes:
//! - interactive: an interactive session of the model (PK section 10), for presentations
//!   without a timeline; controls, drags and keys act on the session.
//! - lesson: a playback of the timeline (PK sections 8, 9). The host's clock chooses the
//!   presentation instant; learner inputs are recorded with their instants and the playback
//!   is recomputed from them, which is deterministic (PK-8.7), so frames before an input
//!   never change.

use std::rc::Rc;
use prismal_ir::present::{Action as TAction, LearnerInput, Observation, Schedule, Source};
use prismal_ir::Op;
use prismal_kernel::{compile_expr, CModel};
use prismal_present::data::Data;
use prismal_present::frame::{CKind, CRep, Frame, Shape, ViewCtx};
use prismal_present::interact::{si_literal, Interactive, Key};
use prismal_present::text::{fmt_binding, fmt_num, fmt_value, print, symbol, unit_text};
use prismal_present::timeline::{play, Input, Medium, Playback};
use prismal_present::{Program, LESSON_HORIZON};
use prismal_runtime::{Action, Config};
use serde_json::{json, Value as Json};

/// Least simulated span of an interactive session of a dynamic model.
pub const SESSION_HORIZON: f64 = LESSON_HORIZON;

enum Mode {
    Closed,
    Interactive(Box<Interactive>),
    Lesson(Box<Lesson>),
}

struct Lesson {
    pres: String,
    medium: Medium,
    inputs: Vec<Input>,
    pb: Playback,
}

impl Lesson {
    fn replay(prog: &Program, pres: &str, medium: Medium, inputs: Vec<Input>) -> Result<Lesson, Json> {
        let pb = play(prog, pres, Config::until(LESSON_HORIZON), medium, inputs.clone()).map_err(|ds| pdiags(prog, &ds))?;
        Ok(Lesson { pres: pres.into(), medium, inputs, pb })
    }
}

fn pdiags(_prog: &Program, ds: &[prismal_present::PDiag]) -> Json {
    Json::Array(ds.iter().map(|d| json!({ "code": d.code, "message": d.message, "element": d.element })).collect())
}

/// An instance: a checked program and at most one open presentation of it. The program is
/// shared with its document, and the instance keeps it alive (HI-2.2).
pub struct Instance {
    prog: Rc<Program>,
    mode: Mode,
}

impl Instance {
    /// An instance with no presentation open yet.
    pub fn new(prog: Rc<Program>) -> Instance {
        Instance { prog, mode: Mode::Closed }
    }

    /// `closed`, `interactive` or `lesson` (HI-4.1).
    pub fn mode(&self) -> &'static str {
        match self.mode {
            Mode::Closed => "closed",
            Mode::Interactive(_) => "interactive",
            Mode::Lesson(_) => "lesson",
        }
    }

    /// The program the instance presents.
    pub fn program(&self) -> &Program {
        &self.prog
    }

    /// Opens a presentation: as a lesson when it has a timeline (in the interactive medium,
    /// or `video` for linear media with fallbacks, PK-9.10), otherwise interactive.
    /// Returns its layout.
    pub fn open(&mut self, presentation: &str, video: bool) -> Result<Json, Json> {
        let pres = self.find_presentation(presentation)?;
        self.mode = if pres.timeline.is_some() {
            let medium = if video { Medium::Video } else { Medium::Interactive };
            Mode::Lesson(Box::new(Lesson::replay(&self.prog, &pres.id, medium, vec![])?))
        } else {
            // A dynamic model runs to the session horizon, or to the end of the longest time
            // axis the presentation plots; a static one has one instant.
            let cm = self.prog.model(&pres.model);
            let horizon = match prismal_present::frame::Projector::new(cm, &pres) {
                Ok(pj) => pj
                    .views
                    .iter()
                    .filter_map(|(_, ctx, _)| match ctx {
                        ViewCtx::Plot { x, dims, .. } if dims.0 == prismal_ir::Dim::time() => Some(x.1),
                        _ => None,
                    })
                    .fold(SESSION_HORIZON, f64::max),
                Err(_) => SESSION_HORIZON,
            };
            let i = Interactive::new(&self.prog, &pres.id, Config::until(horizon)).map_err(|ds| pdiags(&self.prog, &ds))?;
            Mode::Interactive(Box::new(i))
        };
        Ok(self.layout())
    }

    fn find_presentation(&self, name: &str) -> Result<prismal_ir::present::Presentation, Json> {
        self.prog.doc.presentations.iter().find(|p| p.name == name || p.id == name).cloned().ok_or_else(|| json!([{ "code": "HI-E02", "message": format!("no presentation `{name}`") }]))
    }

    fn cm(&self) -> Option<&CModel> {
        match &self.mode {
            Mode::Closed => None,
            Mode::Interactive(i) => Some(&i.cm),
            Mode::Lesson(l) => Some(&l.pb.cm),
        }
    }

    /// The views of the open presentation with their coordinate systems, the extent of
    /// their content (view coordinates, for the renderer's initial viewport), and for a
    /// lesson its beats, captions and explore windows.
    pub fn layout(&self) -> Json {
        let (projector, pres) = match &self.mode {
            Mode::Closed => return Json::Null,
            Mode::Interactive(i) => (&i.projector, &i.pres),
            Mode::Lesson(l) => (&l.pb.projector, &l.pb.pres),
        };
        let cm = self.cm().unwrap();
        let frames: Vec<Frame> = match &self.mode {
            // The first and last instants: a trace at the end covers the whole path.
            Mode::Interactive(i) => vec![i.frame_at(i.cfg_t0()), i.frame_at(i.end_time())],
            Mode::Lesson(l) => l.pb.export(10.0),
            Mode::Closed => vec![],
        };
        let views: Vec<Json> = projector
            .views
            .iter()
            .map(|(id, ctx, _)| {
                let name = pres.views.iter().find(|v| &v.id == id).map(|v| v.name.clone()).unwrap_or_default();
                let mut v = json!({ "id": id, "name": name, "kind": ctx.kind() });
                match ctx {
                    ViewCtx::Spatial { space, px_per_m, y_up } => {
                        let axes = cm.spaces.iter().find(|s| &s.id == space).map(|s| s.axes.clone()).unwrap_or_default();
                        v["px_per_m"] = json!(px_per_m);
                        v["y_up"] = json!(y_up);
                        v["axes"] = json!(axes);
                        v["extent"] = json!(extent(frames.iter().flat_map(|f| f.views.iter().filter(|x| &x.id == id)).flat_map(|x| x.reps.iter()).map(|r| &r.shape)));
                    }
                    ViewCtx::Plot { x, y, dims } => {
                        v["x"] = json!([x.0, x.1]);
                        v["y"] = json!([y.0, y.1]);
                        v["units"] = json!([unit_text(&dims.0), unit_text(&dims.1)]);
                    }
                    ViewCtx::Panel => {}
                }
                v
            })
            .collect();
        let permits: Vec<&String> = pres.permissions.iter().flat_map(|p| p.allows.iter()).collect();
        let mut out = json!({ "presentation": pres.name, "views": views, "permits": permits });
        match &self.mode {
            Mode::Lesson(l) => {
                out["mode"] = json!("lesson");
                out["lesson"] = self.lesson_info(l);
            }
            Mode::Interactive(i) => {
                out["mode"] = json!("interactive");
                out["session"] = self.session_info(i);
            }
            Mode::Closed => {}
        }
        out
    }

    /// The clock of an interactive session: `dynamic` when the model evolves in time, the
    /// instant shown, the run's span, its diagnostics and the number of interventions.
    fn session_info(&self, i: &Interactive) -> Json {
        let run = &i.session.current;
        json!({
            "dynamic": !i.cm.is_static,
            "t": i.t,
            "t0": run.config.t0,
            "end": i.end_time(),
            "interventions": i.session.log().len(),
            "diagnostics": run.diagnostics.iter().map(|d| d.message.clone()).collect::<Vec<_>>(),
        })
    }

    /// Shows simulation instant `t` in an interactive session (play and seek). Returns the
    /// session's clock.
    pub fn seek(&mut self, t: f64) -> Json {
        match &mut self.mode {
            Mode::Interactive(i) => {
                i.seek(t);
            }
            _ => return Json::Null,
        }
        let Mode::Interactive(i) = &self.mode else { unreachable!() };
        self.session_info(i)
    }

    /// A new run of the session, with an empty log, at `t0` (RC section 12).
    pub fn reset(&mut self) -> Json {
        if let Mode::Interactive(i) = &mut self.mode {
            i.reset();
        }
        self.layout()
    }

    /// The session's clock after an action (interventions change the run and its span).
    pub fn session(&self) -> Json {
        match &self.mode {
            Mode::Interactive(i) => self.session_info(i),
            _ => Json::Null,
        }
    }

    fn lesson_info(&self, l: &Lesson) -> Json {
        let pb = &l.pb;
        let explore: Vec<Json> = match l.medium {
            Medium::Video => vec![],
            Medium::Interactive => pb
                .pres
                .timeline
                .iter()
                .flat_map(|t| t.scenes.iter().flat_map(|s| s.beats.iter()))
                .filter(|b| b.actions.iter().any(|a| matches!(a, TAction::Explore { .. })))
                .map(|b| {
                    let bt = pb.beat(&b.id);
                    json!({ "beat": b.name, "start": bt.start, "end": bt.end })
                })
                .collect(),
        };
        let beats: Vec<Json> = pb
            .pres
            .timeline
            .iter()
            .flat_map(|t| t.scenes.iter().flat_map(|s| s.beats.iter().map(move |b| (s, b))))
            .map(|(s, b)| {
                let bt = pb.beat(&b.id);
                json!({ "scene": s.name, "beat": b.name, "start": bt.start, "end": bt.end })
            })
            .collect();
        json!({
            "medium": match l.medium { Medium::Video => "video", Medium::Interactive => "interactive" },
            "end": pb.end,
            "beats": beats,
            "explore": explore,
            "captions": pb.captions,
            "inputs": l.inputs.iter().map(|i| json!({ "at": i.at, "input": i.input })).collect::<Vec<_>>(),
            "refusals": pb.refusals,
            "diagnostics": pb.diagnostics,
            "unsupported": pb.unsupported,
        })
    }

    /// The frame description at presentation instant `p` (a lesson; announcements due in
    /// `(p - dt, p]`), or the session's current frame (interactive). Formulas carry their
    /// MathML typesetting.
    pub fn frame(&self, p: f64, dt: f64) -> Json {
        let frame = match &self.mode {
            Mode::Closed => return Json::Null,
            Mode::Interactive(i) => {
                let mut f = i.frame();
                if dt > 0.0 {
                    f.announcements = i.events_between(i.t - dt, i.t);
                }
                f
            }
            Mode::Lesson(l) => l.pb.frame(p, dt),
        };
        serde_json::to_value(&frame).expect("frame description")
    }

    /// The math box tree of a formula or equation representation (D-046), for a medium
    /// with its own math engine (MathML in a browser).
    pub fn math(&self, rep: &str) -> Option<prismal_present::math::MathBox> {
        let (cm, compiled): (&CModel, Vec<&CRep>) = match &self.mode {
            Mode::Closed => return None,
            Mode::Interactive(i) => (&i.cm, i.projector.views.iter().flat_map(|v| v.2.iter()).collect()),
            Mode::Lesson(l) => (&l.pb.cm, l.pb.projector.views.iter().flat_map(|v| v.2.iter()).chain(l.pb.shown.iter().map(|s| &s.rep)).collect()),
        };
        let c = compiled.into_iter().find(|c| c.rep.id == rep)?;
        match &c.kind {
            CKind::Formula { lhs, rhs, params, .. } => Some(prismal_present::math::formula(lhs, rhs, cm, params)),
            CKind::Equation { lhs, rhs, .. } => Some(prismal_present::math::equation(lhs, rhs, cm)),
            _ => None,
        }
    }

    /// The observations of the open presentation, as text lines per observation.
    pub fn observations(&self) -> Json {
        let mut out = serde_json::Map::new();
        match &self.mode {
            Mode::Closed => {}
            Mode::Interactive(i) => {
                for o in &i.pres.observations {
                    let text = match i.observe(&o.name) {
                        Ok(d) => fmt_data(&i.cm, &i.session.current, o, &d),
                        Err(e) => vec![e],
                    };
                    out.insert(o.name.clone(), json!(text));
                }
            }
            Mode::Lesson(l) => {
                for o in &l.pb.pres.observations {
                    if matches!(o.schedule, Schedule::On { .. }) {
                        let ty = obs_type(&l.pb.cm, o);
                        let lines: Vec<String> = l.pb.observed(&o.name).iter().map(|(p, v)| format!("at {} s: {}", fmt_num(*p), ty.as_ref().map(|t| fmt_value(v, t)).unwrap_or_else(|| v.to_string()))).collect();
                        out.insert(o.name.clone(), json!(lines));
                    }
                }
            }
        }
        Json::Object(out)
    }

    // ------------------------------------------------------------ interactive mode

    fn interactive(&mut self) -> Result<&mut Interactive, Json> {
        match &mut self.mode {
            Mode::Interactive(i) => Ok(i),
            _ => Err(json!("no interactive presentation is open")),
        }
    }

    fn outcome(r: Result<(), prismal_present::interact::Report>) -> Json {
        match r {
            Ok(()) => json!({ "ok": true }),
            Err(rep) => json!({ "ok": false, "why": format!("{:?}", rep.why).to_lowercase(), "message": rep.message }),
        }
    }

    /// Sets a control to a value in coherent SI units (the frame's `value`).
    pub fn set_control(&mut self, rep: &str, value: f64) -> Json {
        let i = match self.interactive() {
            Ok(i) => i,
            Err(e) => return json!({ "ok": false, "message": e }),
        };
        let ty = control_type(&i.cm, i.projector.views.iter().flat_map(|v| v.2.iter()), rep);
        let r = i.set_control(rep, literal(value, &ty));
        Self::outcome(r)
    }

    /// A key on a focused control or draggable representation: `left`, `right`, `up`, `down`.
    pub fn key(&mut self, rep: &str, key: &str) -> Json {
        let key = match key {
            "left" => Key::Left,
            "right" => Key::Right,
            "up" => Key::Up,
            _ => Key::Down,
        };
        match self.interactive() {
            Ok(i) => Self::outcome(i.key(rep, key)),
            Err(e) => json!({ "ok": false, "message": e }),
        }
    }

    /// Presses a button: requests its event at the instant shown (D-027).
    pub fn press(&mut self, rep: &str) -> Json {
        match self.interactive() {
            Ok(i) => Self::outcome(i.press(rep)),
            Err(e) => json!({ "ok": false, "message": e }),
        }
    }

    pub fn pointer_down(&mut self, rep: &str, part: Option<&str>) -> Json {
        match self.interactive() {
            Ok(i) => Self::outcome(i.pointer_down(rep, part)),
            Err(e) => json!({ "ok": false, "message": e }),
        }
    }

    /// Moves the pointer during a drag, in view coordinates. Returns whether the proposal
    /// is valid, with the runtime's message when it is not (PK-10.6).
    pub fn pointer_move(&mut self, x: f64, y: f64) -> Json {
        match self.interactive() {
            Ok(i) => {
                let valid = i.pointer_move([x, y]);
                json!({ "ok": valid, "message": i.preview().and_then(|p| p.message.clone()) })
            }
            Err(e) => json!({ "ok": false, "message": e }),
        }
    }

    pub fn pointer_up(&mut self) -> Json {
        match self.interactive() {
            Ok(i) => match i.pointer_up() {
                Ok(committed) => json!({ "ok": true, "committed": committed }),
                Err(r) => json!({ "ok": false, "message": r.message }),
            },
            Err(e) => json!({ "ok": false, "message": e }),
        }
    }

    pub fn cancel(&mut self) {
        if let Ok(i) = self.interactive() {
            i.cancel();
        }
    }

    pub fn undo(&mut self) {
        if let Ok(i) = self.interactive() {
            i.undo();
        }
    }

    pub fn redo(&mut self) {
        if let Ok(i) = self.interactive() {
            i.redo();
        }
    }

    // ------------------------------------------------------------ lesson mode

    fn lesson_input(&mut self, input: Input) -> Result<Json, Json> {
        let Mode::Lesson(l) = &self.mode else { return Err(json!("no lesson is open")) };
        let mut inputs = l.inputs.clone();
        inputs.push(input);
        let next = Lesson::replay(&self.prog, &l.pres, l.medium, inputs)?;
        self.mode = Mode::Lesson(Box::new(next));
        let Mode::Lesson(l) = &self.mode else { unreachable!() };
        Ok(self.lesson_info(l))
    }

    /// A learner's control setting at presentation instant `p` (an explore beat's control;
    /// refused outside explore beats, PK-9.8). Returns the recomputed lesson.
    pub fn lesson_set_control(&mut self, p: f64, rep: &str, value: f64) -> Result<Json, Json> {
        let Mode::Lesson(l) = &self.mode else { return Err(json!("no lesson is open")) };
        let shown: Vec<&CRep> = l.pb.shown.iter().map(|s| &s.rep).collect();
        let Some(CKind::Control { control, binding, idx, .. }) = shown.iter().find(|r| r.rep.id == rep).map(|r| &r.kind) else {
            return Err(json!(format!("no control `{rep}` in this lesson")));
        };
        let value = literal(value, &l.pb.cm.bindings[*idx].ty);
        let input = Input { at: p, input: LearnerInput::SetControl { control: control.clone(), binding: binding.clone(), value } };
        self.lesson_input(input)
    }

    /// The learner continues at presentation instant `p` (ends an explore beat).
    pub fn lesson_continue(&mut self, p: f64) -> Result<Json, Json> {
        self.lesson_input(Input { at: p, input: LearnerInput::Continue })
    }

    /// Plays the lesson again without the learner's inputs.
    pub fn lesson_restart(&mut self) -> Result<Json, Json> {
        let Mode::Lesson(l) = &self.mode else { return Err(json!("no lesson is open")) };
        let next = Lesson::replay(&self.prog, &l.pres, l.medium, vec![])?;
        self.mode = Mode::Lesson(Box::new(next));
        Ok(self.layout())
    }

}

/// A control's value as a literal: a Boolean for a toggle, else a number in coherent SI units.
fn literal(value: f64, ty: &prismal_ir::Type) -> prismal_ir::Expr {
    match ty {
        prismal_ir::Type::Boolean => prismal_ir::Expr::Bool { bool: value != 0.0 },
        _ => si_literal(value, ty),
    }
}

fn control_type<'a>(cm: &CModel, reps: impl Iterator<Item = &'a CRep>, rep: &str) -> prismal_ir::Type {
    for r in reps {
        if r.rep.id == rep || r.rep.name.as_deref() == Some(rep) {
            if let CKind::Control { idx, .. } = &r.kind {
                return cm.bindings[*idx].ty.clone();
            }
        }
    }
    prismal_ir::Type::real()
}


/// The extent `[xmin, ymin, xmax, ymax]` of shapes in view coordinates, including the origin.
fn extent<'a>(shapes: impl Iterator<Item = &'a Shape>) -> [f64; 4] {
    fn walk(s: &Shape, add: &mut impl FnMut(&[f64; 2])) {
        match s {
            Shape::Point { at } => add(at),
            Shape::Arrow { from, to } | Shape::Segment { from, to } => {
                add(from);
                add(to);
            }
            Shape::Polyline { points } | Shape::Polygon { points } => points.iter().for_each(add),
            Shape::Group { members } => members.iter().for_each(|m| walk(&m.shape, add)),
            _ => {}
        }
    }
    let mut e = [0.0f64, 0.0, 0.0, 0.0];
    let mut add = |p: &[f64; 2]| {
        if p[0].is_finite() && p[1].is_finite() {
            e[0] = e[0].min(p[0]);
            e[1] = e[1].min(p[1]);
            e[2] = e[2].max(p[0]);
            e[3] = e[3].max(p[1]);
        }
    };
    for s in shapes {
        walk(s, &mut add);
    }
    e
}

fn obs_type(cm: &CModel, o: &Observation) -> Option<prismal_ir::Type> {
    match &o.source {
        Source::Expr { expr } => compile_expr(cm, expr, None).ok().map(|x| x.1),
        _ => None,
    }
}

fn fmt_op(cm: &CModel, run: &prismal_runtime::Run, t: f64, op: &Op) -> String {
    match op {
        Op::Set { target, value } => {
            // A proposal is shown by its value: a constant, or evaluated on the state from
            // before the intervention at its instant (it may read other bindings).
            let v = match prismal_present::constant(cm, value) {
                Ok(v) => Some(v),
                Err(_) => run.committed.iter().find(|c| c.t == t).and_then(|c| compile_expr(cm, value, None).ok().and_then(|(ce, _)| run.eval_state(&ce, &c.vals, t).ok())),
            };
            let shown = match (v, cm.index.get(&target.binding)) {
                (Some(v), Some(&i)) => fmt_binding(cm, i, &v),
                _ => print(value, cm, &[]),
            };
            format!("set {} = {shown}", symbol(cm, &target.binding))
        }
        other => format!("{other:?}"),
    }
}

/// Observation data as text lines (PK-3.5).
fn fmt_data(cm: &CModel, run: &prismal_runtime::Run, o: &Observation, d: &Data) -> Vec<String> {
    let ty = obs_type(cm, o);
    let v = |x: &prismal_kernel::Value| ty.as_ref().map(|t| fmt_value(x, t)).unwrap_or_else(|| x.to_string());
    match d {
        Data::Value(x) => vec![v(x)],
        Data::Series(s) => s.iter().map(|(t, x)| format!("t = {} s: {}", fmt_num(*t), v(x))).collect(),
        Data::Events(es) => es.iter().map(|e| format!("{} at {} s", e.name, fmt_num(e.t))).collect(),
        Data::Diagnostics(ds) => ds.iter().map(|d| d.message.clone()).collect(),
        Data::Interventions(is) => is
            .iter()
            .map(|s| match &s.action {
                Action::Intervene(ops) => format!("at {} s: {}", fmt_num(s.t), ops.iter().map(|o| fmt_op(cm, run, s.t, o)).collect::<Vec<_>>().join("; ")),
                Action::Request(e) => format!("at {} s: request {}", fmt_num(s.t), e.rsplit('.').next().unwrap_or(e)),
            })
            .collect(),
    }
}
