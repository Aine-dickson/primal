//! Prismal web player (D-018): the reference web host of the host interface (D-045).
//!
//! The player logic lives in `prismal-host`; this crate adds what the reference renderer
//! (`web/`) uses: [`Player`], one document with one instance, the embedded example programs,
//! and, compiled for `wasm32`, the JavaScript bindings (`wasm`): the player and the
//! engine's JSON protocol, for any web host.

pub mod examples;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use prismal_host::mathml;
use prismal_host::{Content, Diagnostic, Document, Instance};
use serde_json::{json, Value as Json};
use std::ops::{Deref, DerefMut};

/// Least simulated span of an interactive session of a dynamic model.
pub const SESSION_HORIZON: f64 = prismal_host::instance::SESSION_HORIZON;

/// A diagnostic located in the source text, as the reference renderer's editor shows it.
fn located(d: &Diagnostic) -> Json {
    let s = d.span.unwrap_or(prismal_host::Span { line: 0, col: 0, start: 0, end: 0 });
    json!({ "code": d.code, "message": d.message, "line": s.line, "col": s.col, "start": s.start, "end": s.end })
}

/// The reference player: one document and one instance of it. The instance's operations
/// are reached through `Deref`.
pub struct Player {
    doc: Document,
    inst: Instance,
}

impl Player {
    /// Compiles and checks a program: source text in the working syntax, or a Markdown
    /// document whose `text` and `cases` blocks form it. Fails with located diagnostics.
    pub fn load(src: &str) -> Result<Player, Json> {
        let doc = Document::load(Content::Text(src.to_string())).map_err(|ds| Json::Array(ds.iter().map(located).collect()))?;
        let Some(prog) = doc.program() else { return Err(Json::Array(doc.diagnostics().iter().map(located).collect())) };
        Ok(Player { inst: Instance::new(prog), doc })
    }

    /// The program in the canonical form (working syntax section 1.4).
    pub fn formatted(&self) -> String {
        self.doc.text()
    }

    /// The presentations and cases of the program.
    pub fn catalogue(&self) -> Json {
        self.doc.catalogue()
    }

    /// Runs every case of the program headless and reports each expectation (PK-4.2).
    pub fn run_cases(&self) -> Json {
        self.doc.run_cases().unwrap_or_else(|d| json!([{ "case": "-", "error": d.message }]))
    }

    /// The source location of an IR element (for linking diagnostics and representations
    /// to the editor).
    pub fn locate(&self, element: &str) -> Json {
        match self.doc.locate(element) {
            Some(s) => json!({ "line": s.line, "col": s.col, "start": s.start, "end": s.end }),
            None => Json::Null,
        }
    }
}

impl Deref for Player {
    type Target = Instance;
    fn deref(&self) -> &Instance {
        &self.inst
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Instance {
        &mut self.inst
    }
}
