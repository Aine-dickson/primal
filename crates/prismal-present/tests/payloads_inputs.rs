//! Event payloads (MK-15.1, D-050) and inputs from the environment (RC-11.6, D-051): a cart
//! pushed by an input force and kicked by requests that carry an impulse.

mod common;
use common::*;
use prismal_ir::build::num;
use prismal_present::expect::run_case;
use prismal_runtime::{run, Action, Config, RunStatus};
use prismal_syntax::{check, compile, format};

const SRC: &str = "model Cart {
  const { m: Mass = 2 kg }
  input { thrust: Force = 0 N }
  state {
    x: Length   = 0 m
    v: Velocity = 0 m/s
  }
  discrete {
    pushes: Real     = 0
    last:   Momentum = 0 kg*m/s
    total:  Momentum = 0 kg*m/s
  }
  flow {
    der(x) = v
    der(v) = thrust / m
  }
  /// A kick of a given impulse, requested from outside.
  event kick on request(j: Momentum) if j > 0 kg*m/s { set v = v + j / m; emit tally(2 * j) }
  event tally on request(k: Momentum)
  event counted on tally(k: Momentum) { set total = total + k }
  event echo on kick(j: Momentum) { set last = j }
  event pushed on input(thrust) { set pushes = pushes + 1 }
}

presentation CartChecks for Cart {
  observe {
    x_end  = x at t0 + 2 s
    v_end  = v at t0 + 2 s
    count  = pushes at t0 + 2 s
    events = event_log over [t0, t0 + 2 s]
  }
}

run push of Cart with CartChecks {
  input {
    thrust = 4 N
    thrust = 0 N at t0 + 1 s
  }
  until t0 + 2 s
  expect {
    v_end == 2 m/s within 1e-9 m/s
    x_end == 3 m within 1e-9 m
    count == 1 exactly
    events == [pushed]
  }
}

run still of Cart with CartChecks {
  until t0 + 2 s
  expect {
    x_end == 0 m exactly
    count == 0 exactly
  }
}
";

#[test]
fn inputs_from_run_cases() {
    let prog = program(SRC);
    for case in &prog.doc.runs {
        let r = run_case(&prog, case).unwrap();
        assert!(r.failures().is_empty(), "{}: {:?}", case.name, r.failures());
    }
    // The canonical text prints the inputs of a case and the payloads of events.
    let text = format(&compile(SRC).unwrap().doc);
    assert!(text.contains("input { thrust = 4 N; thrust = 0 N at t0 + 1 s }"), "{text}");
    assert!(text.contains("event kick on request(j: Momentum) if j > 0 kg*m/s {"), "{text}");
    assert!(text.contains("event counted on tally(k: Momentum)"), "{text}");
    assert!(text.contains("emit tally(2 * j)"), "{text}");
}

#[test]
fn payloads_from_requests_and_emits() {
    let prog = program(SRC);
    let cm = prog.model("Cart");
    let cfg = Config::until(2.0)
        .at(0.5, Action::RequestWith("Cart.event.kick".into(), num(4.0, "kg*m/s")))
        .at(1.0, Action::RequestWith("Cart.event.kick".into(), num(-1.0, "kg*m/s")))
        .at(1.5, Action::Request("Cart.event.kick".into()));
    let r = run(cm, cfg);
    assert!(matches!(r.status, RunStatus::Completed), "{:?}", r.status);
    let names: Vec<&str> = r.log.iter().map(|e| e.name.as_str()).collect();
    // The request at 1 s does not occur: its enabling condition is false for a negative
    // impulse (MK-15.5), and it is rejected with that reason (D-059). The one at 1.5 s
    // supplies no payload and is rejected. The followers of `kick` share the next microstep,
    // in declaration order.
    assert_eq!(names, ["kick", "counted", "echo"]);
    assert_eq!(r.log[0].payload, Some(prismal_kernel::Value::Num(4.0)), "the requested impulse");
    assert_eq!(r.log[1].payload, Some(prismal_kernel::Value::Num(8.0)), "`emit tally(2 * j)` supplies the followers' payload");
    assert_eq!(r.log[2].payload, Some(prismal_kernel::Value::Num(4.0)), "`on kick(j)` receives kick's payload");
    assert_eq!(r.rejected.len(), 2, "{:?}", r.rejected);
    assert!(r.rejected[0].message.contains("not enabled"), "{}", r.rejected[0].message);
    assert!(r.rejected[1].message.contains("supplies its payload"), "{}", r.rejected[1].message);
    let end = r.state_at(2.0);
    let (v, last, total) = (cm.idx("Cart.v"), cm.idx("Cart.last"), cm.idx("Cart.total"));
    assert_eq!(end[v], prismal_kernel::Value::Num(2.0), "4 kg m/s on 2 kg");
    assert_eq!(end[last], prismal_kernel::Value::Num(4.0));
    assert_eq!(end[total], prismal_kernel::Value::Num(8.0));
}

