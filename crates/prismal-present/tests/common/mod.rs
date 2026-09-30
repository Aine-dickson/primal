//! Reference programs read from their documents, compiled to checked programs.
#![allow(dead_code)]

use prismal_present::Program;
use prismal_syntax::{compile, markdown_blocks, markdown_source};
use std::path::PathBuf;

pub fn docs() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs")
}

pub fn rp_doc(prefix: &str) -> String {
    let dir = docs().join("spec/reference-programs");
    let path = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.file_name().unwrap().to_string_lossy().starts_with(prefix))
        .unwrap_or_else(|| panic!("no document {prefix}"));
    std::fs::read_to_string(path).unwrap()
}

pub fn rp(prefix: &str) -> String {
    markdown_source(&rp_doc(prefix))
}

/// RP-08: the RP-01 model with RP-08's addition, and RP-08's presentation.
pub fn rp08() -> String {
    let rp01 = markdown_blocks(&rp_doc("RP-01"));
    let rp08 = markdown_blocks(&rp_doc("RP-08"));
    let model = &rp01[0].1;
    let end = model.rfind("\n}").expect("end of the model");
    let mut src = format!("{}\n{}{}", &model[..end], rp08[0].1, &model[end..]);
    for (_, b) in &rp08[1..] {
        src.push_str(b);
    }
    src
}

/// The sections of the working-syntax document, by program (`RP-01` ...).
pub fn working_syntax(section: &str) -> String {
    let md = std::fs::read_to_string(docs().join("syntax-study/working-syntax.md")).unwrap();
    md.split("\n## ").find(|p| p.starts_with(section)).map(markdown_source).unwrap_or_else(|| panic!("no section {section}"))
}

/// The `cases` blocks of a working-syntax section.
pub fn working_syntax_cases(section: &str) -> String {
    let md = std::fs::read_to_string(docs().join("syntax-study/working-syntax.md")).unwrap();
    let part = md.split("\n## ").find(|p| p.starts_with(section)).unwrap();
    markdown_blocks(part).into_iter().map(|b| b.1).filter(|b| b.trim_start().starts_with("run ")).collect()
}

pub fn program(src: &str) -> Program {
    let c = compile(src).unwrap_or_else(|ds| panic!("{}", ds.iter().map(|d| d.render(src, "program")).collect::<Vec<_>>().join("\n")));
    Program::new(c.doc).unwrap_or_else(|ds| panic!("{}", ds.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("\n")))
}
