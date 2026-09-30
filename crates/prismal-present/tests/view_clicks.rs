//! Clicks on a point of a view (D-060): `on click as p request E(p)` requests an event with
//! the point clicked; frames say what a click does; printing and diagnostics.

mod common;
use common::*;
use prismal_present::interact::Interactive;
use prismal_present::Program;
use prismal_runtime::Config;

const LAB: &str = "
space Plane = euclidean(2)

model Lab in Plane {
  object Ball {
    state {
      pos: Point = origin
    }
  }
  parts {
    balls: Ball[1, max 3]
  }
  event add on request(q: Point) { create balls { pos = q } }
  derived {
    n: Real    = count(balls)
    x2: Length = balls[2].pos.x
    y2: Length = balls[2].pos.y
  }
}

presentation Table for Lab {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    on click as p request add(p)
    for b in balls {
      marker(b.pos) as ball
    }
  }
  panel side {
    label(n)
  }
}
";

fn value(i: &Interactive, name: &str) -> f64 {
    let k = i.cm.index[&format!("Lab.{name}")];
    i.session.current.state_at(i.t)[k].num()
}

fn session(src: &str) -> Interactive {
    let prog = program(src);
    Interactive::new(&prog, "Table", Config::until(2.0)).unwrap_or_else(|d| panic!("{d:?}"))
}

#[test]
fn a_click_on_a_point_requests_with_the_point() {
    let mut i = session(LAB);
    let f = i.frame();
    assert_eq!(f.views[0].click.as_deref(), Some("add"));
    assert!(f.views[1].click.is_none());
    assert_eq!(value(&i, "n"), 1.0);
    // View coordinates are pixels from the space's origin, y down on the screen: with
    // `y: up`, (100, -50) px is (2 m, 1 m).
    i.seek(0.5);
    i.click_at("scene", [100.0, -50.0]).unwrap();
    assert_eq!(value(&i, "n"), 2.0);
    assert!((value(&i, "x2") - 2.0).abs() < 1e-12, "{}", value(&i, "x2"));
    assert!((value(&i, "y2") - 1.0).abs() < 1e-12, "{}", value(&i, "y2"));
    assert!(i.frame().rep("ball[2]").is_some());
    i.click_at("scene", [0.0, 0.0]).unwrap();
    assert_eq!(value(&i, "n"), 3.0);
    // A view without a click, and an unknown view, refuse.
    let e = i.click_at("side", [0.0, 0.0]).unwrap_err();
    assert!(e.message.contains("does nothing"), "{}", e.message);
    assert!(i.click_at("nowhere", [0.0, 0.0]).is_err());
    // Undone, the member made is gone.
    i.undo();
    assert_eq!(value(&i, "n"), 2.0);
    assert_eq!(i.session.log().len(), 1);
}

#[test]
fn printing_round_trip() {
    let doc = prismal_syntax::compile(LAB).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert!(printed.contains("{\n    on click as p request add(p)\n    for b in balls"), "{printed}");
    assert_eq!(prismal_syntax::compile(&printed).unwrap_or_else(|d| panic!("{d:?}\n{printed}")).doc, doc, "{printed}");
    // A payload that computes from the point.
    let shifted = LAB.replace("request add(p)", "request add(p + (0 m, 1 m))");
    let doc = prismal_syntax::compile(&shifted).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert!(printed.contains("on click as p request add(p + (0 m, 1 m))"), "{printed}");
    assert_eq!(prismal_syntax::compile(&printed).unwrap().doc, doc);
}

#[test]
fn diagnostics() {
    let codes = |edit: &str, with: &str| -> Vec<String> {
        let bad = LAB.replace(edit, with);
        assert_ne!(bad, LAB, "{edit}");
        match prismal_syntax::compile(&bad) {
            Err(ds) => ds.iter().map(|d| d.code.to_string()).collect(),
            Ok(c) => Program::new(c.doc).err().map(|ds| ds.iter().map(|d| d.code.to_string()).collect()).unwrap_or_default(),
        }
    };
    assert_eq!(codes("request add(p)", "request add"), ["PK-E02"], "the click supplies the payload");
    assert_eq!(codes("request add(p)", "request add(p.x)"), ["PK-E02"], "the payload's type");
    assert_eq!(codes("request add(p)", "request nothing(p)"), ["SX-E03"], "unknown event");
    assert_eq!(codes("on click as p request add(p)", "on drag as p request add(p)"), ["SX-E02"], "views take clicks only");
    assert_eq!(codes("    label(n)", "    on click as p request add(p)"), ["SX-E06"], "a panel has no points");
    let twice = LAB.replace("    on click as p request add(p)\n", "    on click as p request add(p)\n    on click as p request add(p)\n");
    assert_eq!(prismal_syntax::compile(&twice).err().unwrap().iter().map(|d| d.code.to_string()).collect::<Vec<_>>(), ["SX-E06"], "one click per view");
    let follower = LAB.replace("  derived {\n    n", "  event later on add(r: Point)\n  derived {\n    n").replace("request add(p)", "request later(p)");
    let err = Program::new(prismal_syntax::compile(&follower).unwrap().doc).err().unwrap();
    assert_eq!(err.iter().map(|d| d.code.to_string()).collect::<Vec<_>>(), ["PK-E03"], "a click requests an event declared `on request`");
}
