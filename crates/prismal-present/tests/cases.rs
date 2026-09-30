//! Every case written in the working syntax runs headless and meets all its expectations
//! (PK-4.2), including list-valued ones (event logs, empty diagnostics) and beat timings.

mod common;
use common::*;
use prismal_ir::Document;
use prismal_present::expect::run_case;

fn check_all(src: &str, what: &str) -> usize {
    let prog = program(src);
    let mut n = 0;
    for case in &prog.doc.runs {
        let report = run_case(&prog, case).unwrap_or_else(|e| panic!("{what} {}: {e}", case.name));
        for r in &report.results {
            assert!(r.pass, "{what} {}: {}: {}", case.name, r.id, r.message);
        }
        n += report.results.len();
    }
    n
}

#[test]
fn working_syntax_cases_pass() {
    let mut n = 0;
    n += check_all(&working_syntax("RP-01"), "RP-01");
    n += check_all(&(working_syntax("RP-01") + &working_syntax("RP-02")), "RP-02");
    n += check_all(&working_syntax("RP-03"), "RP-03");
    n += check_all(&working_syntax("RP-04"), "RP-04");
    n += check_all(&working_syntax("RP-05"), "RP-05");
    n += check_all(&(rp08() + &working_syntax_cases("RP-08")), "RP-08");
    // RP-01 runs twice (it is also part of the RP-02 program).
    assert_eq!(n, 36, "expectations evaluated");
}

#[test]
fn presentation_and_run_ir_round_trip() {
    for src in [rp("RP-06"), rp("RP-07"), rp08() + &working_syntax_cases("RP-08"), working_syntax("RP-03")] {
        let doc = prismal_syntax::compile(&src).unwrap().doc;
        assert!(!doc.presentations.is_empty());
        let json = doc.to_json();
        assert_eq!(Document::from_json(&json).unwrap(), doc);
    }
}
