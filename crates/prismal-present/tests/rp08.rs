//! RP-08 Narrated projectile lesson: the explanation timeline over the lesson run
//! (docs/spec/reference-programs/RP-08-narrated-lesson.md, E1 to E16).

mod common;
use common::*;
use prismal_ir::build::num;
use prismal_ir::present::LearnerInput;
use prismal_present::timeline::{play, Input, Medium, Playback};
use prismal_present::{flat, Program, LESSON_HORIZON};
use prismal_runtime::{Action, Config};

const T45: f64 = 2.88320807823261;
const T60: f64 = 3.53119430697019;
const R45: f64 = 40.7747196738022;
const R60: f64 = 35.3119430697019;
const ANGLE: &str = "Projectile.angle";

fn close(got: f64, want: f64, what: &str) {
    assert!((got - want).abs() <= 1e-8, "{what}: got {got:.15}, expected {want:.15}");
}

fn script(extra: &[Input]) -> Vec<Input> {
    let mut v = vec![
        Input { at: 23.0, input: LearnerInput::SetControl { control: "slider".into(), binding: ANGLE.into(), value: num(60.0, "deg") } },
        Input { at: 25.0, input: LearnerInput::Continue },
    ];
    v.extend(extra.iter().cloned());
    v
}

fn lesson<'a>(prog: &'a Program, medium: Medium, inputs: Vec<Input>) -> Playback<'a> {
    let pb = play(prog, "ProjectileLesson", Config::until(LESSON_HORIZON), medium, inputs).unwrap();
    assert!(pb.diagnostics.is_empty(), "{:?}", pb.diagnostics);
    pb
}

fn angle_interventions(pb: &Playback, lineage: &str) -> usize {
    pb.lineage(lineage)
        .unwrap()
        .config
        .log
        .iter()
        .filter(|s| matches!(&s.action, Action::Intervene(ops) if ops.iter().any(|o| matches!(o, prismal_ir::Op::Set { target, .. } if target.binding == ANGLE))))
        .count()
}

#[test]
fn case_a_keep() {
    let prog = program(&rp08());
    let pb = lesson(&prog, Medium::Interactive, script(&[]));
    // E1, E2
    close(pb.beat("b2").start, 4.0, "RP-08.E1 b2");
    close(pb.beat("b3").start, 4.0 + T45, "RP-08.E1 b3");
    close(pb.beat("b4").start, 7.0 + T45, "RP-08.E1 b4");
    close(pb.beat("b5").start, 10.0 + T45, "RP-08.E1 b5");
    close(pb.beat("b5").end, 18.6496242346978, "RP-08.E2");
    // E3: the landings seen in b2 and in b5 are (T45, R45) and bit-identical (RC-12.2).
    let landings = pb.observed("landings");
    assert_eq!(landings.len(), 3, "landings in b2, b5 and b8");
    close(landings[0].0, pb.beat("b2").end, "landing ends b2");
    close(landings[1].0, pb.beat("b5").end, "landing ends b5");
    let (l2, l5) = (flat(&landings[0].1), flat(&landings[1].1));
    assert!(l2.iter().zip(&l5).all(|(a, b)| a.to_bits() == b.to_bits()), "RP-08.E3 bit-identical");
    close(l2[0], T45, "RP-08.E3 time");
    close(l2[1], R45, "RP-08.E3 range");
    // E4
    assert_eq!(pb.beat("b7").end, 25.0, "RP-08.E4");
    // E5: b8 runs on the learner's branch at 60 degrees.
    let l8 = flat(&landings[2].1);
    close(l8[0], T45 + T60, "RP-08.E5 landing");
    close(l8[1], R60, "RP-08.E5 range");
    assert_eq!(pb.runs[pb.segment_at(pb.beat("b8").start + 1.0).run].lineage, "branch 1", "RP-08.E5 on the branch");
    // E6
    close(pb.beat("b8").end, 28.5311943069702, "RP-08.E6 b8");
    close(pb.end, 31.5311943069702, "RP-08.E6 end");
    // E7: the lesson run's own log has no intervention on `angle`.
    assert_eq!(angle_interventions(&pb, "lesson"), 0, "RP-08.E7");
    assert_eq!(angle_interventions(&pb, "branch 1"), 1);
    // E8: a caption for every narration, shown for its duration; `landed` announced at
    // each occurrence.
    let caps: Vec<(&str, f64, f64)> = pb.captions.iter().map(|c| (c.text.as_str(), c.start, c.end - c.start)).collect();
    assert_eq!(caps.len(), 5, "RP-08.E8 five narrations");
    for (b, (_, start, d)) in ["b1", "b3", "b4", "b6", "b9"].iter().zip(&caps) {
        close(*start, pb.beat(b).start, "RP-08.E8 caption start");
        assert!((d - 4.0).abs() < 1e-9 || (d - 3.0).abs() < 1e-9, "caption duration {d}");
        let mid = pb.frame(start + d / 2.0, 0.04);
        assert_eq!(mid.captions.len(), 1, "RP-08.E8 caption shown at {}", start + d / 2.0);
    }
    let landed: Vec<f64> = pb.announcements.iter().filter(|a| a.event == "landed").map(|a| a.at).collect();
    assert_eq!(landed.len(), 3, "RP-08.E8 announcements");
    for at in landed {
        assert!(pb.frame(at, 1.0 / 30.0).announcements.contains(&"landed".to_string()));
    }
    // Frames: the ball is shown from b1, highlighted in b3, at the landing point there.
    let f = pb.frame(pb.beat("b3").start + 1.0, 0.04);
    let ball = f.rep("ball").unwrap();
    assert!(ball.highlighted, "b3 highlights the ball");
    match ball.shape {
        prismal_present::frame::Shape::Point { at } => close(at[0], R45 * 10.0, "ball at the landing point"),
        ref s => panic!("{s:?}"),
    }
    assert!(!pb.frame(pb.beat("b4").start + 1.0, 0.04).rep("ball").unwrap().highlighted);
    // b4: after the seek the formula is shown with live values and display symbols.
    let f = pb.frame(pb.beat("b4").start + 0.5, 0.04);
    assert_eq!(f.t, 0.0, "b4 seeks to t0 (D-033)");
    let formula = f.overlay.iter().find(|r| r.kind == "formula").unwrap();
    assert!(formula.text.starts_with("R = v^2 sin(2 θ) / g"), "{}", formula.text);
    // The explore controls are shown only during b7.
    let during = pb.frame(24.0, 0.04);
    assert!(during.overlay.iter().any(|r| r.kind == "slider"));
    assert!(!pb.frame(26.0, 0.04).overlay.iter().any(|r| r.kind == "slider"));
}

