//! Checks a program in the working syntax and prints its diagnostics or its IR.
//!
//! `cargo run -p prismal-syntax --example prismalc -- FILE [--ir]`
//!
//! FILE is a source file, or a Markdown document whose `text` and `cases` blocks form the
//! program (the reference programs). With `--ir` the IR is printed as JSON (D-037).

use prismal_syntax::{check, markdown_source};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = args.iter().find(|a| !a.starts_with("--")) else {
        eprintln!("usage: prismalc FILE [--ir]");
        std::process::exit(2);
    };
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("{path}: {e}");
        std::process::exit(2);
    });
    let src = if path.ends_with(".md") { markdown_source(&text) } else { text };
    match check(&src) {
        Ok((c, _)) => {
            if args.iter().any(|a| a == "--ir") {
                println!("{}", c.doc.to_json());
            } else {
                for m in &c.doc.models {
                    println!(
                        "{path}: model {}: {} bindings, {} flows, {} events, {} equations, {} constraints: ok",
                        m.name,
                        m.bindings.len(),
                        m.flows.len(),
                        m.events.len(),
                        m.equations.len(),
                        m.constraints.len()
                    );
                }
            }
        }
        Err(diags) => {
            for d in &diags {
                eprintln!("{}\n", d.render(&src, path));
            }
            eprintln!("{path}: {} error(s)", diags.len());
            std::process::exit(1);
        }
    }
}
