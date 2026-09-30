//! Raw input through the protocol (HI-4.5, D-047): the host forwards pointer, wheel and key
//! events in its own terms (pixels of a view as it drew it, key names) and the engine
//! targets them, runs drags, pans and zooms, and moves keyboard focus. No browser takes
//! part: these are the gestures of RP-06, RP-07, RP-08 and the guide's labs as any host
//! would forward them.

use prismal_host::Engine;
use serde_json::{json, Value as Json};

struct Host {
    engine: Engine,
    inst: String,
}

impl Host {
    fn open(key: &str, pres: &str) -> (Host, Json) {
        let src = prismal_web::examples::all().into_iter().find(|e| e.key == key).expect("example").source;
        let mut engine = Engine::new();
        let doc = ok(engine.handle(&json!({ "protocol": 1, "op": "load", "text": src }).to_string()));
        let opened = ok(engine.handle(&json!({ "protocol": 1, "op": "open", "document": doc["document"], "presentation": pres }).to_string()));
        (Host { engine, inst: opened["instance"].as_str().unwrap().into() }, opened["layout"].clone())
    }

    fn ask(&mut self, mut req: Json) -> Json {
        req["protocol"] = json!(1);
        req["instance"] = json!(self.inst);
        serde_json::from_str(&self.engine.handle(&req.to_string())).unwrap()
    }

    fn op(&mut self, req: Json) -> Json {
        let out = self.ask(req.clone());
        out.get("ok").cloned().unwrap_or_else(|| panic!("{req} failed: {out}"))
    }

    fn frame(&mut self, time: f64) -> Json {
        self.op(json!({ "op": "frame", "time": time }))
    }

    fn pointer(&mut self, phase: &str, view: &str, x: f64, y: f64, extra: Json) -> Json {
        let mut req = json!({ "op": "pointer", "phase": phase, "view": view, "x": x, "y": y });
        req.as_object_mut().unwrap().extend(extra.as_object().cloned().unwrap_or_default());
        self.op(req)
    }

    fn key(&mut self, key: &str, extra: Json) -> Json {
        let mut req = json!({ "op": "key_down", "key": key });
        req.as_object_mut().unwrap().extend(extra.as_object().cloned().unwrap_or_default());
        self.op(req)
    }
}

fn ok(out: String) -> Json {
    let v: Json = serde_json::from_str(&out).unwrap();
    v.get("ok").cloned().unwrap_or_else(|| panic!("{v}"))
}

fn rep<'a>(frame: &'a Json, suffix: &str) -> &'a Json {
    frame["views"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|v| v["reps"].as_array().unwrap().iter())
        .chain(frame["overlay"].as_array().into_iter().flatten())
        .find(|r| r["id"].as_str().unwrap().ends_with(suffix))
        .unwrap()
}

