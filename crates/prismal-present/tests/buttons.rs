//! Buttons in lessons and runtime-control buttons in labs (D-069): an explore beat's button
//! requests its event on the learner's branch; `button(reset)`, `button(undo)` and
//! `button(redo)` act on a lab's session.

mod common;
use common::*;
use prismal_ir::present::LearnerInput;
use prismal_present::interact::Interactive;
use prismal_present::timeline::{play, Input, Medium, Playback};
use prismal_present::Program;
use prismal_runtime::Config;

const SRC: &str = "
space Plane = euclidean(2)
model FreeFall in Plane {
  param {
    g: Acceleration = 9.81 m/s^2
    h: Length       = 10 m     in [1 m, 50 m]
  }
  state {
    pos: Point            = origin + (0 m, h)
    vel: Vector<Velocity> = 0
  }
  discrete { airborne: Boolean = true }
  flow {
    der(pos) = if airborne then vel else 0
    der(vel) = if airborne then (0, -g) else 0
  }
  event landed on falling(pos.y) {
    set airborne = false
    set vel = 0
  }
  event drop on request {
    set pos = origin + (0 m, h)
    set vel = 0
    set airborne = true
  }
}
presentation TryIt for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) { marker(pos) as ball }
  timeline {
    scene s {
      beat fall { run rate 1 until landed }
      beat play {
        explore limit 10 s keep h {
          slider(h, range: [1 m, 50 m])
          button(drop, label: \"Drop it\")
        }
      }
    }
  }
}
presentation Lab for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) { marker(pos) as ball }
  panel controls {
    button(drop)
    button(undo)
    button(redo)
    button(reset, label: \"Start again\")
  }
}
";

fn fall() -> f64 {
    (2.0 * 10.0 / 9.81_f64).sqrt()
}

fn lesson(inputs: Vec<Input>) -> Playback {
    play(&program(SRC), "TryIt", Config::until(20.0), Medium::Interactive, inputs).unwrap_or_else(|d| panic!("{d:?}"))
}

fn press(at: f64) -> Input {
    Input { at, input: LearnerInput::Press { event: "FreeFall.event.drop".into() } }
}

#[test]
fn an_explore_beats_button_requests_its_event_on_the_branch() {
    let at = fall() + 2.0;
    let pb = lesson(vec![press(at)]);
    assert!(pb.refusals.is_empty(), "{:?}", pb.refusals);
    let drops: Vec<_> = pb.announcements.iter().filter(|a| a.event == "drop").collect();
    assert_eq!(drops.len(), 1);
    assert!((drops[0].at - at).abs() < 1e-12);
    let branch = pb.lineage("branch 1").expect("the learner's branch");
    assert_eq!(branch.run.log.iter().filter(|l| l.name == "drop").count(), 1);
    // The lesson's own run is unchanged.
    assert_eq!(pb.lineage("lesson").unwrap().run.log.iter().filter(|l| l.name == "drop").count(), 0);
}

#[test]
fn presses_outside_explore_beats_or_of_buttons_not_offered_are_refused() {
    let pb = lesson(vec![press(0.5)]);
    assert_eq!(pb.refusals.len(), 1);
    assert!(pb.refusals[0].reason.contains("outside an explore beat"), "{:?}", pb.refusals);

    let src = SRC.replace("          button(drop, label: \"Drop it\")\n", "");
    let pb = play(&program(&src), "TryIt", Config::until(20.0), Medium::Interactive, vec![press(fall() + 1.0)]).unwrap();
    assert_eq!(pb.refusals.len(), 1);
    assert_eq!(pb.refusals[0].reason, "play offers no button for `drop` (PK-9.8)");
}

#[test]
fn runtime_control_buttons_act_on_a_lab() {
    let prog = program(SRC);
    let mut lab = Interactive::new(&prog, "Lab", Config::until(10.0)).unwrap_or_else(|d| panic!("{d:?}"));
    let drops = |i: &Interactive| i.session.current.log.iter().filter(|l| l.name == "drop").count();
    lab.seek(1.0);
    let id = |k: usize| lab.frame().rep(&format!("button.{k}")).unwrap().id.clone();
    let (drop, undo, redo, reset) = (id(1), id(2), id(3), id(4));
    lab.press(&drop).unwrap();
    assert_eq!(drops(&lab), 1, "the event button");
    lab.press(&undo).unwrap();
    assert_eq!(drops(&lab), 0, "undo");
    lab.press(&redo).unwrap();
    assert_eq!(drops(&lab), 1, "redo");
    lab.press(&reset).unwrap();
    assert_eq!(drops(&lab), 0, "reset");
    assert_eq!(lab.t, 0.0, "a reset shows t0");
    let f = lab.frame();
    let r = f.rep(&reset).unwrap();
    assert_eq!(r.text, "button Start again: reset");
}

#[test]
fn formatting_and_diagnostics() {
    let cases = "
run pressed of FreeFall with TryIt {
  learner {
    at 4 s: press drop
    at 5 s: continue
  }
}
";
    let src = format!("{SRC}{cases}");
    let doc = prismal_syntax::compile(&src).unwrap_or_else(|d| panic!("{d:?}")).doc;
    let printed = prismal_syntax::format::format(&doc);
    for line in ["at 4 s: press drop", "button(reset, label: \"Start again\")", "button(undo)"] {
        assert!(printed.contains(line), "{line} in\n{printed}");
    }
    assert_eq!(prismal_syntax::compile(&printed).unwrap().doc, doc);

    let codes = |edit: &str, with: &str| -> Vec<String> {
        let bad = SRC.replace(edit, with);
        assert_ne!(bad, SRC, "{edit}");
        match prismal_syntax::compile(&bad) {
            Err(ds) => ds.iter().map(|d| d.code.to_string()).collect(),
            Ok(c) => Program::new(c.doc).err().map(|ds| ds.iter().map(|d| d.code.to_string()).collect()).unwrap_or_default(),
        }
    };
    assert_eq!(codes("button(drop, label: \"Drop it\")", "button(reset)"), ["PK-E05"], "a runtime control in a lesson");
    assert_eq!(codes("    button(redo)\n", "    button(landed)\n"), ["PK-E03"], "not requestable");
    assert_eq!(codes("    button(redo)\n", "    button(rewind)\n"), ["SX-E03"], "unknown");
}
