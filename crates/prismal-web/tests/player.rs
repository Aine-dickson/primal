//! The web player's logic on the reference programs, without a browser: what the renderer
//! receives (layout, frames, MathML) and what the learner's gestures do.

use prismal_web::{examples, Player};
use serde_json::Value;

fn load(key: &str) -> Player {
    let ex = examples::all().into_iter().find(|e| e.key == key).unwrap();
    Player::load(&ex.source).unwrap_or_else(|d| panic!("{key}: {d}"))
}

fn rep<'a>(frame: &'a Value, name: &str) -> &'a Value {
    let views = frame["views"].as_array().unwrap().iter().flat_map(|v| v["reps"].as_array().unwrap().iter());
    let overlay = frame["overlay"].as_array().into_iter().flatten();
    views
        .chain(overlay)
        .find(|r| r["name"] == name || r["kind"] == name || r["id"].as_str().unwrap().ends_with(&format!(".{name}")))
        .unwrap_or_else(|| panic!("no {name} in {frame}"))
}

#[test]
fn every_example_loads() {
    for ex in examples::all() {
        let p = Player::load(&ex.source).unwrap_or_else(|d| panic!("{}: {d}", ex.key));
        let c = p.catalogue();
        assert!(!c["presentations"].as_array().unwrap().is_empty(), "{}", ex.key);
    }
}

#[test]
fn cases_pass_in_the_player() {
    let mut n = 0;
    for key in ["rp01", "rp02", "rp03", "rp04", "rp05", "rp08"] {
        let p = load(key);
        for case in p.run_cases().as_array().unwrap() {
            assert!(case.get("error").is_none(), "{key}: {case}");
            for r in case["results"].as_array().unwrap() {
                assert_eq!(r["pass"], true, "{key} {}: {r}", case["case"]);
                n += 1;
            }
        }
    }
    assert_eq!(n, 36, "the expectations of prismal-present/tests/cases.rs");
}

#[test]
fn diagnostics_are_located() {
    let err = Player::load("model M {\n  param { g: Acceleration = 9.81 m/s^2 + }\n}\n").err().unwrap();
    let d = &err[0];
    assert_eq!(d["line"], 2, "{d}");
    // A kernel diagnostic is located at the declaration it names.
    let err = Player::load("model M {\n  param { a: Real = 1 }\n  derived { b: Length = a }\n}\n").err().unwrap();
    assert!(err[0]["code"].as_str().unwrap().starts_with("MK-E"), "{err}");
    assert_eq!(err[0]["line"], 3, "{err}");
}

#[test]
fn rp06_interactive() {
    let mut p = load("rp06");
    let layout = p.open("QuadraticPlot", false).unwrap();
    assert_eq!(layout["mode"], "interactive");
    assert_eq!(layout["views"][0]["kind"], "plot");
    assert_eq!(layout["views"][0]["x"], serde_json::json!([-3.0, 3.0]));

    let f = p.frame(0.0, 0.0);
    let formula = rep(&f, "formula");
    let ml = formula["mathml"].as_str().unwrap();
    assert!(ml.starts_with("<math display=\"block\">"), "{ml}");
    assert!(ml.contains("<msup><mi>x</mi><mn>2</mn></msup>"), "a x² typeset: {ml}");
    assert!(ml.contains("data-binding=\"QuadraticDemo.a\""), "{ml}");
    assert_eq!(rep(&f, "handle")["drag"], "body");
    let slider = rep(&f, "slider");
    assert_eq!(slider["value"], 1.0);
    assert_eq!(slider["symbol"], "a");

    // The slider, then a drag of the marker (RP-06 learner script, E2 and E5).
    let id = slider["id"].as_str().unwrap().to_string();
    assert_eq!(p.set_control(&id, 2.0)["ok"], true);
    assert_eq!(rep(&p.frame(0.0, 0.0), "handle")["at"], serde_json::json!([1.0, 2.0]));
    assert_eq!(p.pointer_down("handle", None)["ok"], true);
    assert_eq!(p.pointer_move(1.0, 3.0)["ok"], true);
    assert_eq!(rep(&p.frame(0.0, 0.0), "handle")["valid"], true, "preview marked valid");
    assert_eq!(p.pointer_up()["committed"], true);
    assert_eq!(rep(&p.frame(0.0, 0.0), "slider")["value"], 3.0);
    // Out of range: rejected by the runtime, previewed as invalid, not committed (E7).
    p.pointer_down("handle", None);
    let m = p.pointer_move(1.0, 9.0);
    assert_eq!(m["ok"], false);
    assert!(m["message"].as_str().is_some_and(|s| !s.is_empty()), "the runtime's reason: {m}");
    assert_eq!(p.pointer_up()["committed"], false);
    assert_eq!(rep(&p.frame(0.0, 0.0), "slider")["value"], 3.0);
    p.undo();
    assert_eq!(rep(&p.frame(0.0, 0.0), "slider")["value"], 2.0);
    let obs = p.observations();
    assert_eq!(obs["a_now"], serde_json::json!(["2"]));
    assert_eq!(obs["log"].as_array().unwrap().len(), 1);
}

