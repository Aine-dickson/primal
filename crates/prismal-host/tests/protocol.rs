//! The host interface through its JSON protocol (docs/spec/05-host-interface.md): documents
//! from text and from the IR, updates with identities kept, instances of both modes, the
//! interface's diagnostics, and closing without leaks.

use prismal_host::{Content, Engine};
use serde_json::{json, Value as Json};

const DROP: &str = "space Plane = euclidean(2)
model FreeFall in Plane {
  param { g: Acceleration = 9.81 m/s^2; h: Length = 10 m in [1 m, 50 m] }
  state { pos: Point = origin + (0 m, h); vel: Vector<Velocity> = 0 }
  discrete { airborne: Boolean = true }
  flow {
    der(pos) = if airborne then vel else 0
    der(vel) = if airborne then (0, -g) else 0
  }
  event landed on falling(pos.y) { set airborne = false; set vel = 0 }
  event drop on request { set pos = origin + (0 m, h); set vel = 0; set airborne = true }
}
presentation Lab for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) { marker(pos) as ball }
  panel controls { slider(h, range: [1 m, 50 m]) }
}
presentation Lesson for FreeFall {
  view scene: spatial(Plane, scale: 1 m -> 10 px, y: up) { marker(pos) as ball }
  timeline {
    scene s {
      beat fall   { run rate 1 until landed }
      beat choose { hold; explore limit 30 s keep h { slider(h, range: [1 m, 50 m]) } }
      beat again  { request drop; run rate 1 until landed }
    }
  }
}
";

/// Sends a request and answers `ok`, panicking on an error.
fn ok(e: &mut Engine, req: Json) -> Json {
    let out: Json = serde_json::from_str(&e.handle(&req.to_string())).unwrap();
    match out.get("ok") {
        Some(v) => v.clone(),
        None => panic!("{req}: {out}"),
    }
}

/// Sends a request and answers the codes of its error.
fn codes(e: &mut Engine, req: &str) -> Vec<String> {
    let out: Json = serde_json::from_str(&e.handle(req)).unwrap();
    out["error"].as_array().unwrap_or_else(|| panic!("{req}: no error in {out}")).iter().map(|d| d["code"].as_str().unwrap().to_string()).collect()
}

fn load(e: &mut Engine, text: &str) -> String {
    ok(e, json!({ "protocol": 1, "op": "load", "text": text }))["document"].as_str().unwrap().to_string()
}