#[test]
fn case_b_no_keep() {
    let prog = program(&rp08().replacen("explore limit 60 s keep angle {", "explore limit 60 s {", 1));
    let pb = lesson(&prog, Medium::Interactive, script(&[]));
    // E9: b8 runs on the lesson run at 45 degrees.
    let landings = pb.observed("landings");
    let l8 = flat(&landings[2].1);
    close(l8[0], 2.0 * T45, "RP-08.E9 landing");
    close(l8[1], R45, "RP-08.E9 range");
    assert_eq!(pb.runs[pb.segment_at(pb.beat("b8").start + 1.0).run].lineage, "lesson");
    // E10
    close(pb.beat("b8").end, 27.8832080782326, "RP-08.E10 b8");
    close(pb.end, 30.8832080782326, "RP-08.E10 end");
    // E11: the learner's branch is kept, with its own log.
    assert_eq!(angle_interventions(&pb, "branch 1"), 1, "RP-08.E11");
    assert_eq!(angle_interventions(&pb, "lesson"), 0);
}

#[test]
fn case_c_video() {
    let prog = program(&rp08());
    let pb = lesson(&prog, Medium::Video, vec![]);
    // E12: the fallback sets the angle on the lesson run; the beat lasts 3 s.
    close(pb.beat("b7").end - pb.beat("b7").start, 3.0, "RP-08.E12 duration");
    assert_eq!(angle_interventions(&pb, "lesson"), 1, "RP-08.E12 on the lesson run");
    assert!(pb.lineage("branch 1").is_none());
    // E13
    close(pb.beat("b7").end, 24.6496242346978, "RP-08.E13 b7");
    close(pb.beat("b8").end, 28.1808185416680, "RP-08.E13 b8");
    close(pb.end, 31.1808185416680, "RP-08.E13 end");
    // E14: two exports give identical frame sequences (PK-8.7).
    let a = serde_json::to_string(&pb.export(30.0)).unwrap();
    let pb2 = lesson(&prog, Medium::Video, vec![]);
    let b = serde_json::to_string(&pb2.export(30.0)).unwrap();
    assert_eq!(a, b, "RP-08.E14");
    assert_eq!(pb.export(30.0).len(), (pb.end * 30.0).floor() as usize + 1);
    // E15: nothing unsupported without a fallback.
    assert!(pb.unsupported.is_empty(), "RP-08.E15 {:?}", pb.unsupported);
    // Without its fallback, the explore beat is reported, not dropped silently (PK-12.3).
    let bare = program(&rp08().replacen(
        "} fallback {\n          sequence { intervene { set angle = 60 deg }; wait 3 s }\n        }",
        "}",
        1,
    ));
    let pb3 = play(&bare, "ProjectileLesson", Config::until(LESSON_HORIZON), Medium::Video, vec![]).unwrap();
    assert_eq!(pb3.unsupported.len(), 1, "PK-12.3");
}

#[test]
fn case_d_restricted() {
    let prog = program(&rp08());
    let extra = Input { at: 10.0, input: LearnerInput::SetControl { control: "slider".into(), binding: ANGLE.into(), value: num(30.0, "deg") } };
    let pb = lesson(&prog, Medium::Interactive, script(&[extra]));
    // E16: refused, the lesson run unaffected, timing as in case A.
    assert_eq!(pb.refusals.len(), 1, "RP-08.E16 {:?}", pb.refusals);
    assert_eq!(pb.refusals[0].at, 10.0);
    assert!(pb.refusals[0].reason.contains("PK-9.8"));
    assert_eq!(angle_interventions(&pb, "lesson"), 0);
    close(pb.beat("b8").end, 28.5311943069702, "RP-08.E16 timing");
    close(pb.end, 31.5311943069702, "RP-08.E16 timing");
}
