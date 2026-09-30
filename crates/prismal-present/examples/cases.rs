//! Runs the cases of a program headless and reports each expectation (PK-4.2).
//!
//! `cargo run -p prismal-present --example cases -- FILE...`
//!
//! Each FILE is a source file or a Markdown document whose `text` and `cases` blocks form
//! the program; several files are read as one program (RP-02 presents the RP-01 model).

use prismal_present::expect::run_case;
use prismal_present::Program;
use prismal_syntax::{compile, markdown_source};

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("usage: cases FILE...");
        std::process::exit(2);
    }
    let mut src = String::new();
    for p in &paths {
        let text = std::fs::read_to_string(p).unwrap_or_else(|e| {
            eprintln!("{p}: {e}");
            std::process::exit(2);
        });
        src.push_str(&if p.ends_with(".md") { markdown_source(&text) } else { text });
    }
    let doc = match compile(&src) {
        Ok(c) => c.doc,
        Err(ds) => {
            for d in &ds {
                eprintln!("{}\n", d.render(&src, "program"));
            }
            std::process::exit(1);
        }
    };
    let prog = Program::new(doc).unwrap_or_else(|ds| {
        for d in &ds {
            eprintln!("{d}");
        }
        std::process::exit(1);
    });
    if prog.doc.runs.is_empty() {
        println!("no cases");
    }
    let mut failed = 0;
    for case in &prog.doc.runs {
        match run_case(&prog, case) {
            Ok(report) => {
                println!("run {}", case.name);
                if let Some(pb) = &report.playback {
                    for b in &pb.beats {
                        println!("  beat {:<4} {:>10.6} s to {:>10.6} s", b.name, b.start, b.end);
                    }
                }
                for r in &report.results {
                    let id = r.id.rsplit('.').next().unwrap_or(&r.id);
                    if r.pass {
                        println!("  expect {id}: pass");
                    } else {
                        failed += 1;
                        println!("  expect {id}: FAIL: {}", r.message);
                    }
                }
            }
            Err(e) => {
                failed += 1;
                println!("run {}: {e}", case.name);
            }
        }
    }
    if failed > 0 {
        std::process::exit(1);
    }
}
