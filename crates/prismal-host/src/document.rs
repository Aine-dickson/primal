//! A document (HI section 3): a program loaded from source text or from the IR, with its
//! identities kept across updates (D-036), its diagnostics, and, when it passes its checks,
//! the checked program that instances present.

use prismal_ir::Document as Ir;
use prismal_present::expect::run_case;
use prismal_present::Program;
use prismal_syntax::{compile, markdown_source, SourceMap};
use serde::Serialize;
use serde_json::{json, Value as Json};
use std::rc::Rc;

/// What a host sends: source text (the working syntax, or a Markdown document whose `text`
/// and `cases` blocks form the program) or an IR document (04).
#[derive(Clone, Debug)]
pub enum Content {
    Text(String),
    Ir(Ir),
}

/// A source span: line and column from 1, byte offsets into the text.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Span {
    pub line: u32,
    pub col: u32,
    pub start: usize,
    pub end: usize,
}

impl From<prismal_syntax::Span> for Span {
    fn from(s: prismal_syntax::Span) -> Span {
        Span { line: s.line, col: s.col, start: s.start, end: s.end }
    }
}

/// A diagnostic (HI-3.4): the element it concerns and, for a document from text, where.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

impl Diagnostic {
    pub fn new(code: &str, message: impl Into<String>) -> Diagnostic {
        Diagnostic { code: code.into(), message: message.into(), element: None, span: None }
    }
}

pub struct Document {
    ir: Ir,
    /// The source text the document was last loaded or updated from, if any.
    text: Option<String>,
    /// Spans of elements in `text`; empty for a document from the IR or edited by `rename`
    /// (IR-1.6).
    spans: SourceMap,
    /// The checked program, when the document passes its checks.
    program: Option<Rc<Program>>,
    diagnostics: Vec<Diagnostic>,
}

/// A compiled document before checking: its IR, text and spans.
struct Compiled {
    ir: Ir,
    text: Option<String>,
    spans: SourceMap,
}

fn compile_content(content: Content) -> Result<Compiled, Vec<Diagnostic>> {
    match content {
        Content::Ir(ir) => Ok(Compiled { ir, text: None, spans: SourceMap::new() }),
        Content::Text(src) => {
            let text = if src.contains("```") { markdown_source(&src) } else { src };
            match compile(&text) {
                Ok(c) => Ok(Compiled { ir: c.doc, text: Some(text), spans: c.source_map }),
                Err(ds) => Err(ds
                    .into_iter()
                    .map(|d| Diagnostic { code: d.code.to_string(), message: d.message, element: d.element, span: Some(d.span.into()) })
                    .collect()),
            }
        }
    }
}

impl Document {
    /// Loads a document (HI-3.1). Fails only when the content does not compile; a document
    /// that fails its checks is loaded with its diagnostics.
    pub fn load(content: Content) -> Result<Document, Vec<Diagnostic>> {
        let c = compile_content(content)?;
        let mut d = Document { ir: c.ir, text: c.text, spans: c.spans, program: None, diagnostics: vec![] };
        d.check();
        Ok(d)
    }

    /// Replaces the document's content (HI-3.2), keeping the identities of elements that
    /// match the previous version. Answers the identities that disappeared. When the new
    /// content does not compile, the document is unchanged.
    pub fn update(&mut self, content: Content) -> Result<Vec<String>, Vec<Diagnostic>> {
        let c = compile_content(content)?;
        let (ir, map) = prismal_syntax::identity::reconcile_map(c.ir, &self.ir);
        let removed = prismal_syntax::diff(&self.ir, &ir).removed;
        self.spans = c.spans.into_iter().map(|(id, s)| (map.get(&id).cloned().unwrap_or(id), s)).collect();
        self.ir = ir;
        self.text = c.text;
        self.check();
        Ok(removed)
    }

