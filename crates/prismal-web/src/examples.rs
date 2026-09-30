//! The reference programs (docs/spec/reference-programs), embedded in the player as example
//! sources. Each is assembled from its documents the way the acceptance tests read them, so
//! the player always shows the programs as specified.

use prismal_syntax::markdown_blocks;

const WORKING_SYNTAX: &str = include_str!("../../../docs/syntax-study/working-syntax.md");
const RP01: &str = include_str!("../../../docs/spec/reference-programs/RP-01-projectile.md");
const RP02: &str = include_str!("../../../docs/spec/reference-programs/RP-02-projectile-drag.md");
const RP03: &str = include_str!("../../../docs/spec/reference-programs/RP-03-bouncing-ball.md");
const RP04: &str = include_str!("../../../docs/spec/reference-programs/RP-04-pendulum.md");
const RP05: &str = include_str!("../../../docs/spec/reference-programs/RP-05-spring-mass.md");
const RP06: &str = include_str!("../../../docs/spec/reference-programs/RP-06-function-plot.md");
const RP07: &str = include_str!("../../../docs/spec/reference-programs/RP-07-vector-addition.md");
const RP08: &str = include_str!("../../../docs/spec/reference-programs/RP-08-narrated-lesson.md");

/// An example program: a title and its source in the working syntax.
pub struct Example {
    pub key: &'static str,
    pub title: &'static str,
    pub source: String,
}

fn blocks(md: &str) -> Vec<String> {
    markdown_blocks(md).into_iter().map(|b| b.1).collect()
}

/// The `text` and `cases` blocks of a working-syntax section (`RP-01` ...).
fn section(name: &str) -> Vec<String> {
    WORKING_SYNTAX.split("\n## ").find(|p| p.starts_with(name)).map(blocks).unwrap_or_default()
}

/// The cases (`run` blocks) of a working-syntax section.
fn cases(name: &str) -> Vec<String> {
    section(name).into_iter().filter(|b| b.trim_start().starts_with("run ")).collect()
}

fn join(parts: &[String]) -> String {
    parts.iter().map(|p| p.trim_end()).collect::<Vec<_>>().join("\n\n") + "\n"
}

/// RP-08: the RP-01 model with RP-08's addition inside it, RP-08's presentation, and the
/// cases of RP-08's working-syntax section.
fn rp08() -> String {
    let rp01 = blocks(RP01);
    let rp08 = blocks(RP08);
    let model = &rp01[0];
    let end = model.rfind("\n}").expect("end of the model");
    let mut parts = vec![format!("{}\n{}{}", &model[..end], rp08[0].trim_end_matches('\n'), &model[end..])];
    parts.extend(rp08[1..].iter().cloned());
    parts.extend(cases("RP-08"));
    join(&parts)
}

pub fn all() -> Vec<Example> {
    // A program from its documents, with the cases of its working-syntax sections.
    let prog = |docs: &[&str], sections: &[&str]| {
        let mut parts: Vec<String> = docs.iter().flat_map(|d| blocks(d)).collect();
        parts.extend(sections.iter().flat_map(|s| cases(s)));
        join(&parts)
    };
    vec![
        Example { key: "rp08", title: "RP-08 Narrated projectile lesson", source: rp08() },
        Example { key: "rp01", title: "RP-01 Projectile, no drag", source: prog(&[RP01], &["RP-01"]) },
        Example { key: "rp02", title: "RP-02 Projectile with drag", source: prog(&[RP01, RP02], &["RP-01", "RP-02"]) },
        Example { key: "rp03", title: "RP-03 Bouncing ball", source: prog(&[RP03], &["RP-03"]) },
        Example { key: "rp04", title: "RP-04 Pendulum", source: prog(&[RP04], &["RP-04"]) },
        Example { key: "rp05", title: "RP-05 Spring-mass", source: prog(&[RP05], &["RP-05"]) },
        Example { key: "rp06", title: "RP-06 Function plot with draggable parameter", source: join(&blocks(RP06)) },
        Example { key: "rp07", title: "RP-07 Vector addition", source: join(&blocks(RP07)) },
    ]
}
