//! Input on members of collections (D-059): drags that propose a member's binding, refused
//! for members not alive and for endpoints of relations; clicks that request an event with
//! the member as payload; frames, printing and diagnostics.

mod common;
use common::*;
use prismal_ir::build::num;
use prismal_present::interact::{Interactive, Key};
use prismal_present::Program;
use prismal_runtime::Config;

const LAB: &str = "
space Plane = euclidean(2)

model Lab in Plane {
  object Ball {
    state {
      pos: Point            = origin intervenable
      vel: Vector<Velocity> = 0
    }
    flow { der(pos) = vel }
  }
  relation Tie(a in balls, b in balls) {}
  parts {
    balls: Ball[3, max 4] { pos = origin + (index * 1 m, 0 m) }
    ties: Tie[1, max 1] {
      a = balls[1]
      b = balls[2]
    }
  }
  event remove on request(b in balls) { destroy b }
  derived {
    x1: Length = balls[1].pos.x
    x2: Length = balls[2].pos.x
    y2: Length = balls[2].pos.y
  }
}

presentation Table for Lab {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    for b in balls {
      marker(b.pos) as ball { on drag as p { propose b.pos = p } }
    }
    for b in balls {
      marker(b.pos + (0 m, 1 m)) as tag { on click request remove(b) }
    }
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
fn a_drag_moves_one_member() {
    let mut i = session(LAB);
    // A keyboard step on `ball[2]` moves ball 2 by 10 px, 0.2 m; ball 1 stays.
    i.key("ball[2]", Key::Right).unwrap();
    assert!((value(&i, "x2") - 2.2).abs() < 1e-9, "{}", value(&i, "x2"));
    assert_eq!(value(&i, "x1"), 1.0);
    i.key("ball[2]", Key::Up).unwrap();
    assert!((value(&i, "y2") - 0.2).abs() < 1e-9, "{}", value(&i, "y2"));
    // Undone, the member is back.
    i.undo();
    i.undo();
    assert_eq!(value(&i, "x2"), 2.0);
}

#[test]
fn members_not_alive_are_not_dragged() {
    let mut i = session(LAB);
    i.request("remove", Some(num(2.0, ""))).unwrap();
    let e = i.pointer_down("ball[2]", None).unwrap_err();
    assert!(e.message.contains("not shown now"), "{}", e.message);
    // `ball[4]` is not made yet.
    let e = i.key("ball[4]", Key::Right).unwrap_err();
    assert!(e.message.contains("not shown now"), "{}", e.message);
    // The others still are.
    i.key("ball[3]", Key::Right).unwrap();
}

#[test]
fn printing_round_trip() {
    let doc = prismal_syntax::compile(LAB).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert_eq!(prismal_syntax::compile(&printed).unwrap_or_else(|d| panic!("{d:?}\n{printed}")).doc, doc, "{printed}");
    assert!(printed.contains("marker(b.pos) as ball { on drag as p { propose b.pos = p } }"), "{printed}");
    assert!(printed.contains("as tag { on click request remove(b) }"), "{printed}");
    // A drag and a click on one representation.
    let both = LAB.replace("{ on drag as p { propose b.pos = p } }", "{ on drag as p { propose b.pos = p }; on click request remove(b) }");
    let doc = prismal_syntax::compile(&both).unwrap().doc;
    let printed = prismal_syntax::format::format(&doc);
    assert!(printed.contains("as ball { on drag as p { propose b.pos = p }; on click request remove(b) }"), "{printed}");
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
    // The member's binding must be intervenable in the object type (D-023); the mistake is
    // reported for each member's representation.
    assert_eq!(codes("pos: Point            = origin intervenable", "pos: Point            = origin"), ["PK-E03"; 4], "not intervenable");
    assert_eq!(codes("propose b.pos = p", "propose b.place = p"), ["SX-E03"], "no such binding");
    assert_eq!(codes("propose b.pos = p", "propose b.pos.x = p.x"), ["SX-E06"], "a whole binding");
    assert_eq!(codes("on click request remove(b)", "on click request remove"), ["PK-E02"; 4], "the click supplies the payload");
    assert_eq!(codes("on click request remove(b)", "on click request nothing(b)"), ["SX-E03"], "unknown event");
    let follower = LAB.replace("  derived {
    x1", "  event gone on remove(c in balls)
  derived {
    x1").replace("on click request remove(b)", "on click request gone(b)");
    let err = Program::new(prismal_syntax::compile(&follower).unwrap().doc).err().unwrap();
    assert_eq!(err.iter().map(|d| d.code.to_string()).collect::<Vec<_>>(), ["PK-E03"; 4], "a click requests an event declared `on request`");
    // A member at an endpoint is dragged by its own representation.
    let ends = LAB.replace(
        "    for b in balls {\n      marker(b.pos) as ball { on drag as p { propose b.pos = p } }\n    }",
        "    for s in ties {\n      marker(s.a.pos) as end { on drag as p { propose s.a.pos = p } }\n    }",
    );
    assert_ne!(ends, LAB);
    let err = Program::new(prismal_syntax::compile(&ends).unwrap().doc).err().unwrap();
    assert_eq!(err.iter().map(|d| d.code.to_string()).collect::<Vec<_>>(), ["MK-E26"]);
    assert!(err[0].to_string().contains("dragged by its own representation"), "{}", err[0]);
}

#[test]
fn a_click_requests_with_the_member() {
    let mut i = session(LAB);
    // The frame says what a click does, in the text alternative too.
    let f = i.frame();
    let tag = f.rep("tag[2]").unwrap();
    assert_eq!(tag.click.as_deref(), Some("remove"));
    assert!(tag.text.ends_with(", activate: remove"), "{}", tag.text);
    assert!(f.rep("ball[2]").unwrap().click.is_none());
    i.seek(0.5);
    i.click("tag[2]").unwrap();
    assert!(i.frame().rep("tag[2]").is_none(), "removed, the member is not drawn");
    assert!(i.frame().rep("ball[2]").is_none());
    // Its representation cannot be clicked any more; others can.
    let e = i.click("tag[2]").unwrap_err();
    assert!(e.message.contains("not shown now"), "{}", e.message);
    let e = i.click("ball[1]").unwrap_err();
    assert!(e.message.contains("does nothing when clicked"), "{}", e.message);
    i.click("tag[3]").unwrap();
    assert_eq!(i.session.log().len(), 2);
}
