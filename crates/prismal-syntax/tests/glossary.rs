//! The reference's glossary (guide chapter 10) covers every word of the parser's lists,
//! and its "Words" section lists them exactly.

use prismal_syntax::parser::{CONTEXTUAL, RESERVED};
use std::collections::BTreeSet;

fn reference() -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/guide/10-reference.md");
    std::fs::read_to_string(path).expect("guide chapter 10")
}

fn section<'a>(text: &'a str, heading: &str) -> &'a str {
    let start = text.find(heading).unwrap_or_else(|| panic!("no section {heading}"));
    let rest = &text[start + heading.len()..];
    let end = rest.find("\n## ").unwrap_or(rest.len());
    &rest[..end]
}

/// Words in backticks in the first column of the glossary table.
fn glossary_words(text: &str) -> BTreeSet<String> {
    let mut words = BTreeSet::new();
    for line in section(text, "## Glossary").lines() {
        let Some(first) = line.strip_prefix('|').and_then(|l| l.split('|').next()) else { continue };
        for (i, part) in first.split('`').enumerate() {
            if i % 2 == 1 {
                words.insert(part.trim().to_string());
            }
        }
    }
    words
}

/// Words in backticks after a bold label in the "Words" section.
fn listed(text: &str, label: &str) -> BTreeSet<String> {
    let words = section(text, "## Words");
    let line = words.lines().find(|l| l.starts_with(label)).unwrap_or_else(|| panic!("no line {label}"));
    let inner = line.split('`').nth(1).expect("backticked list");
    inner.split_whitespace().map(str::to_string).collect()
}

#[test]
fn glossary_covers_every_word() {
    let text = reference();
    let glossary = glossary_words(&text);
    let missing: Vec<_> = RESERVED.iter().chain(CONTEXTUAL).filter(|w| !glossary.contains(**w)).collect();
    assert!(missing.is_empty(), "words missing from the glossary: {missing:?}");
}

#[test]
fn word_lists_match_the_parser() {
    let text = reference();
    let reserved: BTreeSet<String> = RESERVED.iter().map(|w| w.to_string()).collect();
    let contextual: BTreeSet<String> = CONTEXTUAL.iter().map(|w| w.to_string()).collect();
    let r = listed(&text, "**Reserved**");
    let c = listed(&text, "**Contextual**");
    assert_eq!(r, reserved, "reserved words differ: doc-only {:?}, parser-only {:?}", r.difference(&reserved).collect::<Vec<_>>(), reserved.difference(&r).collect::<Vec<_>>());
    assert_eq!(c, contextual, "contextual words differ: doc-only {:?}, parser-only {:?}", c.difference(&contextual).collect::<Vec<_>>(), contextual.difference(&c).collect::<Vec<_>>());
}

/// A word in the wrong block is reported with where it belongs (guide, What goes where).
#[test]
fn misplaced_words_say_where_they_belong() {
    let cases = [
        ("model M {\n  param { k: Real = 1 }\n  slider(k, range: [0, 2])\n}\n", "belongs in a presentation, inside `panel"),
        ("model M {\n  state { x: Real = 1; der(x) = -x }\n}\n", "belongs in the model's `flow { ... }` block"),
        ("model M {\n  state { x: Real = 1 }\n  set x = 1\n}\n", "belongs in an event's braces"),
        ("model M {\n  state { x: Real = 1 }\n}\npresentation P for M {\n  marker(x)\n}\n", "belongs inside `view name: plot(...)"),
    ];
    for (src, hint) in cases {
        let (_, ds) = prismal_syntax::parse(src);
        assert!(ds.iter().any(|d| d.message.contains(hint)), "{src}: {:?}", ds.iter().map(|d| &d.message).collect::<Vec<_>>());
    }
}
