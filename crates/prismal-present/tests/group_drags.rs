//! Drags on members of a group (D-043, D-062): the gesture value is the pointer in the
//! group's own frame, where the member is written; keyboard steps and the host's focus
//! order reach members of groups.

mod common;
use common::*;
use prismal_present::interact::{Interactive, Key};
use prismal_runtime::Config;

const DIAL: &str = "
space Plane = euclidean(2)

model Dial in Plane {
  param { r: Length = 1 m  in [0.2 m, 3 m] }
  state { x: Length = 0 m }
  flow { der(x) = 1 m/s }
}

presentation Knob for Dial {
  view scene: spatial(Plane, scale: 1 m -> 50 px, y: up) {
    group(at: origin + (2 m, 1 m), rotate: 90 deg) as dial {
      segment(origin, origin + (r, 0 m))
      marker(origin + (r, 0 m)) as tip { on drag as p { propose r = p.x } }
    }
  }
}
";

fn r(i: &Interactive) -> f64 {
    let k = i.cm.index["Dial.r"];
    i.session.current.state_at(i.t)[k].num()
}

fn session() -> Interactive {
    Interactive::new(&program(DIAL), "Knob", Config::until(2.0)).unwrap_or_else(|d| panic!("{d:?}"))
}

#[test]
fn a_member_is_dragged_in_its_group_frame() {
    let mut i = session();
    // The tip is drawn at (2 m, 2 m): the group's origin, then r along its x axis turned up.
    let f = i.frame();
    let tip = f.rep("tip").expect("the tip is in the frame");
    assert!(tip.drag.is_some());
    // Dragged to the view point of (2 m, 2.5 m): in the group's frame that is (1.5 m, 0 m),
    // so `p.x` is 1.5 m. Read in the view's space it would be 2 m.
    i.pointer_down("tip", None).unwrap();
    assert!(i.pointer_move([100.0, -125.0]));
    assert!(i.pointer_up().unwrap());
    assert!((r(&i) - 1.5).abs() < 1e-12, "{}", r(&i));
    // A keyboard step up the screen is along the group's x axis: r grows. A step right is
    // across it: r stays.
    i.key("tip", Key::Up).unwrap();
    let up = r(&i);
    assert!(up > 1.5 + 1e-9, "{up}");
    i.key("tip", Key::Right).unwrap();
    assert!((r(&i) - up).abs() < 1e-9, "{} {up}", r(&i));
}

#[test]
fn proposals_are_validated_in_the_group_too() {
    let mut i = session();
    // (2 m, 5 m) is 4 m along the group's axis: outside `r in [0.2 m, 3 m]`.
    i.pointer_down("tip", None).unwrap();
    assert!(!i.pointer_move([100.0, -250.0]));
    assert!(!i.pointer_up().unwrap());
    assert_eq!(r(&i), 1.0);
}