#[test]
fn rp07_drag_arrow_head() {
    let mut p = load("rp07");
    let layout = p.open("VectorPlot", false).unwrap();
    let v = &layout["views"][0];
    assert_eq!((v["kind"].as_str(), v["px_per_m"].as_f64(), v["y_up"].as_bool()), (Some("spatial"), Some(40.0), Some(true)));
    // Extent in view coordinates: B = (4 m, 4 m) is at (160, -160) px.
    assert_eq!(v["extent"], serde_json::json!([0.0, -160.0, 160.0, 0.0]));
    let f = p.frame(0.0, 0.0);
    let u = f["views"][0]["reps"].as_array().unwrap().iter().find(|r| r["kind"] == "arrow").unwrap();
    let id = u["id"].as_str().unwrap().to_string();
    assert_eq!(u["drag"], "head");
    assert_eq!(p.pointer_down(&id, Some("head"))["ok"], true);
    // Head to (3 m, 3 m) + A: u = (2 m, 1 m).
    assert_eq!(p.pointer_move(120.0, -120.0)["ok"], true);
    assert_eq!(p.pointer_up()["committed"], true);
    let obs = p.observations();
    assert!(obs["values"][0].as_str().unwrap().starts_with("((2 m, 3 m)"), "{obs}");
    assert_eq!(p.pointer_down(&id, None)["ok"], false, "dragged by its head");
}

#[test]
fn rp08_lesson() {
    let mut p = load("rp08");
    let layout = p.open("ProjectileLesson", false).unwrap();
    assert_eq!(layout["mode"], "lesson");
    let lesson = &layout["lesson"];
    let explore = &lesson["explore"][0];
    assert_eq!(explore["beat"], "b7");
    let (start, end) = (explore["start"].as_f64().unwrap(), explore["end"].as_f64().unwrap());
    assert!((end - start - 60.0).abs() < 1e-9, "without inputs the beat lasts its limit");
    assert!(layout["permits"].as_array().unwrap().iter().any(|x| x == "zoom"));

    // A frame during b2: the ball is flying; b4 shows the live formula as MathML.
    let before = p.frame(5.0, 0.1);
    assert!(rep(&before, "ball")["at"][0].as_f64().unwrap() > 0.0);
    let b4 = lesson["beats"].as_array().unwrap().iter().find(|b| b["beat"] == "b4").unwrap()["start"].as_f64().unwrap();
    let f = p.frame(b4 + 0.5, 0.1);
    let ml = rep(&f, "formula")["mathml"].as_str().unwrap().to_string();
    assert!(ml.contains("<mfrac>") && ml.contains("<mi>sin</mi>"), "{ml}");
    assert!(!f["captions"].as_array().unwrap().is_empty());

    // The learner sets the angle in b7 and continues; the lesson is recomputed.
    let f7 = p.frame(start + 0.5, 0.1);
    let slider = rep(&f7, "slider");
    // The angle is shown in its display unit (MK-3.12, PK-11.1).
    assert_eq!(slider["display_unit"]["text"], "deg");
    assert_eq!(slider["text"], "slider for θ = 45 deg, from 10 deg to 80 deg");
    let id = slider["id"].as_str().unwrap().to_string();
    p.lesson_set_control(start + 1.0, &id, 60f64.to_radians()).unwrap();
    let info = p.lesson_continue(start + 2.0).unwrap();
    assert!(info["refusals"].as_array().unwrap().is_empty(), "{info}");
    let new_end = info["explore"][0]["end"].as_f64().unwrap();
    assert!((new_end - (start + 2.0)).abs() < 1e-9);
    // Frames before the first input are unchanged (PK-8.7).
    assert_eq!(p.frame(5.0, 0.1), before);
    // b8 relaunches at 60 degrees; the landing is RP-08's R60.
    let obs = p.observations();
    let last = obs["landings"].as_array().unwrap().last().unwrap().as_str().unwrap().to_string();
    assert!(last.contains("35.3119 m"), "{last}");

    // Outside an explore beat the input is refused (PK-9.8).
    let info = p.lesson_continue(1.0).unwrap();
    assert!(!info["refusals"].as_array().unwrap().is_empty());

    // Video medium: the fallback plays; no explore window.
    let layout = p.open("ProjectileLesson", true).unwrap();
    assert!(layout["lesson"]["explore"].as_array().unwrap().is_empty());
    assert_eq!(layout["lesson"]["medium"], "video");
}

#[test]
fn labs_play_in_the_player() {
    for (key, lab) in [("rp01", "ProjectileLab"), ("rp02", "ProjectileLab"), ("rp03", "BounceLab"), ("rp04", "PendulumLab"), ("rp05", "SpringLab")] {
        let mut p = load(key);
        let layout = p.open(lab, false).unwrap_or_else(|d| panic!("{key}: {d}"));
        let s = &layout["session"];
        assert_eq!(s["dynamic"], true, "{key}");
        assert_eq!(s["end"], 60.0, "{key}: {s}");
        assert_eq!(p.seek(1.5)["t"], 1.5);
        let f = p.frame(1.5, 0.5);
        assert_eq!(f["t"], 1.5);
        assert!(f["views"].as_array().unwrap().iter().flat_map(|v| v["reps"].as_array().unwrap()).any(|r| r["shape"] == "polyline"), "{key}");
    }
    // Plot axes carry their units; events passed while playing are announced.
    let mut p = load("rp03");
    let layout = p.open("BounceLab", false).unwrap();
    assert_eq!(layout["views"][1]["units"], serde_json::json!(["s", "m"]));
    p.seek(0.5);
    assert_eq!(p.frame(0.5, 0.1)["announcements"], serde_json::json!(["bounce"]), "first bounce at 0.4515 s");
    // A static model has no clock.
    let mut p = load("rp06");
    assert_eq!(p.open("QuadraticPlot", false).unwrap()["session"]["dynamic"], false);
}
