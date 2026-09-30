//! `prismal-stdio`: the host interface as a process (HI-6.4). A host written in any language
//! starts this program and exchanges the JSON protocol of `docs/spec/05-host-interface.md`
//! with it over standard input and output.
//!
//! Framing is JSON Lines: each request is one line of JSON, and each response is one line,
//! written in the order the requests arrived. A request may carry an `id` of any JSON
//! value; its response carries the same `id`, so a host can match responses to requests.
//! Blank lines are ignored. The process ends when its input ends.
//!
//! ```text
//! > {"protocol": 1, "op": "capabilities", "id": 1}
//! < {"id":1,"ok":{"protocol":1, ...}}
//! ```
//!
//! Diagnostics of the protocol itself (a line that is not JSON, an unknown operation) are
//! answered as the engine answers them (HI-E01); nothing is written to standard output but
//! responses. Messages for a person go to standard error.

use prismal_host::Engine;
use serde_json::Value as Json;
use std::io::{BufRead, Write};

/// Answers one line: the engine's response, with the request's `id` when it has one.
pub fn answer(engine: &mut Engine, line: &str) -> String {
    let response = engine.handle(line);
    let id = serde_json::from_str::<Json>(line).ok().and_then(|r| r.get("id").cloned());
    match id {
        Some(id) => {
            let mut v: Json = serde_json::from_str(&response).expect("the engine answers JSON");
            let mut out = serde_json::Map::new();
            out.insert("id".into(), id);
            if let Json::Object(m) = v.take() {
                out.extend(m);
            }
            Json::Object(out).to_string()
        }
        None => response,
    }
}

fn main() {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        eprintln!("prismal-stdio: the Prismal host protocol over standard input and output.");
        eprintln!("Write one JSON request per line; read one JSON response per line.");
        eprintln!("See docs/spec/05-host-interface.md.");
        return;
    }
    let mut engine = Engine::new();
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let response = answer(&mut engine, &line);
        // A host that stopped reading ends the process.
        if writeln!(out, "{response}").and_then(|_| out.flush()).is_err() {
            break;
        }
    }
}
