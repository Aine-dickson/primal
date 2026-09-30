//! Static checks of a presentation against its model.
//!
//! | Code | Meaning |
//! |---|---|
//! | PK-E01 | broken reference: an element the model or presentation does not have (PK-2.3) |
//! | PK-E02 | an expression does not check in the model's scope (the kernel's code is quoted) |
//! | PK-E03 | a control, inverse, intervention or explore beat targets a binding that is not intervenable, or requests an event that is not requestable (D-023, D-027) |
//! | PK-E04 | an encoding without a declared scale of the right dimension (PK-5.5) |
//! | PK-E05 | a representation whose sources do not suit its kind or view (PK-6.3) |
//! | PK-E06 | a representation kind the prototype does not implement |

use crate::frame::{compile_rep, Projector, ViewCtx};
use crate::{number, PDiag};
use prismal_ir::present::{Action, Presentation, Schedule, Source};
use prismal_ir::{Expr, Trigger, Type};
use prismal_kernel::{check_intervention, compile_expr, CModel};

pub fn show_type(t: &Type) -> String {
    prismal_kernel::show(t)
}

fn expr(cm: &CModel, e: &Expr, want: Option<&Type>, element: &str, out: &mut Vec<PDiag>) {
    match compile_expr(cm, e, want) {
        Ok((_, t)) => {
            if let Some(w) = want {
                if &t != w {
                    out.push(PDiag { code: "PK-E02", message: format!("found {}, expected {}", show_type(&t), show_type(w)), element: element.into() });
                }
            }
        }
        Err(ds) => out.push(PDiag {
            code: "PK-E02",
            message: ds.iter().map(|d| format!("{}: {}", d.code, d.message)).collect::<Vec<_>>().join("; "),
            element: element.into(),
        }),
    }
}

fn event_exists(cm: &CModel, id: &str, element: &str, out: &mut Vec<PDiag>) -> bool {
    let ok = cm.ir.events.iter().any(|e| e.id == id);
    if !ok {
        out.push(PDiag { code: "PK-E01", message: format!("unknown event `{id}` (PK-2.3)"), element: element.into() });
    }
    ok
}