    /// Renames an element and keeps its identity (HI-3.3, D-036).
    pub fn rename(&mut self, element: &str, name: &str) -> Result<(), Diagnostic> {
        let ir = prismal_syntax::rename(&self.ir, element, name).map_err(|m| Diagnostic { code: "HI-E02".into(), message: m, element: Some(element.into()), span: None })?;
        self.ir = ir;
        // The text no longer says what the IR says; a host asks for the canonical text.
        self.text = None;
        self.spans.clear();
        self.check();
        Ok(())
    }

    fn check(&mut self) {
        match Program::new(self.ir.clone()) {
            Ok(p) => {
                self.program = Some(Rc::new(p));
                self.diagnostics = vec![];
            }
            Err(ds) => {
                self.program = None;
                self.diagnostics = ds
                    .iter()
                    .map(|d| Diagnostic { code: d.code.to_string(), message: d.message.clone(), element: Some(d.element.clone()), span: self.span_of(&d.element) })
                    .collect();
            }
        }
    }

    /// The span of an element, or of the nearest enclosing element that has one.
    fn span_of(&self, element: &str) -> Option<Span> {
        // An element of a member is located at its declaration (D-055).
        let mut id = prismal_ir::elaborate::declaration(element);
        loop {
            if let Some(s) = self.spans.get(id) {
                return Some((*s).into());
            }
            id = &id[..id.rfind('.')?];
        }
    }

    pub fn ir(&self) -> &Ir {
        &self.ir
    }

    /// The source text the document came from, if it came from text and was not renamed.
    pub fn source(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// The canonical text (working syntax section 1.4).
    pub fn text(&self) -> String {
        prismal_syntax::format(&self.ir)
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// The checked program, or `None` when the document has diagnostics.
    pub fn program(&self) -> Option<Rc<Program>> {
        self.program.clone()
    }

    /// The source span of an element (HI section 6.1, `locate`).
    pub fn locate(&self, element: &str) -> Option<Span> {
        self.spans.get(prismal_ir::elaborate::declaration(element)).map(|s| (*s).into())
    }

    /// The models, presentations (with the mode each opens in) and cases (HI-3.4).
    pub fn catalogue(&self) -> Json {
        let doc = &self.ir;
        let pres: Vec<Json> = doc
            .presentations
            .iter()
            .map(|p| {
                let kind = if p.timeline.is_some() {
                    "lesson"
                } else if p.views.iter().any(|v| !v.representations.is_empty()) {
                    "interactive"
                } else {
                    "observations"
                };
                let model = doc.models.iter().find(|m| m.id == p.model).map(|m| m.name.clone()).unwrap_or_default();
                json!({ "id": p.id, "name": p.name, "model": model, "kind": kind })
            })
            .collect();
        let cases: Vec<Json> = doc.runs.iter().map(|r| json!({ "id": r.id, "name": r.name })).collect();
        let models: Vec<Json> = doc.models.iter().map(|m| json!(m.name)).collect();
        json!({ "models": models, "presentations": pres, "cases": cases })
    }

    /// Runs every case headless and reports each expectation (HI-3.5, PK-4.2).
    pub fn run_cases(&self) -> Result<Json, Diagnostic> {
        let prog = self.program.as_ref().ok_or_else(|| Diagnostic::new("HI-E03", "the document has diagnostics; its cases cannot run"))?;
        let cases: Vec<Json> = prog
            .doc
            .runs
            .iter()
            .map(|case| match run_case(prog, case) {
                Ok(report) => {
                    let results: Vec<Json> = report.results.iter().map(|r| json!({ "id": r.id.rsplit('.').next().unwrap_or(&r.id), "pass": r.pass, "message": r.message })).collect();
                    let beats: Vec<Json> = report.playback.iter().flat_map(|pb| pb.beats.iter()).map(|b| json!({ "beat": b.name, "start": b.start, "end": b.end })).collect();
                    json!({ "case": case.name, "results": results, "beats": beats })
                }
                Err(e) => json!({ "case": case.name, "error": e }),
            })
            .collect();
        Ok(json!(cases))
    }
}