#[test]
fn input_changes_at_instants() {
    let prog = program(SRC);
    let cm = prog.model("Cart");
    // Thrust 2 N from 0.5 s: v(2) = 1 m/s^2 * 1.5 s; each change makes `pushed` due.
    let cfg = Config::until(2.0).at(0.5, Action::Input("Cart.thrust".into(), num(2.0, "N"))).at(1.0, Action::Input("Cart.thrust".into(), num(2.0, "N")));
    let r = run(cm, cfg);
    assert!(matches!(r.status, RunStatus::Completed), "{:?}", r.status);
    let end = r.state_at(2.0);
    match &end[cm.idx("Cart.v")] {
        prismal_kernel::Value::Num(v) => assert!((v - 1.5).abs() < 1e-9, "{v}"),
        other => panic!("{other:?}"),
    }
    assert_eq!(end[cm.idx("Cart.pushes")], prismal_kernel::Value::Num(2.0), "every supplied value is a change event (RC-11.6)");
    // An input is not a parameter: interventions cannot set it, and inputs are validated.
    let bad = run(cm, Config::until(1.0).at(0.5, Action::Input("Cart.x".into(), num(1.0, "m"))));
    assert!(bad.rejected.iter().any(|d| d.message.contains("not an input")), "{:?}", bad.rejected);
    let wrong = run(cm, Config::until(1.0).at(0.5, Action::Input("Cart.thrust".into(), num(1.0, "m"))));
    assert_eq!(wrong.rejected.len(), 1, "a length is not a force");
}

#[test]
fn inputs_without_a_value_and_payload_diagnostics() {
    // Without a default and without a starting value, the run cannot start.
    let src = SRC.replace("input { thrust: Force = 0 N }", "input { thrust: Force }");
    let prog = program(&src);
    let r = run(prog.model("Cart"), Config::until(1.0));
    assert!(matches!(&r.status, RunStatus::NotStarted(d) if d.message.contains("no value at the start")), "{:?}", r.status);
    let r = run(prog.model("Cart"), Config::until(1.0).param("Cart.thrust", num(1.0, "N")));
    assert!(matches!(r.status, RunStatus::Completed), "a starting value from the configuration: {:?}", r.status);

    let codes = |s: &str| check(s).err().map(|ds| ds.iter().map(|d| d.code).collect::<Vec<_>>()).unwrap_or_default();
    // A payload needs a source: a request, or an event that carries one of the same type.
    assert!(codes(&SRC.replace("event pushed on input(thrust)", "event pushed on input(thrust) ").replace("event echo on kick(j: Momentum)", "event echo on pushed(j: Momentum)")).contains(&"MK-E24"));
    assert!(codes(&SRC.replace("event echo on kick(j: Momentum)", "event echo on kick(j: Force)")).contains(&"MK-E24"));
    // Emitting needs the declared payload, of its type.
    assert!(codes(&SRC.replace("emit tally(2 * j)", "emit tally")).contains(&"MK-E01"));
    assert!(codes(&SRC.replace("emit tally(2 * j)", "emit tally(2 * v)")).contains(&"MK-E01"));
    // A payload is read only in its own event.
    assert!(codes(&SRC.replace("{ set pushes = pushes + 1 }", "{ set pushes = pushes + 1; set last = j }")).contains(&"SX-E03"));
}