/// Checks a presentation against its model and returns every diagnostic.
pub fn check_presentation(cm: &CModel, p: &Presentation) -> Vec<PDiag> {
    let mut out = vec![];
    let time = Type::Instant { dim: prismal_ir::Dim::time() };
    let duration = Type::Quantity { dim: prismal_ir::Dim::time() };
    let projector = match Projector::new(cm, p) {
        Ok(pr) => Some(pr),
        Err(ds) => {
            out.extend(ds);
            None
        }
    };
    for o in &p.observations {
        match &o.source {
            Source::Expr { expr: e } => expr(cm, e, None, &o.id, &mut out),
            Source::EventLog { event: Some(ev), .. } => {
                event_exists(cm, ev, &o.id, &mut out);
            }
            Source::Diagnostics { element: Some(el) } => {
                let m = &cm.ir;
                let known = m.equations.iter().any(|x| &x.id == el)
                    || m.constraints.iter().any(|x| &x.id == el)
                    || m.events.iter().any(|x| &x.id == el)
                    || m.bindings.iter().any(|x| &x.id == el);
                if !known {
                    out.push(PDiag { code: "PK-E01", message: format!("unknown element `{el}` (PK-2.3)"), element: o.id.clone() });
                }
            }
            _ => {}
        }
        match &o.schedule {
            Schedule::Every { period } => expr(cm, period, Some(&duration), &o.id, &mut out),
            Schedule::At { time: t } => expr(cm, t, Some(&time), &o.id, &mut out),
            Schedule::On { event, .. } => {
                event_exists(cm, event, &o.id, &mut out);
            }
            Schedule::Over { from, to, .. } => {
                expr(cm, from, Some(&time), &o.id, &mut out);
                if let Some(t) = to {
                    expr(cm, t, Some(&time), &o.id, &mut out);
                }
            }
            Schedule::Live | Schedule::Run => {}
        }
    }
    let Some(projector) = projector else { return out };
    let mut rep_ids: Vec<String> = projector.views.iter().flat_map(|v| v.2.iter().map(|r| r.rep.id.clone())).collect();
    if let Some(tl) = &p.timeline {
        for b in tl.scenes.iter().flat_map(|s| &s.beats) {
            check_actions(cm, &projector, &b.actions, &b.id, &mut rep_ids, &mut out, &time, &duration);
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn check_actions(cm: &CModel, pr: &Projector, acts: &[Action], beat: &str, reps: &mut Vec<String>, out: &mut Vec<PDiag>, time: &Type, duration: &Type) {
    for a in acts {
        match a {
            Action::Reveal { view, reps: rs, duration: d, .. } => {
                if let Some(d) = d {
                    expr(cm, d, Some(duration), beat, out);
                }
                if let Some(v) = view {
                    if !pr.views.iter().any(|x| &x.0 == v) {
                        out.push(PDiag { code: "PK-E01", message: format!("unknown view `{v}`"), element: beat.into() });
                        continue;
                    }
                }
                let ctx = pr.ctx(view.as_deref());
                for r in rs {
                    if let Err(d) = compile_rep(cm, &ctx, r) {
                        out.extend(d);
                    }
                    reps.push(r.id.clone());
                }
            }
            Action::Camera { view, center, zoom, duration: d } => {
                match pr.views.iter().find(|x| &x.0 == view) {
                    Some((_, ViewCtx::Spatial { space, .. }, _)) => {
                        if let Some(c) = center {
                            expr(cm, c, Some(&Type::Point { space: space.clone() }), beat, out);
                        }
                    }
                    Some(_) => out.push(PDiag { code: "PK-E05", message: "a camera moves a spatial view (D-042)".into(), element: beat.into() }),
                    None => out.push(PDiag { code: "PK-E01", message: format!("unknown view `{view}`"), element: beat.into() }),
                }
                if let Some(z) = zoom {
                    expr(cm, z, Some(&Type::real()), beat, out);
                    if number(cm, z).is_ok_and(|z| z <= 0.0) {
                        out.push(PDiag { code: "PK-E02", message: "a zoom is positive".into(), element: beat.into() });
                    }
                }
                if let Some(d) = d {
                    expr(cm, d, Some(duration), beat, out);
                }
            }
            Action::Show { view, reps: rs } => {
                if let Some(v) = view {
                    if !pr.views.iter().any(|x| &x.0 == v) {
                        out.push(PDiag { code: "PK-E01", message: format!("unknown view `{v}`"), element: beat.into() });
                        continue;
                    }
                }
                let ctx = pr.ctx(view.as_deref());
                for r in rs {
                    if let Err(d) = compile_rep(cm, &ctx, r) {
                        out.extend(d);
                    }
                    reps.push(r.id.clone());
                }
            }
            Action::Highlight { target } | Action::Hide { target, .. } => {
                if !reps.contains(target) {
                    out.push(PDiag { code: "PK-E01", message: format!("unknown representation `{target}`"), element: beat.into() });
                }
                if let Action::Hide { duration: Some(d), .. } = a {
                    expr(cm, d, Some(duration), beat, out);
                }
            }
            Action::Narrate { duration: Some(d), .. } | Action::Wait { duration: d } => expr(cm, d, Some(duration), beat, out),
            Action::Narrate { .. } | Action::Hold | Action::Reset | Action::Branch => {}
            Action::Run { rate } => {
                expr(cm, rate, Some(&Type::real()), beat, out);
                if number(cm, rate).is_ok_and(|r| r < 0.0) {
                    out.push(PDiag { code: "PK-E02", message: "a playback rate is not negative (PK-8.2)".into(), element: beat.into() });
                }
            }
            Action::Seek { time: t } => expr(cm, t, Some(time), beat, out),
            Action::Intervene { ops } => {
                if let Err(ds) = check_intervention(cm, ops) {
                    out.extend(ds.into_iter().map(|d| PDiag { code: "PK-E03", message: format!("{}: {}", d.code, d.message), element: beat.into() }));
                }
            }
            Action::Request { event } => {
                if event_exists(cm, event, beat, out) && !matches!(cm.ir.events.iter().find(|e| &e.id == event).unwrap().trigger, Trigger::Request) {
                    out.push(PDiag { code: "PK-E03", message: format!("`{event}` is not declared `on request` (D-027)"), element: beat.into() });
                }
            }
            Action::WaitUntil { event } => {
                event_exists(cm, event, beat, out);
            }
            Action::Explore { limit, keep, controls, fallback } => {
                if let Some(l) = limit {
                    expr(cm, l, Some(duration), beat, out);
                }
                for k in keep {
                    match cm.index.get(k) {
                        Some(&i) if cm.bindings[i].intervenable => {}
                        _ => out.push(PDiag { code: "PK-E03", message: format!("`{k}` cannot be kept: not an intervenable binding (PK-9.9)"), element: beat.into() }),
                    }
                }
                for r in controls {
                    if let Err(d) = compile_rep(cm, &ViewCtx::Panel, r) {
                        out.extend(d);
                    }
                    reps.push(r.id.clone());
                }
                check_actions(cm, pr, fallback, beat, reps, out, time, duration);
            }
            Action::Sequence { actions } => check_actions(cm, pr, actions, beat, reps, out, time, duration),
        }
    }
}