/// Pixels of a view position in a view drawn at `size`, from the frame's viewport, as a host
/// that draws the view would compute them.
fn px(view: &Json, p: [f64; 2], size: [f64; 2]) -> [f64; 2] {
    let b: Vec<f64> = view["box"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
    if let Some(m) = view["margin"].as_f64() {
        let (w, h) = (size[0], size[1]);
        return [m + (p[0] - b[0]) * (w - 2.0 * m) / b[2], h - m - (p[1] - b[1]) * (h - 2.0 * m) / b[3]];
    }
    let s = (size[0] / b[2]).min(size[1] / b[3]);
    let off = [(size[0] - b[2] * s) / 2.0, (size[1] - b[3] * s) / 2.0];
    [off[0] + (p[0] - b[0]) * s, off[1] + (p[1] - b[1]) * s]
}

fn pt(j: &Json) -> [f64; 2] {
    [j[0].as_f64().unwrap(), j[1].as_f64().unwrap()]
}

/// RP-07: grabbing the head of `u` and moving it, in a view drawn at its natural size and
/// at twice that size; a touch reaches farther than a mouse; the host learns what is under
/// the pointer.
#[test]
fn drag_an_arrow_head() {
    let (mut h, _) = Host::open("rp07", "VectorPlot");
    let f = h.frame(0.0);
    let view = f["views"][0].clone();
    assert_eq!(view["box"], json!([-80.0, -200.0, 320.0, 240.0]), "the view's framing from its extent");
    assert_eq!(view["size"], json!([320.0, 240.0]));
    let head = pt(&rep(&f, "arrow.1")["to"]);
    let natural = [320.0, 240.0];
    let [x, y] = px(&view, head, natural);

    let hover = h.pointer("move", "scene", x + 2.0, y, json!({}));
    assert_eq!(hover["handled"], false);
    assert_eq!(hover["hover"]["rep"], "VectorPlot.view.scene.arrow.1");
    assert_eq!(hover["hover"]["part"], "head");

    let down = h.pointer("down", "scene", x + 3.0, y - 2.0, json!({}));
    assert_eq!((down["handled"].clone(), down["action"].clone(), down["ok"].clone()), (json!(true), json!("drag"), json!(true)));
    assert_eq!(down["target"], json!({ "rep": "VectorPlot.view.scene.arrow.1", "part": "head" }));
    // One metre to the right is 40 view pixels.
    let moved = h.pointer("move", "scene", x + 40.0, y, json!({}));
    assert_eq!(moved["ok"], true);
    let up = h.pointer("up", "scene", x + 40.0, y, json!({}));
    assert_eq!(up["committed"], true);
    let f = h.frame(0.0);
    assert!(rep(&f, "arrow.1")["text"].as_str().unwrap().starts_with("arrow u = (4 m, 0 m)"), "{}", rep(&f, "arrow.1")["text"]);
    assert_eq!(f["focus"], "VectorPlot.view.scene.arrow.1", "the grabbed representation takes focus");

    // Drawn twice as large: the same gesture in the host's pixels.
    let head = pt(&rep(&f, "arrow.1")["to"]);
    let big = [640.0, 480.0];
    let [x, y] = px(&f["views"][0], head, big);
    let size = json!({ "width": 640, "height": 480 });
    assert_eq!(h.pointer("down", "scene", x, y, size.clone())["action"], "drag");
    h.pointer("move", "scene", x + 80.0, y, size.clone());
    assert_eq!(h.pointer("up", "scene", x + 80.0, y, size.clone())["committed"], true);
    assert!(rep(&h.frame(0.0), "arrow.1")["text"].as_str().unwrap().starts_with("arrow u = (5 m, 0 m)"));

    // 20 px below the head: out of a mouse's reach, within a finger's.
    let head = pt(&rep(&h.frame(0.0), "arrow.1")["to"]);
    let [x, y] = px(&h.frame(0.0)["views"][0], head, big);
    assert_eq!(h.pointer("down", "scene", x, y + 20.0, size.clone())["handled"], false, "a mouse misses");
    let touch = h.pointer("down", "scene", x, y + 20.0, json!({ "width": 640, "height": 480, "pointer": "touch" }));
    assert_eq!(touch["action"], "drag");
    assert_eq!(h.pointer("cancel", "scene", x, y, json!({}))["action"], "cancel");
    assert!(rep(&h.frame(0.0), "arrow.1")["text"].as_str().unwrap().starts_with("arrow u = (5 m, 0 m)"), "a cancelled drag commits nothing");

    // Empty space: nothing to grab, and the presentation permits no pan.
    assert_eq!(h.pointer("down", "scene", 5.0, 5.0, json!({})), json!({ "handled": false, "action": null }));
}

/// RP-06: the handle of a plot, grabbed in plot pixels; an invalid proposal is previewed as
/// invalid and the last valid one is committed (PK-10.6, PK-10.8).
#[test]
fn drag_in_a_plot() {
    let (mut h, _) = Host::open("rp06", "QuadraticPlot");
    let f = h.frame(0.0);
    let view = f["views"][0].clone();
    assert_eq!(view["margin"], 44.0);
    assert_eq!(view["box"], json!([-3.0, -5.0, 6.0, 25.0]));
    let size = [560.0, 347.0];
    let [x, y] = px(&view, pt(&rep(&f, "handle")["at"]), size);
    assert_eq!(h.pointer("down", "plot", x, y, json!({ "width": 560, "height": 347 }))["action"], "drag");
    let [x2, y2] = px(&view, [1.0, 3.0], size);
    assert_eq!(h.pointer("move", "plot", x2, y2, json!({}))["ok"], true);
    let [x3, y3] = px(&view, [1.0, 9.0], size);
    let bad = h.pointer("move", "plot", x3, y3, json!({}));
    assert_eq!(bad["ok"], false, "a = 9 is outside [-5, 5]");
    assert!(bad["message"].as_str().is_some());
    assert_eq!(h.pointer("up", "plot", x3, y3, json!({}))["committed"], true);
    let at = pt(&rep(&h.frame(0.0), "handle")["at"]);
    assert!((at[1] - 3.0).abs() < 1e-9, "the last valid proposal, a = 3: {at:?}");
}

/// RP-06: keyboard focus moves through the draggable handle and the slider, and arrow keys
/// step what has focus (PK-11.2); focus returns to the host past the end.
#[test]
fn keyboard_focus_and_steps() {
    let (mut h, _) = Host::open("rp06", "QuadraticPlot");
    assert_eq!(h.key("ArrowRight", json!({}))["handled"], false, "nothing has focus yet");
    let t = h.key("Tab", json!({}));
    assert_eq!((t["handled"].clone(), t["focus"].clone()), (json!(true), json!("QuadraticPlot.view.plot.handle")));
    assert_eq!(h.key("ArrowUp", json!({}))["ok"], true);
    let a = rep(&h.frame(0.0), "handle")["at"][1].as_f64().unwrap();
    assert!(a > 1.0, "the handle moved up: {a}");

    let t = h.key("Tab", json!({}));
    let slider = t["focus"].as_str().unwrap().to_string();
    assert!(slider.contains("slider"), "{slider}");
    assert_eq!(h.frame(0.0)["focus"], json!(slider));
    let before = rep(&h.frame(0.0), "slider.1")["value"].as_f64().unwrap();
    assert_eq!(h.key("ArrowRight", json!({}))["action"], "step");
    let after = rep(&h.frame(0.0), "slider.1")["value"].as_f64().unwrap();
    assert!((after - before - 0.1).abs() < 1e-9, "one declared step: {before} to {after}");
    assert_eq!(h.key("Enter", json!({}))["handled"], false, "Enter does nothing to a slider");

    let past = h.key("Tab", json!({}));
    assert_eq!((past["handled"].clone(), past["focus"].clone()), (json!(false), json!(null)), "focus returns to the host");
    assert!(h.frame(0.0).get("focus").is_none());
    assert_eq!(h.key("Tab", json!({ "shift": true }))["focus"], json!(slider), "backwards from outside: the last");
    assert_eq!(h.key(" ", json!({}))["handled"], false, "space is left to the host");

    // A host with its own focus system sets focus directly.
    assert_eq!(h.op(json!({ "op": "focus", "rep": "handle" }))["focus"], "QuadraticPlot.view.plot.handle");
    assert_eq!(h.op(json!({ "op": "focus", "rep": "nothing" }))["ok"], false);
}

/// The guide's labs: Enter presses a focused button (D-027) and flips a focused toggle.
#[test]
fn buttons_and_toggles_from_the_keyboard() {
    let (mut h, _) = Host::open("g8-freefall", "DropLab");
    let mut focus = h.key("Tab", json!({}));
    while !focus["focus"].as_str().unwrap_or("").contains("button") {
        focus = h.key("Tab", json!({}));
        assert_eq!(focus["handled"], true, "a button in the focus order");
    }
    let pressed = h.key("Enter", json!({}));
    assert_eq!((pressed["action"].clone(), pressed["ok"].clone()), (json!("press"), json!(true)));
    assert_eq!(h.op(json!({ "op": "undo" }))["interventions"], 0, "the press was one intervention");

    let (mut h, _) = Host::open("g7-cannon", "CannonLab");
    let mut focus = h.key("Tab", json!({}));
    while !focus["focus"].as_str().unwrap_or("").contains("toggle") {
        focus = h.key("Tab", json!({}));
        assert_eq!(focus["handled"], true, "a toggle in the focus order");
    }
    assert_eq!(rep(&h.frame(0.0), "toggle.1")["value"], 0.0);
    assert_eq!(h.key(" ", json!({}))["action"], "toggle");
    assert_eq!(rep(&h.frame(0.0), "toggle.1")["value"], 1.0);
    assert_eq!(h.key("ArrowRight", json!({}))["handled"], false, "arrows do not flip a toggle");
}

/// RP-08 permits zoom and pan: the wheel zooms about the pointer, a press on empty space
/// pans, Escape or cancel undoes a pan, and `view_reset` restores the view's framing. In a
/// lesson raw input carries its presentation instant, and no representation is dragged.
#[test]
fn zoom_and_pan_in_a_lesson() {
    let (mut h, layout) = Host::open("rp08", "ProjectileLesson");
    let missing = h.ask(json!({ "op": "wheel", "view": "scene", "x": 10, "y": 10, "delta": -100 }));
    assert_eq!(missing["error"][0]["code"], "HI-E01", "a lesson's raw input needs its instant");

    let box_at = |h: &mut Host| -> Vec<f64> { h.frame(1.0)["views"][0]["box"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect() };
    let own = box_at(&mut h);
    let size = h.frame(1.0)["views"][0]["size"].clone();
    let (w, hh) = (size[0].as_f64().unwrap(), size[1].as_f64().unwrap());
    let z = h.op(json!({ "op": "wheel", "view": "scene", "x": w / 2.0, "y": hh / 2.0, "delta": -462, "time": 1.0 }));
    assert_eq!(z["action"], "zoom");
    let zoomed = box_at(&mut h);
    assert!((zoomed[2] - own[2] / 2.0).abs() < 0.01 * own[2], "zoomed in about twice: {zoomed:?} from {own:?}");
    let centre = |b: &[f64]| [b[0] + b[2] / 2.0, b[1] + b[3] / 2.0];
    assert!((centre(&zoomed)[0] - centre(&own)[0]).abs() < 1e-6, "about the pointer at the centre");

    let t = json!({ "time": 1.0 });
    assert_eq!(h.pointer("down", "scene", 100.0, 100.0, t.clone())["action"], "pan");
    h.pointer("move", "scene", 150.0, 100.0, t.clone());
    let panned = box_at(&mut h);
    assert!((panned[0] - (zoomed[0] - 50.0 * zoomed[2] / w)).abs() < 1e-6, "50 px to the right moves the view left: {panned:?}");
    assert_eq!(h.key("Escape", t.clone())["action"], "cancel");
    assert_eq!(box_at(&mut h), zoomed, "cancel returns to where the pan began");

    assert_eq!(h.op(json!({ "op": "view_reset", "view": "scene" }))["action"], "view_reset");
    assert_eq!(box_at(&mut h), own);

    // The ball can be grabbed in a lab, not in a lesson.
    let f = h.frame(1.0);
    let ball = rep(&f, "ball");
    assert!(ball["drag"].is_null() || h.pointer("down", "scene", 0.0, 0.0, t.clone())["action"] != "drag");

    // Keyboard steps of the explore slider, refused outside the explore beat (PK-9.8).
    let explore = layout["lesson"]["explore"][0].clone();
    let inside = explore["start"].as_f64().unwrap() + 1.0;
    let mut focus = h.key("Tab", json!({ "time": inside }));
    while !focus["focus"].as_str().unwrap_or("").contains("slider") {
        focus = h.key("Tab", json!({ "time": inside }));
        assert_eq!(focus["handled"], true, "the explore slider in the focus order");
    }
    let step = h.key("ArrowRight", json!({ "time": inside }));
    assert_eq!((step["action"].clone(), step["ok"].clone()), (json!("step"), json!(true)), "{step}");
    assert_eq!(step["lesson"]["inputs"].as_array().unwrap().len(), 1);
}
