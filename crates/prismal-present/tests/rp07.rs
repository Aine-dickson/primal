//! RP-07 Vector addition, through the presentation: drawing, and the drag of `u`'s head
//! (docs/spec/reference-programs/RP-07-vector-addition.md, E1 to E5).

mod common;
use common::*;
use prismal_present::data::Data;
use prismal_present::frame::{Frame, Shape};
use prismal_present::interact::Interactive;
use prismal_runtime::Config;

fn values(i: &Interactive) -> Vec<f64> {
    match i.observe("values").unwrap() {
        Data::Value(v) => prismal_present::flat(&v),
        d => panic!("{d:?}"),
    }
}

fn arrow(f: &Frame, n: usize) -> ([f64; 2], [f64; 2]) {
    match f.rep(&format!("arrow.{n}")).unwrap().shape {
        Shape::Arrow { from, to } => (from, to),
        ref s => panic!("{s:?}"),
    }
}

#[test]
fn rp07_drag_head_of_u() {
    let prog = program(&rp("RP-07"));
    let mut i = Interactive::new(&prog, "VectorPlot", Config::until(0.0)).unwrap();

    // E1 to E3: sum = (3 m, 2 m), length = sqrt 13, B = (4 m, 4 m).
    assert_eq!(values(&i), vec![3.0, 2.0, 13f64.sqrt(), 4.0, 4.0], "RP-07.E1 to E3");
    // Drawing at 40 px per metre, y up: A = (1 m, 2 m) is (40, -80).
    let f = i.frame();
    assert_eq!(arrow(&f, 1), ([40.0, -80.0], [160.0, -80.0]), "u from A");
    assert_eq!(arrow(&f, 2), ([160.0, -80.0], [160.0, -160.0]), "w from mid");
    assert!(f.reps().any(|r| r.kind == "axes") && f.reps().any(|r| r.kind == "grid"));

    // The learner drags the head of `u` to the model point (5 m, 1 m), view (200, -40).
    assert!(i.pointer_down("arrow.1", None).is_err(), "`u` is dragged by its head");
    i.pointer_down("arrow.1", Some("head")).unwrap();
    assert!(i.pointer_move([200.0, -40.0]));
    assert!(i.pointer_up().unwrap());

    // E4: u = (4 m, -1 m), sum = (4 m, 1 m), length = sqrt 17, B = (5 m, 3 m).
    assert_eq!(values(&i), vec![4.0, 1.0, 17f64.sqrt(), 5.0, 3.0], "RP-07.E4");
    let f = i.frame();
    assert_eq!(arrow(&f, 1), ([40.0, -80.0], [200.0, -40.0]), "RP-07.E4 u");
    // E5: w starts at the new mid = (5 m, 1 m), in the same frame (PK-7.5).
    assert_eq!(arrow(&f, 2).0, [200.0, -40.0], "RP-07.E5 w from the new mid");
    assert_eq!(arrow(&f, 3), ([40.0, -80.0], [200.0, -120.0]), "RP-07.E5 sum");
    let label = f.reps().find(|r| r.kind == "label").unwrap();
    assert_eq!(label.text, "length = 4.12311 m");
    assert_eq!(i.session.log().len(), 1, "one intervention");
}

/// PK-11.2, PK-11.2a: the head of `u` moves by 10 px keyboard steps, each one committed
/// intervention through the declared inverse.
#[test]
fn rp07_keyboard_moves_head_of_u() {
    use prismal_present::interact::Key;
    let prog = program(&rp("RP-07"));
    let mut i = Interactive::new(&prog, "VectorPlot", Config::until(0.0)).unwrap();
    i.key("arrow.1", Key::Right).unwrap();
    i.key("arrow.1", Key::Up).unwrap();
    // 10 px is 0.25 m at 40 px per metre: u = (3.25 m, 0.25 m).
    assert_eq!(arrow(&i.frame(), 1), ([40.0, -80.0], [170.0, -90.0]));
    assert_eq!(&values(&i)[..2], &[3.25, 2.25]);
    assert_eq!(i.session.log().len(), 2);
    assert!(i.key("arrow.2", Key::Right).is_err(), "`w` declares no inverse");
}