#[test]
fn capabilities_and_malformed_requests() {
    let mut e = Engine::new();
    let c = ok(&mut e, json!({ "protocol": 1, "op": "capabilities" }));
    assert_eq!(c["protocol"], 1);
    assert_eq!(c["ir"]["version"], prismal_ir::VERSION);
    assert!(c["representations"].as_array().unwrap().iter().any(|k| k == "group"));
    assert_eq!(codes(&mut e, "not json"), ["HI-E01"]);
    assert_eq!(codes(&mut e, r#"{"op": "capabilities"}"#), ["HI-E01"], "no protocol version");
    assert_eq!(codes(&mut e, r#"{"protocol": 2, "op": "capabilities"}"#), ["HI-E01"], "unsupported version");
    assert_eq!(codes(&mut e, r#"{"protocol": 1, "op": "fly"}"#), ["HI-E01"], "no instance for an unknown op");
    assert_eq!(codes(&mut e, r#"{"protocol": 1, "op": "fly", "instance": "i9"}"#), ["HI-E02"]);
    assert_eq!(codes(&mut e, r#"{"protocol": 1, "op": "load"}"#), ["HI-E01"], "neither text nor ir");
    assert_eq!(codes(&mut e, r#"{"protocol": 1, "op": "text", "document": "d9"}"#), ["HI-E02"]);
}

#[test]
fn documents_from_text_and_ir() {
    let mut e = Engine::new();
    // A syntax error: the document is not loaded; diagnostics are located.
    let bad: Json = serde_json::from_str(&e.handle(&json!({ "protocol": 1, "op": "load", "text": "model M { param { g: Real = } }" }).to_string())).unwrap();
    assert_eq!(bad["error"][0]["span"]["line"], 1);
    assert_eq!(e.open_handles(), (0, 0));
    // A check error: loaded with its diagnostics, located and by identity (HI-3.1).
    let d = ok(&mut e, json!({ "protocol": 1, "op": "load", "text": "model M {\n  param { a: Real = 1 }\n  derived { b: Length = a }\n}\n" }));
    let diag = &d["diagnostics"][0];
    assert_eq!(diag["code"], "MK-E01");
    assert_eq!(diag["element"], "M.b");
    assert_eq!(diag["span"]["line"], 3);
    let h = d["document"].as_str().unwrap();
    assert_eq!(codes(&mut e, &json!({ "protocol": 1, "op": "run_cases", "document": h }).to_string()), ["HI-E03"]);

    // The same program from text and from its IR (HI-1.3): same catalogue, same frames.
    let t = load(&mut e, DROP);
    let ir = ok(&mut e, json!({ "protocol": 1, "op": "ir", "document": t }));
    let v = ok(&mut e, json!({ "protocol": 1, "op": "load", "ir": ir }));
    let i = v["document"].as_str().unwrap().to_string();
    assert_eq!(v["diagnostics"], json!([]));
    assert_eq!(v["catalogue"], ok(&mut e, json!({ "protocol": 1, "op": "catalogue", "document": t })));
    assert_eq!(ok(&mut e, json!({ "protocol": 1, "op": "text", "document": i })), ok(&mut e, json!({ "protocol": 1, "op": "text", "document": t })));
    let frame = |e: &mut Engine, doc: &str| {
        let inst = ok(e, json!({ "protocol": 1, "op": "open", "document": doc, "presentation": "Lab" }))["instance"].as_str().unwrap().to_string();
        ok(e, json!({ "protocol": 1, "op": "seek", "instance": inst, "time": 1.0 }));
        ok(e, json!({ "protocol": 1, "op": "frame", "instance": inst }))
    };
    assert_eq!(frame(&mut e, &t), frame(&mut e, &i));
    // Spans exist for text, not for the IR (IR-1.6).
    assert!(ok(&mut e, json!({ "protocol": 1, "op": "locate", "document": t, "element": "FreeFall.h" }))["line"].is_number());
    assert!(ok(&mut e, json!({ "protocol": 1, "op": "locate", "document": i, "element": "FreeFall.h" })).is_null());
}

#[test]
fn updates_keep_identities() {
    let mut e = Engine::new();
    let d = load(&mut e, DROP);
    // A tool rename keeps the identity (HI-3.3); references follow.
    let r = ok(&mut e, json!({ "protocol": 1, "op": "rename", "document": d, "element": "FreeFall.h", "name": "height" }));
    assert_eq!(r["diagnostics"], json!([]));
    let text = ok(&mut e, json!({ "protocol": 1, "op": "text", "document": d })).as_str().unwrap().to_string();
    assert!(text.contains("height:") && text.contains("origin + (0 m, height)"), "{text}");
    // The host edits that text and sends it back: `height` keeps the identity it had.
    let edited = text.replace("9.81 m/s^2", "1.62 m/s^2");
    let u = ok(&mut e, json!({ "protocol": 1, "op": "update", "document": d, "text": edited }));
    assert_eq!(u["removed"], json!([]));
    let ir = ok(&mut e, json!({ "protocol": 1, "op": "ir", "document": d }));
    let param = |ir: &Json, name: &str| ir["models"][0]["bindings"].as_array().unwrap().iter().find(|b| b["name"] == name).cloned();
    assert_eq!(param(&ir, "height").unwrap()["id"], "FreeFall.h");
    // Spans follow the identities `reconcile` restored.
    let at = ok(&mut e, json!({ "protocol": 1, "op": "locate", "document": d, "element": "FreeFall.h" }));
    assert_eq!(&edited[at["start"].as_u64().unwrap() as usize..][..6], "height");
    // A plain-text rename that leaves references broken does not compile: the update is
    // refused with located diagnostics and the document is unchanged (HI-3.2).
    let broken = edited.replace("height:", "hh:");
    let out: Json = serde_json::from_str(&e.handle(&json!({ "protocol": 1, "op": "update", "document": d, "text": broken }).to_string())).unwrap();
    assert!(out["error"].as_array().unwrap().iter().all(|d| d["code"] == "SX-E03" && d["span"]["line"].is_number()), "{out}");
    assert_eq!(ok(&mut e, json!({ "protocol": 1, "op": "ir", "document": d })), ir);
    // Removing elements reports the identities that disappeared.
    let cut = &edited[..edited.find("presentation Lesson").unwrap()];
    let u = ok(&mut e, json!({ "protocol": 1, "op": "update", "document": d, "text": cut }));
    let removed: Vec<&str> = u["removed"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
    assert!(removed.contains(&"Lesson") && removed.iter().all(|x| x.starts_with("Lesson")), "{removed:?}");
}

#[test]
fn instances_of_both_modes() {
    let mut e = Engine::new();
    let d = load(&mut e, DROP);
    // An interactive session: the host's clock, controls, undo.
    let o = ok(&mut e, json!({ "protocol": 1, "op": "open", "document": d, "presentation": "Lab" }));
    let lab = o["instance"].as_str().unwrap().to_string();
    assert_eq!(o["layout"]["mode"], "interactive");
    let clock = ok(&mut e, json!({ "protocol": 1, "op": "seek", "instance": lab, "time": 0.5 }));
    assert_eq!(clock["t"], 0.5);
    let slider = ok(&mut e, json!({ "protocol": 1, "op": "frame", "instance": lab }))["views"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|v| v["reps"].as_array().unwrap().clone())
        .find(|r| r["kind"] == "slider")
        .unwrap()["id"]
        .clone();
    let r = ok(&mut e, json!({ "protocol": 1, "op": "set_control", "instance": lab, "rep": slider, "value": 20.0 }));
    assert_eq!(r["ok"], true);
    assert_eq!(ok(&mut e, json!({ "protocol": 1, "op": "undo", "instance": lab }))["interventions"], 0);
    assert_eq!(codes(&mut e, &json!({ "protocol": 1, "op": "continue", "instance": lab, "time": 1.0 }).to_string()), ["HI-E03"]);

    // A lesson: frames at presentation instants; inputs carry their instant (HI-4.3).
    let o = ok(&mut e, json!({ "protocol": 1, "op": "open", "document": d, "presentation": "Lesson" }));
    let les = o["instance"].as_str().unwrap().to_string();
    let beats = o["layout"]["lesson"]["beats"].as_array().unwrap().clone();
    let choose = beats.iter().find(|b| b["beat"] == "choose").unwrap()["start"].as_f64().unwrap();
    let f = ok(&mut e, json!({ "protocol": 1, "op": "frame", "instance": les, "time": choose + 1.0 }));
    let control = f["overlay"].as_array().unwrap().iter().find(|r| r["kind"] == "slider").unwrap()["id"].clone();
    // Refused before the explore beat; applied in it (D-025).
    let early = ok(&mut e, json!({ "protocol": 1, "op": "set_control", "instance": les, "rep": control, "value": 20.0, "time": 0.5 }));
    assert_eq!(early["refusals"].as_array().unwrap().len(), 1);
    ok(&mut e, json!({ "protocol": 1, "op": "restart", "instance": les }));
    ok(&mut e, json!({ "protocol": 1, "op": "set_control", "instance": les, "rep": control, "value": 20.0, "time": choose + 1.0 }));
    let info = ok(&mut e, json!({ "protocol": 1, "op": "continue", "instance": les, "time": choose + 2.0 }));
    let again = info["beats"].as_array().unwrap().iter().find(|b| b["beat"] == "again").unwrap().clone();
    let fall = (2.0 * 20.0 / 9.81f64).sqrt();
    assert!((again["end"].as_f64().unwrap() - again["start"].as_f64().unwrap() - fall).abs() < 1e-6, "dropped from 20 m: {again}");
    assert_eq!(codes(&mut e, &json!({ "protocol": 1, "op": "press", "instance": les, "rep": "x" }).to_string()), ["HI-E03"]);
    assert_eq!(codes(&mut e, &json!({ "protocol": 1, "op": "open", "document": d, "presentation": "Nope" }).to_string()), ["HI-E02"]);
}

/// A host supplies values of the model's inputs from its environment (HI-4.3a, D-051), and
/// events carry payloads into the event log (D-050).
#[test]
fn environment_inputs_and_payloads() {
    const CART: &str = "model Cart {
  const { m: Mass = 2 kg }
  input { thrust: Force = 0 N }
  state { x: Length = 0 m; v: Velocity = 0 m/s }
  discrete { pushes: Real = 0 }
  flow { der(x) = v; der(v) = thrust / m }
  event pushed on input(thrust) { set pushes = pushes + 1 }
  event kick on request(j: Momentum) { set v = v + j / m }
}
presentation Lab for Cart {
  view track: plot(x: [0 s, 4 s], y: [0 m/s, 10 m/s]) { series_plot(v every 0.1 s) }
  panel status { label(pushes) }
  observe { log = event_log over [t0, t0 + 4 s] }
}
";
    let mut e = Engine::new();
    let d = load(&mut e, CART);
    let lab = ok(&mut e, json!({ "protocol": 1, "op": "open", "document": d, "presentation": "Lab" }))["instance"].as_str().unwrap().to_string();
    let caps = ok(&mut e, json!({ "protocol": 1, "op": "capabilities" }));
    assert_eq!(caps["input"]["environment"], json!(["set_input"]));
    ok(&mut e, json!({ "protocol": 1, "op": "seek", "instance": lab, "time": 1.0 }));
    // 4 N on 2 kg from 1 s: 2 m/s after 1 s more.
    let r = ok(&mut e, json!({ "protocol": 1, "op": "set_input", "instance": lab, "input": "thrust", "value": 4.0 }));
    assert_eq!(r["ok"], true, "{r}");
    ok(&mut e, json!({ "protocol": 1, "op": "seek", "instance": lab, "time": 2.0 }));
    let f = ok(&mut e, json!({ "protocol": 1, "op": "frame", "instance": lab }));
    let text = f.to_string();
    assert!(text.contains("pushes = 1"), "{text}");
    assert!(text.contains("last 2 at 2 s") || text.contains("last 2 m/s"), "v = 2 m/s at 2 s: {text}");
    // Only inputs take environment values; the value has the input's type.
    let r = ok(&mut e, json!({ "protocol": 1, "op": "set_input", "instance": lab, "input": "x", "value": 1.0 }));
    assert_eq!(r["ok"], false, "{r}");
    let obs = ok(&mut e, json!({ "protocol": 1, "op": "observations", "instance": lab }));
    assert!(obs["log"].to_string().contains("pushed at 1 s"), "{obs}");
    // A request with its payload, in coherent SI units (HI-4.3b).
    let r = ok(&mut e, json!({ "protocol": 1, "op": "request", "instance": lab, "event": "kick", "payload": 2.0 }));
    assert_eq!(r["ok"], true, "{r}");
    let r = ok(&mut e, json!({ "protocol": 1, "op": "request", "instance": lab, "event": "kick" }));
    assert_eq!(r["ok"], false, "a request of `kick` supplies its payload: {r}");
    let obs = ok(&mut e, json!({ "protocol": 1, "op": "observations", "instance": lab }));
    assert!(obs["log"].to_string().contains("kick(2 kg m/s) at 2 s"), "{obs}");
}

/// A request names a member by its collection and number (D-059).
#[test]
fn requests_name_members() {
    const TABLE: &str = "space Plane = euclidean(2)
model Table in Plane {
  object Ball {
    state { pos: Point = origin; vel: Vector<Velocity> = 0 }
    flow { der(pos) = vel }
  }
  parts { balls: Ball[3, max 3] { pos = origin + (index * 1 m, 0 m) } }
  event remove on request(b in balls) { destroy b }
  event kick on request(b in balls, j: Vector<Momentum>) { set b.vel = b.vel + j / 1 kg }
  derived { n: Real = count(balls) }
}
presentation Lab for Table {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) { for b in balls { marker(b.pos) as ball } }
  panel status { label(n) }
  observe { log = event_log over [t0, t0 + 4 s] }
}
";
    let mut e = Engine::new();
    let d = load(&mut e, TABLE);
    let lab = ok(&mut e, json!({ "protocol": 1, "op": "open", "document": d, "presentation": "Lab" }))["instance"].as_str().unwrap().to_string();
    let req = |e: &mut Engine, event: &str, payload: Json| ok(e, json!({ "protocol": 1, "op": "request", "instance": lab, "event": event, "payload": payload }));
    assert_eq!(req(&mut e, "remove", json!("balls[2]"))["ok"], true);
    let r = req(&mut e, "remove", json!("balls[2]"));
    assert_eq!(r["ok"], false);
    assert!(r["message"].as_str().unwrap().contains("`balls[2]` is not alive"), "{r}");
    let r = req(&mut e, "remove", json!("rocks[1]"));
    assert_eq!(r["why"], "refused", "{r}");
    assert_eq!(req(&mut e, "kick", json!(["balls[3]", [0.0, 2.0]]))["ok"], true);
    let obs = ok(&mut e, json!({ "protocol": 1, "op": "observations", "instance": lab })).to_string();
    assert!(obs.contains("remove(balls[2]) at 0 s"), "{obs}");
    assert!(obs.contains("kick(balls[3], (0 kg m/s, 2 kg m/s)) at 0 s"), "{obs}");
    let f = ok(&mut e, json!({ "protocol": 1, "op": "frame", "instance": lab })).to_string();
    assert!(f.contains("n = 2"), "{f}");
}

#[test]
fn closing_frees_documents_and_instances() {
    let mut e = Engine::new();
    let d = e.load(Content::Text(DROP.into())).unwrap();
    let weak = std::rc::Rc::downgrade(&e.document(&d).unwrap().program().unwrap());
    let (i, _) = e.open(&d, "Lab", false).unwrap();
    // Instances keep the program alive across updates of their document (HI-2.2).
    e.document_mut(&d).unwrap().update(Content::Text(DROP.replace("9.81", "9.8"))).unwrap();
    assert!(weak.upgrade().is_some(), "held by the open instance");
    e.close(&i).unwrap();
    assert!(weak.upgrade().is_none(), "freed with its instance");
    // Closing a document closes its instances; repeated loads leave nothing behind.
    for _ in 0..50 {
        let d = e.load(Content::Text(DROP.into())).unwrap();
        e.open(&d, "Lesson", false).unwrap();
        e.close(&d).unwrap();
    }
    assert_eq!(e.open_handles(), (1, 0));
    assert!(e.close("d1").is_ok() && e.close("d1").is_err());
}
