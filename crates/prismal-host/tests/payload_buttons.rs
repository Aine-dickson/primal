//! Buttons whose event takes a payload (D-072): `button(push(2 m))`.

use prismal_host::{Content, Engine};

const STACK: &str = "model Stack {
  discrete { height: Length = 0 m }
  event push on request(d: Length) { set height = height + d }
  event clear on request { set height = 0 m }
}
presentation Lab for Stack {
  panel controls {
    button(push(2 m)) as two
    button(push(0.5 m), label: \"half a metre\") as half
    button(clear)
    label(height)
  }
  observe { h = height live }
}
";

#[test]
fn a_button_requests_its_event_with_its_value() {
    let mut e = Engine::new();
    let doc = e.load(Content::Text(STACK.into())).unwrap();
    let (h, _) = e.open(&doc, "Lab", false).unwrap();
    let i = e.instance(&h).unwrap();
    assert_eq!(i.press("two")["ok"], true);
    assert_eq!(i.press("half")["ok"], true);
    assert_eq!(i.press("two")["ok"], true);
    let obs = i.observations().to_string();
    assert!(obs.contains("4.5 m"), "{obs}");
}

#[test]
fn a_payload_must_fit_the_event() {
    let cases = [
        ("button(push)", "takes a value"),
        ("button(clear(1 m))", "takes no value"),
        ("button(push(3 s))", "push"),
    ];
    for (button, msg) in cases {
        let src = STACK.replace("button(clear)\n", &format!("button(clear)\n    {button}\n"));
        let mut e = Engine::new();
        let doc = e.load(Content::Text(src.clone())).unwrap();
        let err = format!("{:?}", e.document(&doc).unwrap().diagnostics());
        assert!(err.contains(msg), "{button}: {err}");
    }
}
