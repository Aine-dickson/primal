//! The protocol through a running `prismal-stdio` process, as a host in another language
//! would use it: requests written to its standard input, responses read from its standard
//! output, one per line, matched by `id`.

use serde_json::{json, Value as Json};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const CART: &str = "model Cart {
  const { m: Mass = 2 kg }
  input { thrust: Force = 0 N }
  state { x: Length = 0 m; v: Velocity = 0 m/s }
  flow { der(x) = v; der(v) = thrust / m }
}
presentation Lab for Cart {
  view speed: plot(x: [0 s, 4 s], y: [0 m/s, 10 m/s]) { series_plot(v every 0.1 s) }
  panel status { label(v) }
}
";

struct Process {
    child: std::process::Child,
    input: std::process::ChildStdin,
    output: BufReader<std::process::ChildStdout>,
}

impl Process {
    fn start() -> Process {
        let mut child = Command::new(env!("CARGO_BIN_EXE_prismal-stdio")).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("the binary starts");
        let input = child.stdin.take().unwrap();
        let output = BufReader::new(child.stdout.take().unwrap());
        Process { child, input, output }
    }

    fn line(&mut self, text: &str) -> Json {
        writeln!(self.input, "{text}").unwrap();
        self.input.flush().unwrap();
        let mut response = String::new();
        self.output.read_line(&mut response).unwrap();
        serde_json::from_str(&response).unwrap_or_else(|e| panic!("{e}: {response}"))
    }

    fn ask(&mut self, id: u64, mut req: Json) -> Json {
        req["protocol"] = json!(1);
        req["id"] = json!(id);
        let r = self.line(&req.to_string());
        assert_eq!(r["id"], json!(id), "the response carries the request's id");
        r
    }
}

#[test]
fn a_host_drives_the_process() {
    let mut p = Process::start();
    let caps = p.ask(1, json!({ "op": "capabilities" }));
    assert_eq!(caps["ok"]["protocol"], 1);

    let doc = p.ask(2, json!({ "op": "load", "text": CART }));
    let d = doc["ok"]["document"].as_str().unwrap().to_string();
    let opened = p.ask(3, json!({ "op": "open", "document": d, "presentation": "Lab" }));
    let i = opened["ok"]["instance"].as_str().unwrap().to_string();
    assert_eq!(opened["ok"]["layout"]["mode"], "interactive");

    // The environment pushes: 4 N from 1 s, so v = 2 m/s at 2 s.
    p.ask(4, json!({ "op": "seek", "instance": i, "time": 1.0 }));
    assert_eq!(p.ask(5, json!({ "op": "set_input", "instance": i, "input": "thrust", "value": 4.0 }))["ok"]["ok"], true);
    p.ask(6, json!({ "op": "seek", "instance": i, "time": 2.0 }));
    let frame = p.ask(7, json!({ "op": "frame", "instance": i }));
    assert!(frame["ok"].to_string().contains("v = 2 m/s"), "{frame}");

    // Errors are responses too; the process keeps serving.
    let bad = p.line("this is not JSON");
    assert_eq!(bad["error"][0]["code"], "HI-E01");
    assert!(bad.get("id").is_none());
    let unknown = p.ask(8, json!({ "op": "fly" }));
    assert_eq!(unknown["error"][0]["code"], "HI-E01");
    let ids = p.ask(9, json!({ "op": "close", "instance": i }));
    assert!(ids.get("ok").is_some(), "{ids}");

    // String ids are echoed as given; blank lines are ignored.
    writeln!(p.input).unwrap();
    let r = p.line(&json!({ "protocol": 1, "op": "capabilities", "id": "caps-2" }).to_string());
    assert_eq!(r["id"], "caps-2");

    // The process ends when its input ends.
    drop(p.input);
    let status = p.child.wait().unwrap();
    assert!(status.success());
}
