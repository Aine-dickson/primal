//! Every program block in the reference programs and the working-syntax document parses,
//! and every reference-program document lowers to the IR.

use prismal_syntax::{compile, markdown_blocks, markdown_source, parse};
use std::path::PathBuf;

fn docs() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs")
}

fn reference_programs() -> Vec<(String, String)> {
    let dir = docs().join("spec/reference-programs");
    let mut out: Vec<(String, String)> = std::fs::read_dir(&dir)
        .expect("reference programs")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with("RP-"))
        .map(|p| (p.file_name().unwrap().to_string_lossy().into_owned(), std::fs::read_to_string(&p).unwrap()))
        .collect();
    out.sort();
    out
}

/// A block that continues a program shown elsewhere (RP-08's model addition) starts indented.
fn is_fragment(block: &str) -> bool {
    block.lines().find(|l| !l.trim().is_empty()).is_some_and(|l| l.starts_with(' '))
}

fn assert_parses(file: &str, first_line: u32, block: &str) {
    let (_, diags) = parse(block);
    let msgs: Vec<String> = diags.iter().map(|d| format!("{file}:{}:{}: {}: {}", d.span.line + first_line - 1, d.span.col, d.code, d.message)).collect();
    assert!(msgs.is_empty(), "{}", msgs.join("\n"));
}

#[test]
fn every_block_parses() {
    let mut n = 0;
    let ws = std::fs::read_to_string(docs().join("syntax-study/working-syntax.md")).unwrap();
    for (name, md) in reference_programs().into_iter().chain([("working-syntax.md".to_string(), ws)]) {
        for (line, block) in markdown_blocks(&md) {
            if is_fragment(&block) {
                continue;
            }
            assert_parses(&name, line, &block);
            n += 1;
        }
    }
    assert!(n >= 28, "only {n} blocks found");
}

#[test]
fn reference_programs_lower() {
    let all = reference_programs();
    let rp01 = markdown_source(&all.iter().find(|(n, _)| n.starts_with("RP-01")).unwrap().1);
    for (name, md) in &all {
        if name.starts_with("RP-08") {
            continue; // an addition to RP-01; lowered in the runtime tests
        }
        // RP-02 presents the RP-01 model.
        let src = if name.starts_with("RP-02") { rp01.clone() + &markdown_source(md) } else { markdown_source(md) };
        if let Err(diags) = compile(&src) {
            let msgs: Vec<String> = diags.iter().map(|d| d.render(&src, name)).collect();
            panic!("{}", msgs.join("\n"));
        }
    }
}
